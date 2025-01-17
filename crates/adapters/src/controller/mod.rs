//! An I/O controller that coordinates the creation, reconfiguration,
//! teardown of input/output adapters, and implements runtime flow control.
//!
//! # Design
//!
//! The controller logic is split into two tasks that run in separate threads.
//! The circuit thread owns the `DBSPHandle` and calls `step()` on it whenever
//! there is some input data available for the circuit.  It can be configured
//! to improve batching by slightly delaying the `step()` call if the number of
//! available input records is below some used-defined threshold.
//!
//! The backpressure thread controls the flow of data through transport
//! endpoints, pausing the endpoints either when the amount of data buffered by
//! the endpoint exceeds a user-defined threshold or in response to an explicit
//! user request.
//!
//! Both tasks require monitoring the state of the input buffers.  To this end,
//! the controller injects `InputProbe`s between each input endpoint and format
//! parser:
//!
//! ```text
//!                ┌────────────┐
//!                │ controller │
//!                └────────────┘
//!                      ▲
//!                      │stats
//!   ┌────────┐   ┌─────┴────┐   ┌──────┐   ┌──────┐
//!   │endpoint├──►│InputProbe├──►│parser├──►│handle│
//!   └────────┘   └──────────┘   └──────┘   └──────┘
//! ```
//!
//! The probe passes the data through to the parser, while counting the number
//! of transmitted bytes and records and updating respective performance
//! counters in the controller.

use crate::transport::InputReader;
use crate::transport::Step;
use crate::transport::{
    input_transport_config_to_endpoint, output_transport_config_to_endpoint, AtomicStep,
};
use crate::{
    catalog::SerBatch, Catalog, CircuitCatalog, Encoder, InputConsumer, InputFormat,
    OutputConsumer, OutputEndpoint, OutputFormat, OutputQueryHandles, ParseError, Parser,
    PipelineState, TransportInputEndpoint,
};
use crate::{create_integrated_output_endpoint, DbspCircuitHandle};
use anyhow::Error as AnyError;
use crossbeam::{
    queue::SegQueue,
    sync::{Parker, ShardedLock, Unparker},
};
use dbsp::circuit::{CircuitConfig, Layout};
use dbsp::profile::GraphProfile;
use metrics::set_global_recorder;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use metrics_util::{
    debugging::{DebuggingRecorder, Snapshotter},
    layers::FanoutBuilder,
};
use pipeline_types::query::OutputQuery;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::{Receiver, Sender};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::OnceLock,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Condvar, Mutex,
    },
    thread::{spawn, JoinHandle},
    time::{Duration, Instant},
};
use tracing::{debug, error, info, trace};
use uuid::Uuid;

mod error;
mod stats;

use crate::catalog::{SerBatchReader, SerTrace};
use crate::integrated::create_integrated_input_endpoint;
pub use error::{ConfigError, ControllerError};
use pipeline_types::config::OutputBufferConfig;
pub use pipeline_types::config::{
    ConnectorConfig, FormatConfig, InputEndpointConfig, OutputEndpointConfig, PipelineConfig,
    RuntimeConfig, TransportConfig,
};
use pipeline_types::program_schema::canonical_identifier;
pub use stats::{ControllerStatus, InputEndpointStatus, OutputEndpointStatus};

/// Maximal number of concurrent API connections per circuit
/// (including both input and output connections).
// TODO: make this configurable.
pub(crate) const MAX_API_CONNECTIONS: u64 = 100;

pub(crate) type EndpointId = u64;

/// Controller that coordinates the creation, reconfiguration, teardown of
/// input/output adapters, and implements runtime flow control.
///
/// The controller instantiates the input and output pipelines according to a
/// user-provided [configuration](`PipelineConfig`) and exposes an API to
/// reconfigure and monitor the pipelines at runtime.
pub struct Controller {
    inner: Arc<ControllerInner>,

    /// The circuit thread handle (see module-level docs).
    circuit_thread_handle: JoinHandle<Result<(), ControllerError>>,

    /// The backpressure thread handle (see module-level docs).
    backpressure_thread_handle: JoinHandle<()>,
}

/// Type of the callback argumen to [`Controller::start_graph_profile`].
pub type GraphProfileCallbackFn = Box<dyn FnOnce(Result<GraphProfile, ControllerError>) + Send>;

impl Controller {
    /// Create a new I/O controller for a circuit.
    ///
    /// Creates a new instance of `Controller` that wraps `circuit`,  with
    /// input and output endpoints specified by `config`.  The controller is
    /// created with all endpoints in a paused state.  Call [`Self::start`]
    /// to unpause the endpoints and start ingesting data.
    ///
    /// # Arguments
    ///
    /// * `circuit` - A handle to a DBSP circuit managed by this controller. The
    ///   controller takes ownership of the circuit.
    ///
    /// * `catalog` - A catalog of input and output streams of the circuit.
    ///
    /// * `config` - Controller configuration, including global config settings
    ///   and individual endpoint configs.
    ///
    /// * `error_cb` - Error callback.  The controller doesn't implement its own
    ///   error handling policy, but simply forwards most errors to this
    ///   callback.
    ///
    /// # Errors
    ///
    /// The method may fail for the following reasons:
    ///
    /// * The input configuration is invalid, e.g., specifies an unknown
    ///   transport or data format.
    ///
    /// * One or more of the endpoints fails to initialize.
    pub fn with_config<F>(
        circuit_factory: F,
        config: &PipelineConfig,
        error_cb: Box<dyn Fn(ControllerError) + Send + Sync>,
    ) -> Result<Self, ControllerError>
    where
        F: FnOnce(
                CircuitConfig,
            )
                -> Result<(Box<dyn DbspCircuitHandle>, Box<dyn CircuitCatalog>), ControllerError>
            + Send
            + 'static,
    {
        let circuit_thread_parker = Parker::new();
        let circuit_thread_unparker = circuit_thread_parker.unparker().clone();

        let backpressure_thread_parker = Parker::new();
        let backpressure_thread_unparker = backpressure_thread_parker.unparker().clone();

        let (profile_request_sender, profile_request_receiver) = channel();
        let inner = Arc::new(ControllerInner::new(
            config,
            circuit_thread_unparker,
            backpressure_thread_unparker,
            error_cb,
            profile_request_sender,
        ));

        let backpressure_thread_handle = {
            let inner = inner.clone();
            spawn(move || Self::backpressure_thread(inner, backpressure_thread_parker))
        };

        let circuit_thread_handle = {
            let inner = inner.clone();

            // A channel to communicate circuit initialization status.
            // The `circuit_factory` closure must be invoked in the context of
            // the circuit thread, because the circuit handle it returns doesn't
            // implement `Send`.  So we need this channel to communicate circuit
            // initialization status back to this thread.
            let (init_status_sender, init_status_receiver) =
                sync_channel::<Result<(), ControllerError>>(0);
            let handle = spawn(move || {
                Self::circuit_thread(
                    circuit_factory,
                    inner,
                    circuit_thread_parker,
                    init_status_sender,
                    profile_request_receiver,
                )
            });
            // If `recv` fails, it indicates that the circuit thread panicked
            // during initialization.
            init_status_receiver
                .recv()
                .map_err(|_| ControllerError::dbsp_panic())??;
            handle
        };

        for (input_name, input_config) in config.inputs.iter() {
            inner.connect_input(input_name, input_config)?;
        }

        for (output_name, output_config) in config.outputs.iter() {
            inner.connect_output(output_name, output_config)?;
        }

        Ok(Self {
            inner,
            circuit_thread_handle,
            backpressure_thread_handle,
        })
    }

    /// Connect a new input endpoint with specified name and configuration.
    ///
    /// Creates an endpoint with data transport and format specified by
    /// `config` and starts streaming data from the endpoint if the pipeline
    /// is running.
    ///
    /// # Errors
    ///
    /// The method may fail for the following reasons:
    ///
    /// * The endpoint configuration is invalid, e.g., specifies an unknown
    ///   transport or data format.
    ///
    /// * The endpoint fails to initialize, e.g., because the network address or
    ///   filename specified in the transport config is unreachable.
    pub fn connect_input(
        &self,
        endpoint_name: &str,
        config: &InputEndpointConfig,
    ) -> Result<EndpointId, ControllerError> {
        debug!("Connecting input endpoint '{endpoint_name}'; config: {config:?}");
        self.inner.connect_input(endpoint_name, config)
    }

    /// Disconnect an existing input endpoint.
    ///
    /// This method is asynchronous and may return before all endpoint
    /// threads have terminated.
    pub fn disconnect_input(&self, endpoint_id: &EndpointId) {
        debug!("Disconnecting input endpoint {endpoint_id}");

        self.inner.disconnect_input(endpoint_id)
    }

    /// Connect a previously instantiated input endpoint.
    ///
    /// Used to connect an endpoint instantiated manually rather than from an
    /// [`InputEndpointConfig`].
    ///
    /// # Arguments
    ///
    /// * `endpoint_name` - endpoint name unique within the pipeline.
    ///
    /// * `endpoint_config` - endpoint config.
    ///
    /// * `endpoint` - transport endpoint object.
    pub fn add_input_endpoint(
        &self,
        endpoint_name: &str,
        endpoint_config: InputEndpointConfig,
        endpoint: Box<dyn TransportInputEndpoint>,
    ) -> Result<EndpointId, ControllerError> {
        debug!("Adding input endpoint '{endpoint_name}'; config: {endpoint_config:?}");

        self.inner
            .add_input_endpoint(endpoint_name, endpoint_config, Some(endpoint))
    }

    /// Disconnect an existing output endpoint.
    ///
    /// This method is asynchronous and may return before all endpoint
    /// threads have terminated.
    pub fn disconnect_output(&self, endpoint_id: &EndpointId) {
        debug!("Disconnecting output endpoint {endpoint_id}");

        self.inner.disconnect_output(endpoint_id)
    }

    /// Connect a previously instantiated output endpoint.
    ///
    /// Used to connect an endpoint instantiated manually rather than from an
    /// [`OutputEndpointConfig`].
    ///
    /// # Arguments
    ///
    /// * `endpoint_name` - endpoint name unique within the pipeline.
    ///
    /// * `endpoint_config` - (partial) endpoint config.  Only `format.name` and
    ///   `stream` fields need to be initialized.
    ///
    /// * `endpoint` - transport endpoint object.
    pub fn add_output_endpoint(
        &self,
        endpoint_name: &str,
        endpoint_config: &OutputEndpointConfig,
        endpoint: Box<dyn OutputEndpoint>,
    ) -> Result<EndpointId, ControllerError> {
        debug!("Adding output endpoint '{endpoint_name}'; config: {endpoint_config:?}");

        self.inner
            .add_output_endpoint(endpoint_name, endpoint_config, Some(endpoint))
    }

    /// Reports whether the circuit is fault tolerant.  A circuit is fault
    /// tolerant if it computes a deterministic function and all of its inputs
    /// and outputs are fault tolerant.  This function assumes that the circuit
    /// is deterministic.
    pub fn is_fault_tolerant(&self) -> bool {
        self.inner
            .status
            .input_status()
            .values()
            .all(|status| status.is_fault_tolerant)
            && self
                .inner
                .outputs
                .read()
                .unwrap()
                .by_id
                .values()
                .all(|descr| descr.is_fault_tolerant)
    }

    /// Increment the number of active API connections.
    ///
    /// API connections are created dynamically via the `ingress` and `egress`
    /// REST API endpoints.
    ///
    /// Fails if the number of connections exceeds the current limit,
    /// returning the number of existing API connections.
    pub fn register_api_connection(&self) -> Result<(), u64> {
        self.inner.register_api_connection()
    }

    /// Decrement the number of active API connections.
    pub fn unregister_api_connection(&self) {
        self.inner.unregister_api_connection();
    }

    /// Return the number of active API connections.
    pub fn num_api_connections(&self) -> u64 {
        self.inner.num_api_connections()
    }

    /// Force the circuit to perform a step even if all of its
    /// input buffers are empty or nearly empty.
    ///
    /// Can be used in two situations:
    ///
    /// * To process scalar inputs, such as neighborhood or quantile queries
    ///   that are not reflected in input buffer sizes.
    ///
    /// * When input buffers are not completely empty, but contain fewer than
    ///   `min_batch_size_records` records the circuit will normally wait for a
    ///   small timeout before processing the data.  This function can be used
    ///   to force immediate processing.
    pub fn request_step(&self) {
        self.inner.request_step();
    }

    /// Change the state of all input endpoints to running.
    ///
    /// Start streaming data through all connected input endpoints.
    pub fn start(&self) {
        debug!("Starting the pipeline");

        self.inner.start();
    }

    /// Pause all input endpoints.
    ///
    /// Sends a pause command to all input endpoints.  Upon receiving the
    /// command, the endpoints must stop pushing data to the pipeline.  This
    /// method is asynchronous and may return before all endpoints have been
    /// fully paused.
    pub fn pause(&self) {
        debug!("Pausing the pipeline");
        self.inner.pause();
    }

    /// Returns controller status.
    pub fn status(&self) -> &ControllerStatus {
        // Update pipeline metrics computed on-demand.
        self.inner
            .status
            .update(self.inner.metrics_snapshotter.snapshot());
        &self.inner.status
    }

    pub fn catalog(&self) -> &Arc<Mutex<Box<dyn CircuitCatalog>>> {
        &self.inner.catalog
    }

    /// Triggers a dump of the circuit's performance profile to the file system.
    /// The profile will be written asynchronously, probably after this function
    /// returns.
    pub fn dump_profile(&self) {
        debug!("Generating DBSP profile dump");
        self.start_graph_profile(Box::new(|profile| {
            match profile.map(|profile| {
                profile
                    .dump("profile")
                    .map_err(|e| ControllerError::io_error(String::from("dumping profile"), e))
            }) {
                Ok(Ok(path)) => info!("Dumped DBSP profile to {}", path.display()),
                Ok(Err(e)) | Err(e) => error!("Failed to write circuit profile: {e}"),
            }
        }));
    }

    /// Triggers a profiling operation in the running pipeline. `cb` will be
    /// called with the profile when it is ready, probably after this function
    /// returns.
    ///
    /// The callback-based nature of this function makes it useful in
    /// asynchronous contexts.
    pub fn start_graph_profile(&self, cb: GraphProfileCallbackFn) {
        self.inner.graph_profile(cb)
    }

    /// Terminate the controller, stop all input endpoints and destroy the
    /// circuit.
    pub fn stop(self) -> Result<(), ControllerError> {
        debug!("Stopping the circuit");

        self.inner.stop();
        self.circuit_thread_handle
            .join()
            .map_err(|_| ControllerError::controller_panic())??;
        self.backpressure_thread_handle
            .join()
            .map_err(|_| ControllerError::controller_panic())?;
        Ok(())
    }

    /// Check whether the pipeline has processed all input data to completion.
    ///
    /// Returns `true` when the following conditions are satisfied:
    ///
    /// * All input endpoints have signalled end-of-input.
    /// * All input records received from all endpoints have been processed by
    ///   the circuit.
    /// * All output records have been sent to respective output transport
    ///   endpoints.
    ///
    /// Note that, depending on the type and configuration of the output
    /// transport, this may not guarantee that all output records have been
    /// written to a persistent storage or delivered to the recipient.
    pub fn pipeline_complete(&self) -> bool {
        self.inner.status.pipeline_complete()
    }

    /// Circuit thread function: holds the handle to the circuit, calls `step`
    /// on it whenever input data is available, pushes output batches
    /// produced by the circuit to output pipelines.
    fn circuit_thread<F>(
        circuit_factory: F,
        controller: Arc<ControllerInner>,
        parker: Parker,
        init_status_sender: SyncSender<Result<(), ControllerError>>,
        profile_request_receiver: Receiver<GraphProfileCallbackFn>,
    ) -> Result<(), ControllerError>
    where
        F: FnOnce(
            CircuitConfig,
        )
            -> Result<(Box<dyn DbspCircuitHandle>, Box<dyn CircuitCatalog>), ControllerError>,
    {
        let mut start: Option<Instant> = None;
        let min_storage_bytes = if controller.status.pipeline_config.global.storage {
            // This reduces the files stored on disk to a reasonable number.
            controller
                .status
                .pipeline_config
                .global
                .min_storage_bytes
                .unwrap_or(1024 * 1024)
        } else {
            usize::MAX
        };
        let config = CircuitConfig {
            layout: Layout::new_solo(controller.status.pipeline_config.global.workers as usize),
            storage: controller.status.pipeline_config.storage_config.clone(),
            min_storage_bytes,
            init_checkpoint: Uuid::nil(),
        };
        let mut circuit = match circuit_factory(config) {
            Ok((circuit, catalog)) => {
                // Complete initialization before sending back the confirmation to
                // prevent a race.
                *controller.catalog.lock().unwrap() = catalog;
                let _ = init_status_sender.send(Ok(()));
                circuit
            }
            Err(e) => {
                let _ = init_status_sender.send(Err(e));
                return Ok(());
            }
        };

        if controller.status.pipeline_config.global.cpu_profiler {
            circuit.enable_cpu_profiler().unwrap_or_else(|e| {
                error!("Failed to enable CPU profiler: {e}");
            });
        }

        let max_buffering_delay = Duration::from_micros(
            controller
                .status
                .pipeline_config
                .global
                .max_buffering_delay_usecs,
        );
        let min_batch_size_records = controller
            .status
            .pipeline_config
            .global
            .min_batch_size_records;

        let mut step = 0;

        loop {
            for reply_cb in profile_request_receiver.try_iter() {
                reply_cb(circuit.graph_profile());
            }
            match controller.state() {
                PipelineState::Running | PipelineState::Paused => {
                    // Backpressure in the output pipeline: wait for room in output buffers to
                    // become available.
                    if controller.output_buffers_full() {
                        debug!("circuit thread: park waiting for output buffer space");
                        parker.park();
                        debug!("circuit thread: unparked");
                        continue;
                    }

                    let buffered_records = controller.status.num_buffered_input_records();

                    // We have sufficient buffered inputs or the buffering delay has expired or
                    // the client explicitly requested the circuit to run -- kick the circuit to
                    // consume buffered data.
                    // Use strict inequality in case `min_batch_size_records` is 0.
                    if controller.status.step_requested()
                        || buffered_records > min_batch_size_records
                        || start
                            .map(|start| start.elapsed() >= max_buffering_delay)
                            .unwrap_or(false)
                    {
                        // Request all of the inputs to complete this step, and
                        // then wait for the completions.
                        for endpoint in controller.inputs.lock().unwrap().values() {
                            endpoint.reader.complete(step);
                        }
                        while !controller.status.is_step_complete(step) {
                            parker.park();
                        }

                        start = None;
                        // Reset all counters of buffered records and bytes to 0.
                        controller.status.consume_buffered_inputs();

                        // All input records accumulated so far (and possibly some more) will
                        // be fully processed after the `step()` call returns.
                        let processed_records = controller.status.num_total_input_records();

                        // Wake up the backpressure thread to unpause endpoints blocked due to
                        // backpressure.
                        controller.unpark_backpressure();
                        debug!("circuit thread: calling 'circuit.step'");
                        circuit.step().unwrap_or_else(|e| controller.error(e));
                        debug!("circuit thread: 'circuit.step' returned");

                        controller
                            .status
                            .set_num_total_processed_records(processed_records);

                        // Push output batches to output pipelines.
                        let outputs = controller.outputs.read().unwrap();
                        for ((_stream, _query), (output_handles, endpoints)) in
                            outputs.iter_by_stream()
                        {
                            let mut delta_batch = output_handles
                                .delta
                                .as_ref()
                                .map(|handle| handle.take_from_all());
                            let num_delta_records = delta_batch
                                .as_ref()
                                .map(|batch| batch.iter().map(|b| b.len()).sum());

                            let mut snapshot_batch = output_handles
                                .snapshot
                                .as_ref()
                                .map(|handle| handle.take_from_all());
                            let num_snapshot_records = snapshot_batch
                                .as_ref()
                                .map(|batch| batch.iter().map(|b| b.len()).sum());

                            for (i, endpoint_id) in endpoints.iter().enumerate() {
                                let endpoint = outputs.lookup_by_id(endpoint_id).unwrap();
                                // If the endpoint has a snapshot stream associated with it,
                                // then the first output sent to this endpoint must be
                                // the snapshot.  Subsequent outputs are deltas on top of
                                // the snapshot.
                                if !endpoint.snapshot_sent.load(Ordering::Acquire)
                                    && output_handles.snapshot.is_some()
                                {
                                    if snapshot_batch
                                        .as_ref()
                                        .map(|batches| !batches.is_empty())
                                        .unwrap_or(false)
                                    {
                                        // Increment stats first, so we don't end up with negative
                                        // counts.
                                        controller.status.enqueue_batch(
                                            *endpoint_id,
                                            num_snapshot_records.unwrap(),
                                        );

                                        // Send the batch without cloning to the last consumer.
                                        let batch = if i == endpoints.len() - 1 {
                                            snapshot_batch.take().unwrap()
                                        } else {
                                            snapshot_batch.as_ref().unwrap().clone()
                                        };

                                        // Associate the input frontier with the batch.  Once the
                                        // batch has
                                        // been sent to the output endpoint, the endpoint will get
                                        // labeled with this
                                        // frontier.
                                        endpoint.queue.push((step, batch, processed_records));
                                        endpoint.snapshot_sent.store(true, Ordering::Release);
                                    }
                                } else if delta_batch.is_some() {
                                    controller
                                        .status
                                        .enqueue_batch(*endpoint_id, num_delta_records.unwrap());

                                    let batch = if i == endpoints.len() - 1 {
                                        delta_batch.take().unwrap()
                                    } else {
                                        delta_batch.as_ref().unwrap().clone()
                                    };

                                    endpoint.queue.push((step, batch, processed_records));
                                }

                                // Wake up the output thread.  We're not trying to be smart here and
                                // wake up the thread conditionally if it was previously idle, as I
                                // don't expect this to make any real difference.
                                endpoint.unparker.unpark();
                            }
                        }

                        step += 1;
                        controller.step.store(step, Ordering::Release);
                        controller.unpark_backpressure();
                    } else if buffered_records > 0 {
                        // We have some buffered data, but less than `min_batch_size_records` --
                        // wait up to `max_buffering_delay` for more data to
                        // arrive.
                        if start.is_none() {
                            start = Some(Instant::now());
                        }
                        parker.park_timeout(Duration::from_millis(1));
                    } else {
                        debug!("circuit thread: park: input buffers empty");
                        parker.park();
                        debug!("circuit thread: unparked");
                    }
                }
                PipelineState::Terminated => {
                    circuit.kill().map_err(|_| ControllerError::dbsp_panic())?;
                    for reply_cb in profile_request_receiver.try_iter() {
                        reply_cb(Err(ControllerError::ControllerExit));
                    }
                    return Ok(());
                }
            }
        }
    }

    /// Backpressure thread function.
    fn backpressure_thread(controller: Arc<ControllerInner>, parker: Parker) {
        #[derive(Copy, Clone, PartialEq, Eq)]
        enum EndpointState {
            Pause,
            Run(Step),
        }

        let mut endpoint_states = HashMap::new();

        loop {
            let global_pause = match controller.state() {
                PipelineState::Paused => true,
                PipelineState::Running => false,
                PipelineState::Terminated => return,
            };
            let step = controller.step.load(Ordering::Acquire);
            trace!("stepping to {step}");

            for (epid, ep) in controller.inputs.lock().unwrap().iter() {
                let current_state = endpoint_states.entry(*epid).or_insert(EndpointState::Pause);
                let desired_state = if global_pause || controller.status.input_endpoint_full(epid) {
                    EndpointState::Pause
                } else {
                    EndpointState::Run(step)
                };
                if *current_state != desired_state {
                    let result = match desired_state {
                        EndpointState::Pause => ep.reader.pause(),
                        EndpointState::Run(step) => ep.reader.start(step),
                    };
                    if let Err(error) = result {
                        controller.input_transport_error(*epid, &ep.endpoint_name, true, error);
                    };
                    *current_state = desired_state;
                }
            }

            parker.park();
        }
    }

    pub(crate) fn metrics(&self) -> PrometheusHandle {
        self.inner.prometheus_handle.clone()
    }
}

/// State tracked by the controller for each input endpoint.
struct InputEndpointDescr {
    endpoint_name: String,
    reader: Box<dyn InputReader>,
}

impl InputEndpointDescr {
    pub fn new(endpoint_name: &str, reader: Box<dyn InputReader>) -> Self {
        Self {
            endpoint_name: endpoint_name.to_owned(),
            reader,
        }
    }
}

/// A lock-free queue used to send output batches from the circuit thread
/// to output endpoint threads.  Each entry is annotated with a progress label
/// that is equal to the number of input records fully processed by
/// DBSP before emitting this batch of outputs.  The label increases
/// monotonically over time.
type BatchQueue = SegQueue<(Step, Vec<Arc<dyn SerBatch>>, u64)>;

/// State tracked by the controller for each output endpoint.
struct OutputEndpointDescr {
    /// Endpoint name.
    endpoint_name: String,

    /// Stream name that the endpoint is connected to.
    stream_name: String,

    /// Query associated with the endpoint.
    query: OutputQuery,

    /// FIFO queue of batches read from the stream.
    queue: Arc<BatchQueue>,

    /// True if the endpoint has already received a complete snapshot
    /// of the query result.
    snapshot_sent: AtomicBool,

    /// Used to notify the endpoint thread that the endpoint is being
    /// disconnected.
    disconnect_flag: Arc<AtomicBool>,

    /// Unparker for the endpoint thread.
    unparker: Unparker,

    /// Whether the output endpoint can discard duplicate output.
    is_fault_tolerant: bool,
}

impl OutputEndpointDescr {
    pub fn new(
        endpoint_name: &str,
        stream_name: &str,
        query: OutputQuery,
        unparker: Unparker,
        is_fault_tolerant: bool,
    ) -> Self {
        Self {
            endpoint_name: endpoint_name.to_string(),
            stream_name: canonical_identifier(stream_name),
            query,
            queue: Arc::new(SegQueue::new()),
            snapshot_sent: AtomicBool::new(false),
            disconnect_flag: Arc::new(AtomicBool::new(false)),
            unparker,
            is_fault_tolerant,
        }
    }
}

type StreamEndpointMap =
    BTreeMap<(String, OutputQuery), (OutputQueryHandles, BTreeSet<EndpointId>)>;

struct OutputEndpoints {
    by_id: BTreeMap<EndpointId, OutputEndpointDescr>,
    by_stream: StreamEndpointMap,
}

impl OutputEndpoints {
    fn new() -> Self {
        Self {
            by_id: BTreeMap::new(),
            by_stream: BTreeMap::new(),
        }
    }

    fn iter_by_stream(
        &self,
    ) -> impl Iterator<
        Item = (
            &'_ (String, OutputQuery),
            &'_ (OutputQueryHandles, BTreeSet<EndpointId>),
        ),
    > {
        self.by_stream.iter()
    }

    fn lookup_by_id(&self, endpoint_id: &EndpointId) -> Option<&OutputEndpointDescr> {
        self.by_id.get(endpoint_id)
    }

    fn lookup_by_name(&self, endpoint_name: &str) -> Option<&OutputEndpointDescr> {
        self.by_id
            .values()
            .find(|ep| ep.endpoint_name == endpoint_name)
    }

    fn alloc_endpoint_id(&self) -> EndpointId {
        self.by_id.keys().next_back().map(|k| k + 1).unwrap_or(0)
    }

    fn insert(
        &mut self,
        endpoint_id: EndpointId,
        handles: OutputQueryHandles,
        endpoint_descr: OutputEndpointDescr,
    ) {
        self.by_stream
            .entry((endpoint_descr.stream_name.clone(), endpoint_descr.query))
            .or_insert_with(|| (handles, BTreeSet::new()))
            .1
            .insert(endpoint_id);
        self.by_id.insert(endpoint_id, endpoint_descr);
    }

    fn remove(&mut self, endpoint_id: &EndpointId) -> Option<OutputEndpointDescr> {
        self.by_id.remove(endpoint_id).map(|descr| {
            self.by_stream
                .get_mut(&(descr.stream_name.clone(), descr.query))
                .map(|(_, endpoints)| endpoints.remove(endpoint_id));
            descr
        })
    }
}

/// Buffer used by the output endpoint thread to accumulate outputs.
struct OutputBuffer {
    #[allow(unused)]
    endpoint_name: String,

    buffer: Option<Box<dyn SerTrace>>,

    /// Step number of the last update in the buffer.
    ///
    /// The endpoint will wait for this step to commit before sending the buffer
    /// out.
    buffered_step: Step,

    /// Time when the first batch was pushed to the buffer.
    buffer_since: Instant,

    /// Number of input records that will be fully processed after the buffer is flushed.
    ///
    /// This is a part of the progress tracking mechanism, which tracks the number of inputs
    /// to the pipeline that have been processed to completion.  It is currently used
    /// to determine when the circuit has run to completion.
    buffered_processed_records: u64,
}

impl OutputBuffer {
    /// Create an empty buffer.
    fn new(endpoint_name: &str) -> Self {
        Self {
            endpoint_name: endpoint_name.to_string(),
            buffer: None,
            buffered_step: 0,
            buffer_since: Instant::now(),
            buffered_processed_records: 0,
        }
    }

    /// Insert `batch` into the buffer.
    fn insert(&mut self, batch: Arc<dyn SerBatch>, step: Step, processed_records: u64) {
        if let Some(buffer) = &mut self.buffer {
            buffer.insert(batch);
        } else {
            self.buffer = Some(batch.into_trace());
            self.buffer_since = Instant::now();
        }
        self.buffered_step = step;
        self.buffered_processed_records = processed_records;
    }

    /// Returns `true` when it is time to flush the buffer either because it's full or
    /// because the max buffering timeout has expired.
    fn flush_needed(&self, config: &OutputBufferConfig) -> bool {
        if let Some(buffer) = &self.buffer {
            let buffer = buffer.as_ref();
            if buffer.len() >= config.max_output_buffer_size_records {
                return true;
            }

            if self.buffer_since.elapsed().as_millis()
                > config.max_output_buffer_time_millis as u128
            {
                return true;
            }
        }

        false
    }

    /// Time when the oldest data was inserted in the buffer.
    fn buffer_since(&self) -> Option<Instant> {
        if self.buffer.is_some() {
            Some(self.buffer_since)
        } else {
            None
        }
    }

    /// Return the contents of the buffer leaving it empty.
    fn take_buffer(&mut self) -> Option<Box<dyn SerTrace>> {
        self.buffer.take()
    }
}

/// Controller state sharable across threads.
///
/// A reference to this struct is held by each input probe and by both
/// controller threads.
pub struct ControllerInner {
    pub status: Arc<ControllerStatus>,
    num_api_connections: AtomicU64,
    profile_request: Sender<GraphProfileCallbackFn>,
    catalog: Arc<Mutex<Box<dyn CircuitCatalog>>>,
    inputs: Mutex<BTreeMap<EndpointId, InputEndpointDescr>>,
    outputs: ShardedLock<OutputEndpoints>,
    circuit_thread_unparker: Unparker,
    backpressure_thread_unparker: Unparker,
    error_cb: Box<dyn Fn(ControllerError) + Send + Sync>,
    step: AtomicStep,
    metrics_snapshotter: Arc<Snapshotter>,
    prometheus_handle: PrometheusHandle,

    /// The lowest-numbered input step not known to have committed yet.
    ///
    /// This is updated lazily, only when we need to wait for a step to commit
    /// in `wait_to_commit`.
    uncommitted_step: Mutex<Step>,

    /// Condition that fires when `uncommitted_step` increases.
    step_committed: Condvar,
}

impl ControllerInner {
    fn new(
        config: &PipelineConfig,
        circuit_thread_unparker: Unparker,
        backpressure_thread_unparker: Unparker,
        error_cb: Box<dyn Fn(ControllerError) + Send + Sync>,
        profile_request: Sender<GraphProfileCallbackFn>,
    ) -> Self {
        let pipeline_name = config
            .name
            .as_ref()
            .map_or_else(|| "unnamed".to_string(), |n| n.clone());
        let status = Arc::new(ControllerStatus::new(config));
        let (metrics_snapshotter, prometheus_handle) =
            Self::install_metrics_recorder(pipeline_name);

        Self {
            status,
            num_api_connections: AtomicU64::new(0),
            profile_request,
            catalog: Arc::new(Mutex::new(Box::new(Catalog::new()))),
            inputs: Mutex::new(BTreeMap::new()),
            outputs: ShardedLock::new(OutputEndpoints::new()),
            circuit_thread_unparker,
            backpressure_thread_unparker,
            error_cb,
            step: AtomicStep::new(0),
            metrics_snapshotter,
            prometheus_handle,
            uncommitted_step: Mutex::new(0),
            step_committed: Condvar::new(),
        }
    }

    /// Sets the global metrics recorder and returns a `Snapshotter` and
    /// a `PrometheusHandle` to get metrics in a prometheus compatible format.
    fn install_metrics_recorder(pipeline_name: String) -> (Arc<Snapshotter>, PrometheusHandle) {
        static METRIC_HANDLES: OnceLock<(Arc<Snapshotter>, PrometheusHandle)> = OnceLock::new();
        METRIC_HANDLES
            .get_or_init(|| {
                let debugging_recorder = DebuggingRecorder::new();
                let snapshotter = debugging_recorder.snapshotter();
                let prometheus_recorder = PrometheusBuilder::new()
                    .add_global_label("pipeline", pipeline_name)
                    .build_recorder();
                let prometheus_handle = prometheus_recorder.handle();
                let builder = FanoutBuilder::default()
                    .add_recorder(debugging_recorder)
                    .add_recorder(prometheus_recorder);

                set_global_recorder(builder.build()).expect("failed to install metrics exporter");

                (Arc::new(snapshotter), prometheus_handle)
            })
            .clone()
    }
    fn connect_input(
        self: &Arc<Self>,
        endpoint_name: &str,
        endpoint_config: &InputEndpointConfig,
    ) -> Result<EndpointId, ControllerError> {
        let endpoint =
            input_transport_config_to_endpoint(endpoint_config.connector_config.transport.clone())
                .map_err(|e| ControllerError::input_transport_error(endpoint_name, true, e))?;

        // If `endpoint` is `None`, it means that the endpoint config specifies an integrated
        // input connector.  Such endpoints are instantiated inside `add_input_endpoint`.
        self.add_input_endpoint(endpoint_name, endpoint_config.clone(), endpoint)
    }

    fn disconnect_input(self: &Arc<Self>, endpoint_id: &EndpointId) {
        let mut inputs = self.inputs.lock().unwrap();

        if let Some(ep) = inputs.remove(endpoint_id) {
            ep.reader.disconnect();
            self.status.remove_input(endpoint_id);
            self.unpark_circuit();
            self.unpark_backpressure();
        }
    }

    fn add_input_endpoint(
        self: &Arc<Self>,
        endpoint_name: &str,
        endpoint_config: InputEndpointConfig,
        endpoint: Option<Box<dyn TransportInputEndpoint>>,
    ) -> Result<EndpointId, ControllerError> {
        let mut inputs = self.inputs.lock().unwrap();

        if inputs.values().any(|ep| ep.endpoint_name == endpoint_name) {
            Err(ControllerError::duplicate_input_endpoint(endpoint_name))?;
        }

        // Create input pipeline, consisting of a transport endpoint, controller
        // probe, and parser.
        //
        // ┌────────┐   ┌──────────┐   ┌──────┐
        // │endpoint├──►│InputProbe├──►│parser├──►
        // └────────┘   └──────────┘   └──────┘

        let catalog = self.catalog.lock().unwrap();
        let input_handle = catalog
            .input_collection_handle(&endpoint_config.stream)
            .ok_or_else(|| {
                ControllerError::unknown_input_stream(endpoint_name, &endpoint_config.stream)
            })?;

        // Create probe.
        let endpoint_id = inputs.keys().next_back().map(|k| k + 1).unwrap_or(0);

        let reader = match endpoint {
            Some(endpoint) => {
                // Create parser.
                let format_config = endpoint_config
                    .connector_config
                    .format
                    .as_ref()
                    .ok_or_else(|| ControllerError::input_format_not_specified(endpoint_name))?
                    .clone();
                let format =
                    <dyn InputFormat>::get_format(&format_config.name).ok_or_else(|| {
                        ControllerError::unknown_input_format(endpoint_name, &format_config.name)
                    })?;

                let parser =
                    format.new_parser(endpoint_name, input_handle, &format_config.config)?;
                let probe = Box::new(InputProbe::new(
                    endpoint_id,
                    endpoint_name,
                    parser,
                    self.clone(),
                ));

                // Initialize endpoint stats.
                self.status.add_input(
                    &endpoint_id,
                    endpoint_name,
                    endpoint_config,
                    endpoint.is_fault_tolerant(),
                );

                endpoint
                    .open(probe, 0)
                    .map_err(|e| ControllerError::input_transport_error(endpoint_name, true, e))?
            }
            None => {
                let endpoint = create_integrated_input_endpoint(
                    endpoint_id,
                    endpoint_name,
                    &endpoint_config,
                    Arc::downgrade(self),
                )?;

                // Initialize endpoint stats.
                self.status.add_input(
                    &endpoint_id,
                    endpoint_name,
                    endpoint_config,
                    endpoint.is_fault_tolerant(),
                );

                endpoint
                    .open(input_handle, 0)
                    .map_err(|e| ControllerError::input_transport_error(endpoint_name, true, e))?
            }
        };

        if self.state() == PipelineState::Running {
            reader
                .start(0)
                .map_err(|e| ControllerError::input_transport_error(endpoint_name, true, e))?;
        }

        inputs.insert(endpoint_id, InputEndpointDescr::new(endpoint_name, reader));

        drop(inputs);

        self.unpark_backpressure();
        Ok(endpoint_id)
    }

    fn register_api_connection(&self) -> Result<(), u64> {
        let num_connections = self.num_api_connections.load(Ordering::Acquire);

        if num_connections >= MAX_API_CONNECTIONS {
            Err(num_connections)
        } else {
            self.num_api_connections.fetch_add(1, Ordering::AcqRel);
            Ok(())
        }
    }

    fn unregister_api_connection(&self) {
        let old = self.num_api_connections.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(old > 0);
    }

    fn num_api_connections(&self) -> u64 {
        self.num_api_connections.load(Ordering::Acquire)
    }

    pub fn request_step(&self) {
        self.status.request_step(&self.circuit_thread_unparker);
    }

    /// Unpark the circuit thread.
    fn unpark_circuit(&self) {
        self.circuit_thread_unparker.unpark();
    }

    /// Unpark the backpressure thread.
    fn unpark_backpressure(&self) {
        self.backpressure_thread_unparker.unpark();
    }

    fn connect_output(
        self: &Arc<Self>,
        endpoint_name: &str,
        endpoint_config: &OutputEndpointConfig,
    ) -> Result<EndpointId, ControllerError> {
        let endpoint =
            output_transport_config_to_endpoint(endpoint_config.connector_config.transport.clone())
                .map_err(|e| ControllerError::output_transport_error(endpoint_name, true, e))?;

        // If `endpoint` is `None`, it means that the endpoint config specifies an integrated
        // output connector.  Such endpoints are instantiated inside `add_output_endpoint`.
        self.add_output_endpoint(endpoint_name, endpoint_config, endpoint)
    }

    fn disconnect_output(self: &Arc<Self>, endpoint_id: &EndpointId) {
        let mut outputs = self.outputs.write().unwrap();

        if let Some(ep) = outputs.remove(endpoint_id) {
            ep.disconnect_flag.store(true, Ordering::Release);
            ep.unparker.unpark();
            self.status.remove_output(endpoint_id);
            // The circuit thread may be waiting for output buffer space.
            self.unpark_circuit();
        }
    }

    fn add_output_endpoint(
        self: &Arc<Self>,
        endpoint_name: &str,
        endpoint_config: &OutputEndpointConfig,
        endpoint: Option<Box<dyn OutputEndpoint>>,
    ) -> Result<EndpointId, ControllerError> {
        let mut outputs = self.outputs.write().unwrap();

        if outputs.lookup_by_name(endpoint_name).is_some() {
            Err(ControllerError::duplicate_output_endpoint(endpoint_name))?;
        }

        // Create output pipeline, consisting of an encoder, output probe and
        // transport endpoint; run the pipeline in a separate thread.
        //
        // ┌───────┐   ┌───────────┐   ┌────────┐
        // │encoder├──►│OutputProbe├──►│endpoint├──►
        // └───────┘   └───────────┘   └────────┘

        // Lookup output handle in catalog.
        let handles = self
            .catalog
            .lock()
            .unwrap()
            .output_query_handles(&endpoint_config.stream, endpoint_config.query)
            .ok_or_else(|| {
                ControllerError::unknown_output_stream(endpoint_name, &endpoint_config.stream)
            })?;

        let endpoint_id = outputs.alloc_endpoint_id();
        let endpoint_name_str = endpoint_name.to_string();

        let self_weak = Arc::downgrade(self);

        let is_fault_tolerant;

        endpoint_config
            .connector_config
            .output_buffer_config
            .validate()
            .map_err(|e| ControllerError::invalid_output_buffer_configuration(endpoint_name, &e))?;

        let encoder = if let Some(mut endpoint) = endpoint {
            endpoint
                .connect(Box::new(move |fatal: bool, e: AnyError| {
                    if let Some(controller) = self_weak.upgrade() {
                        controller.output_transport_error(endpoint_id, &endpoint_name_str, fatal, e)
                    }
                }))
                .map_err(|e| ControllerError::output_transport_error(endpoint_name, true, e))?;
            is_fault_tolerant = endpoint.is_fault_tolerant();

            // Create probe.
            let probe = Box::new(OutputProbe::new(
                endpoint_id,
                endpoint_name,
                endpoint,
                self.clone(),
            ));

            // Create encoder.
            let format_config = endpoint_config
                .connector_config
                .format
                .as_ref()
                .ok_or_else(|| ControllerError::output_format_not_specified(endpoint_name))?
                .clone();

            let format = <dyn OutputFormat>::get_format(&format_config.name).ok_or_else(|| {
                ControllerError::unknown_output_format(endpoint_name, &format_config.name)
            })?;
            format.new_encoder(endpoint_name, &format_config.config, &handles.schema, probe)?
        } else {
            // `endpoint` is `None` - instantiate an integrated endpoint.
            let endpoint = create_integrated_output_endpoint(
                endpoint_id,
                endpoint_name,
                endpoint_config,
                &handles.schema,
                self_weak,
            )?;

            is_fault_tolerant = endpoint.is_fault_tolerant();

            endpoint.into_encoder()
        };

        let parker = Parker::new();
        let endpoint_descr = OutputEndpointDescr::new(
            endpoint_name,
            &endpoint_config.stream,
            endpoint_config.query,
            parker.unparker().clone(),
            is_fault_tolerant,
        );
        let queue = endpoint_descr.queue.clone();
        let disconnect_flag = endpoint_descr.disconnect_flag.clone();
        let controller = self.clone();

        outputs.insert(endpoint_id, handles, endpoint_descr);

        let endpoint_name_string = endpoint_name.to_string();
        let output_buffer_config = endpoint_config
            .connector_config
            .output_buffer_config
            .clone();

        // Thread to run the output pipeline.
        spawn(move || {
            Self::output_thread_func(
                endpoint_id,
                endpoint_name_string,
                output_buffer_config,
                encoder,
                parker,
                queue,
                disconnect_flag,
                controller,
            )
        });

        drop(outputs);

        // Initialize endpoint stats.
        self.status
            .add_output(&endpoint_id, endpoint_name, endpoint_config);

        Ok(endpoint_id)
    }

    fn merge_batches(mut data: Vec<Arc<dyn SerBatch>>) -> Arc<dyn SerBatch> {
        let last = data.pop().unwrap();

        last.merge(data)
    }

    fn push_batch_to_encoder(
        batch: &dyn SerBatchReader,
        endpoint_id: EndpointId,
        endpoint_name: &str,
        encoder: &mut dyn Encoder,
        step: Step,
        controller: &ControllerInner,
    ) {
        encoder.consumer().batch_start(step);
        encoder
            .encode(batch)
            .unwrap_or_else(|e| controller.encode_error(endpoint_id, endpoint_name, e));
        controller.wait_to_commit(step);
        encoder.consumer().batch_end();
    }

    #[allow(clippy::too_many_arguments)]
    fn output_thread_func(
        endpoint_id: EndpointId,
        endpoint_name: String,
        output_buffer_config: OutputBufferConfig,
        mut encoder: Box<dyn Encoder>,
        parker: Parker,
        queue: Arc<BatchQueue>,
        disconnect_flag: Arc<AtomicBool>,
        controller: Arc<ControllerInner>,
    ) {
        let mut output_buffer = OutputBuffer::new(&endpoint_name);

        loop {
            if controller.state() == PipelineState::Terminated {
                return;
            }

            if disconnect_flag.load(Ordering::Acquire) {
                return;
            }

            if output_buffer.flush_needed(&output_buffer_config) {
                // One of the triggering conditions for flushing the output buffer is satisfied:
                // go ahead and flush the buffer; we will check for more messages at the next iteration
                // of the loop.
                Self::push_batch_to_encoder(
                    output_buffer.take_buffer().unwrap().as_batch_reader(),
                    endpoint_id,
                    &endpoint_name,
                    encoder.as_mut(),
                    output_buffer.buffered_step,
                    &controller,
                );

                controller
                    .status
                    .output_buffered_batches(endpoint_id, output_buffer.buffered_processed_records);
            } else if let Some((step, data, processed_records)) = queue.pop() {
                // Dequeue the next output batch. If output buffering is enabled, push it to the
                // buffer; we will check if the buffer needs to be flushed at the next iteration of
                // the loop.  If buffering is disabled, push the buffer directly to the encoder.

                let num_records = data.iter().map(|b| b.len()).sum();
                let consolidated = Self::merge_batches(data);

                // trace!("Pushing {num_records} records to output endpoint {endpoint_name}");

                // Buffer the new output if buffering is enabled.
                if output_buffer_config.enable_output_buffer {
                    output_buffer.insert(consolidated, step, processed_records);
                    controller.status.buffer_batch(
                        endpoint_id,
                        num_records,
                        &controller.circuit_thread_unparker,
                    );
                } else {
                    Self::push_batch_to_encoder(
                        consolidated.as_batch_reader(),
                        endpoint_id,
                        &endpoint_name,
                        encoder.as_mut(),
                        step,
                        &controller,
                    );

                    // `num_records` output records have been transmitted --
                    // update output stats, wake up the circuit thread if the
                    // number of queued records drops below high watermark.
                    controller.status.output_batch(
                        endpoint_id,
                        processed_records,
                        num_records,
                        &controller.circuit_thread_unparker,
                    );
                }
            } else {
                trace!("Queue is empty -- wait for the circuit thread to wake us up when more data is available");
                if let Some(buffer_since) = output_buffer.buffer_since() {
                    // Buffering is enabled: wake us up when the buffer timeout has expired.
                    let timeout = output_buffer_config.max_output_buffer_time_millis as i128
                        - buffer_since.elapsed().as_millis() as i128;
                    if timeout > 0 {
                        parker.park_timeout(Duration::from_millis(timeout as u64));
                    }
                } else {
                    parker.park();
                }
            }
        }
    }

    /// Waits for input `step` to commit.
    ///
    /// An input step must commit before we can commit any of the output.
    /// Otherwise, if there is a crash, we might have output for which we don't
    /// know the corresponding input, which would break fault tolerance.
    fn wait_to_commit(&self, step: Step) {
        if !self.status.is_step_committed(step) {
            let mut guard = self.uncommitted_step.lock().unwrap();
            loop {
                let uncommitted_step = self.status.uncommitted_step().unwrap();
                if uncommitted_step > *guard {
                    *guard = uncommitted_step;
                    self.step_committed.notify_all();
                }
                if uncommitted_step < step {
                    return;
                }
                guard = self.step_committed.wait(guard).unwrap();
            }
        }
    }

    fn state(self: &Arc<Self>) -> PipelineState {
        self.status.state()
    }

    fn start(self: &Arc<Self>) {
        self.status.set_state(PipelineState::Running);
        self.unpark_backpressure();
    }

    fn pause(self: &Arc<Self>) {
        self.status.set_state(PipelineState::Paused);
        self.unpark_backpressure();
    }

    fn stop(self: &Arc<Self>) {
        let mut inputs = self.inputs.lock().unwrap();

        for ep in inputs.values() {
            ep.reader.disconnect();
        }
        inputs.clear();

        self.status.set_state(PipelineState::Terminated);

        self.unpark_circuit();
        self.unpark_backpressure();
    }

    fn graph_profile(&self, cb: GraphProfileCallbackFn) {
        self.profile_request.send(cb).unwrap();
        self.unpark_circuit();
    }

    fn error(&self, error: ControllerError) {
        (self.error_cb)(error);
    }

    /// Process an input transport error.
    ///
    /// Update endpoint stats and notify the error callback.
    pub fn input_transport_error(
        &self,
        endpoint_id: EndpointId,
        endpoint_name: &str,
        fatal: bool,
        error: AnyError,
    ) {
        self.status
            .input_transport_error(endpoint_id, fatal, &error);
        self.error(ControllerError::input_transport_error(
            endpoint_name,
            fatal,
            error,
        ));
    }

    pub fn parse_error(&self, endpoint_id: EndpointId, endpoint_name: &str, error: ParseError) {
        self.status.parse_error(endpoint_id);
        self.error(ControllerError::parse_error(endpoint_name, error));
    }

    pub fn encode_error(&self, endpoint_id: EndpointId, endpoint_name: &str, error: AnyError) {
        self.status.encode_error(endpoint_id);
        self.error(ControllerError::encode_error(endpoint_name, error));
    }

    /// Process an output transport error.
    ///
    /// Update endpoint stats and notify the error callback.
    pub fn output_transport_error(
        &self,
        endpoint_id: EndpointId,
        endpoint_name: &str,
        fatal: bool,
        error: AnyError,
    ) {
        self.status
            .output_transport_error(endpoint_id, fatal, &error);
        self.error(ControllerError::output_transport_error(
            endpoint_name,
            fatal,
            error,
        ));
    }

    /// Update counters after receiving a new input batch.
    ///
    /// See [ControllerStatus::input_batch].
    pub fn input_batch(&self, endpoint_id: EndpointId, num_bytes: usize, num_records: usize) {
        self.status.input_batch(
            endpoint_id,
            num_bytes,
            num_records,
            &self.circuit_thread_unparker,
            &self.backpressure_thread_unparker,
        )
    }

    /// Update counters after receiving an end-of-input event on an input
    /// endpoint.
    ///
    /// See [`ControllerStatus::eoi`].
    pub fn eoi(&self, endpoint_id: EndpointId, num_records: usize) {
        self.status
            .eoi(endpoint_id, num_records, &self.circuit_thread_unparker)
    }

    pub fn start_step(&self, endpoint_id: EndpointId, step: Step) {
        self.status.start_step(endpoint_id, step);
        self.circuit_thread_unparker.unpark();
    }

    pub fn committed(&self, endpoint_id: EndpointId, step: Step) {
        self.status.committed(endpoint_id, step);
        self.circuit_thread_unparker.unpark();
    }

    fn output_buffers_full(&self) -> bool {
        self.status.output_buffers_full()
    }
}

/// An input probe inserted between the transport endpoint and the parser to
/// track stats and errors.
struct InputProbe {
    endpoint_id: EndpointId,
    endpoint_name: String,
    parser: Box<dyn Parser>,
    controller: Arc<ControllerInner>,
}

impl InputProbe {
    fn new(
        endpoint_id: EndpointId,
        endpoint_name: &str,
        parser: Box<dyn Parser>,
        controller: Arc<ControllerInner>,
    ) -> Self {
        Self {
            endpoint_id,
            endpoint_name: endpoint_name.to_owned(),
            parser,
            controller,
        }
    }

    fn input_common(
        &mut self,
        data: &[u8],
        num_records: usize,
        errors: Vec<ParseError>,
    ) -> Vec<ParseError> {
        for error in errors.iter() {
            self.controller
                .parse_error(self.endpoint_id, &self.endpoint_name, error.clone());
        }
        self.controller
            .input_batch(self.endpoint_id, data.len(), num_records);

        errors
    }
}

/// `InputConsumer` interface exposed to the transport endpoint.
impl InputConsumer for InputProbe {
    fn input_fragment(&mut self, data: &[u8]) -> Vec<ParseError> {
        let (num_records, errors) = self.parser.input_fragment(data);
        self.input_common(data, num_records, errors)
    }

    fn input_chunk(&mut self, data: &[u8]) -> Vec<ParseError> {
        let (num_records, errors) = self.parser.input_chunk(data);
        self.input_common(data, num_records, errors)
    }

    fn eoi(&mut self) -> Vec<ParseError> {
        // The endpoint reached end-of-file.  Notify and flush the parser (even though
        // no new data has been received, the parser may contain some partially
        // parsed data and may be waiting for, e.g., and end-of-line or
        // end-of-file to finish parsing it).
        let (num_records, errors) = self.parser.eoi();
        for error in errors.iter() {
            self.controller
                .parse_error(self.endpoint_id, &self.endpoint_name, error.clone());
        }
        self.controller.eoi(self.endpoint_id, num_records);

        errors
    }

    fn error(&mut self, fatal: bool, error: AnyError) {
        self.controller
            .input_transport_error(self.endpoint_id, &self.endpoint_name, fatal, error);
    }

    fn fork(&self) -> Box<dyn InputConsumer> {
        Box::new(Self::new(
            self.endpoint_id,
            &self.endpoint_name,
            self.parser.fork(),
            self.controller.clone(),
        ))
    }

    fn start_step(&mut self, step: Step) {
        self.controller.start_step(self.endpoint_id, step);
    }

    fn committed(&mut self, step: Step) {
        self.controller.committed(self.endpoint_id, step);
    }
}

/// An output probe inserted between the encoder and the output transport
/// endpoint to track stats.
struct OutputProbe {
    endpoint_id: EndpointId,
    endpoint_name: String,
    endpoint: Box<dyn OutputEndpoint>,
    controller: Arc<ControllerInner>,
}

impl OutputProbe {
    pub fn new(
        endpoint_id: EndpointId,
        endpoint_name: &str,
        endpoint: Box<dyn OutputEndpoint>,
        controller: Arc<ControllerInner>,
    ) -> Self {
        Self {
            endpoint_id,
            endpoint_name: endpoint_name.to_owned(),
            endpoint,
            controller,
        }
    }
}

impl OutputConsumer for OutputProbe {
    fn max_buffer_size_bytes(&self) -> usize {
        self.endpoint.max_buffer_size_bytes()
    }

    fn batch_start(&mut self, step: Step) {
        self.endpoint.batch_start(step).unwrap_or_else(|e| {
            self.controller
                .output_transport_error(self.endpoint_id, &self.endpoint_name, false, e);
        })
    }

    fn push_buffer(&mut self, buffer: &[u8], num_records: usize) {
        let num_bytes = buffer.len();

        match self.endpoint.push_buffer(buffer) {
            Ok(()) => {
                self.controller
                    .status
                    .output_buffer(self.endpoint_id, num_bytes, num_records);
            }
            Err(error) => {
                self.controller.output_transport_error(
                    self.endpoint_id,
                    &self.endpoint_name,
                    false,
                    error,
                );
            }
        }
    }

    fn push_key(&mut self, key: &[u8], val: &[u8], num_records: usize) {
        let num_bytes = key.len() + val.len();

        match self.endpoint.push_key(key, val) {
            Ok(()) => {
                self.controller
                    .status
                    .output_buffer(self.endpoint_id, num_bytes, num_records);
            }
            Err(error) => {
                self.controller.output_transport_error(
                    self.endpoint_id,
                    &self.endpoint_name,
                    false,
                    error,
                );
            }
        }
    }

    fn batch_end(&mut self) {
        self.endpoint.batch_end().unwrap_or_else(|e| {
            self.controller
                .output_transport_error(self.endpoint_id, &self.endpoint_name, false, e);
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        test::{generate_test_batch, test_circuit, wait, TestStruct, DEFAULT_TIMEOUT_MS},
        Controller, PipelineConfig,
    };
    use csv::{ReaderBuilder as CsvReaderBuilder, WriterBuilder as CsvWriterBuilder};
    use std::fs::remove_file;
    use tempfile::NamedTempFile;

    use proptest::prelude::*;

    // TODO: Parameterize this with config string, so we can test different
    // input/output formats and transports when we support more than one.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(30))]
        #[test]
        fn proptest_csv_file(
            data in generate_test_batch(5000),
            min_batch_size_records in 1..100usize,
            max_buffering_delay_usecs in 1..2000usize,
            input_buffer_size_bytes in 1..1000usize,
            output_buffer_size_records in 1..100usize)
        {
            let temp_input_file = NamedTempFile::new().unwrap();
            let temp_output_path = NamedTempFile::new().unwrap().into_temp_path();
            let output_path = temp_output_path.to_str().unwrap().to_string();
            temp_output_path.close().unwrap();

            let config_str = format!(
                r#"
min_batch_size_records: {min_batch_size_records}
max_buffering_delay_usecs: {max_buffering_delay_usecs}
name: test
workers: 4
inputs:
    test_input1:
        stream: test_input1
        transport:
            name: file_input
            config:
                path: {:?}
                buffer_size_bytes: {input_buffer_size_bytes}
                follow: false
        format:
            name: csv
outputs:
    test_output1:
        stream: test_output1
        transport:
            name: file_output
            config:
                path: {:?}
        format:
            name: csv
            config:
                buffer_size_records: {output_buffer_size_records}
        "#,
            temp_input_file.path().to_str().unwrap(),
            output_path,
            );

            println!("input file: {}", temp_input_file.path().to_str().unwrap());
            println!("output file: {output_path}");
            let config: PipelineConfig = serde_yaml::from_str(&config_str).unwrap();
            let controller = Controller::with_config(
                    |circuit_config| Ok(test_circuit::<TestStruct>(circuit_config, &[])),
                    &config,
                    Box::new(|e| panic!("error: {e}")),
                )
                .unwrap();

            let mut writer = CsvWriterBuilder::new()
                .has_headers(false)
                .from_writer(temp_input_file.as_file());

            for val in data.iter().cloned() {
                writer.serialize(val).unwrap();
            }
            writer.flush().unwrap();
            controller.start();

            // Wait for the pipeline to output all records.
            wait(|| controller.pipeline_complete(), DEFAULT_TIMEOUT_MS).unwrap();

            assert_eq!(controller.status().output_status().get(&0).unwrap().transmitted_records(), data.len() as u64);

            controller.stop().unwrap();

            let mut expected = data;
            expected.sort();

            let mut actual: Vec<_> = CsvReaderBuilder::new()
                .has_headers(false)
                .from_path(&output_path)
                .unwrap()
                .deserialize::<(TestStruct, i32)>()
                .map(|res| {
                    let (val, weight) = res.unwrap();
                    assert_eq!(weight, 1);
                    val
                })
                .collect();
            actual.sort();

            // Don't leave garbage in the FS.
            remove_file(&output_path).unwrap();

            assert_eq!(actual, expected);
        }
    }
}
