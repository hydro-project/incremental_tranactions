use std::cell::RefCell;

use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup,
    BenchmarkId, Criterion,
};
use dbsp::{
    circuit::CircuitConfig,
    utils::{Tup10, Tup2},
};
use tpcc::{
    byname_max_sql::{self, Tup21},
    byname_max_sql_incremental,
};

#[derive(Clone, Copy)]
struct ExperimentInput {
    base_size: i32,
    updates_size: i32,
}

impl std::fmt::Display for ExperimentInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Base Size={}, Updates Size={}",
            self.base_size, self.updates_size
        )
    }
}

fn byname(c: &mut Criterion) {
    let num_workers = 1;
    // Input the initial size of the customer relation and the number of updates
    let base_size = 3000; // Number of customers per district
    let update_sizes = vec![0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    let inputs: Vec<_> = update_sizes
        .into_iter()
        .map(|updates_size| ExperimentInput {
            base_size,
            updates_size,
        })
        .collect();

    let mut group = c.benchmark_group("Byname (Max)");

    byname_plain(&mut group, inputs.clone(), num_workers);
    byname_incremental(&mut group, inputs.clone(), num_workers);

    group.finish();
}

fn byname_plain(
    group: &mut BenchmarkGroup<WallTime>,
    inputs: Vec<ExperimentInput>,
    num_workers: usize,
) {
    for input in inputs {
        group.throughput(criterion::Throughput::Elements(input.updates_size as u64));
        group.bench_with_input(BenchmarkId::new("Plain", input), &input, |b, input| {
            /*
            c_id INT,
            c_d_id INT,
            c_w_id INT,
            c_first VARCHAR(16),
            c_middle CHAR(2),
            c_last VARCHAR(16),
             */
            let c_id = 1;
            let c_d_id = 1;
            let c_w_id = 1;
            //let c_first = "first_name".to_string();
            let c_middle = "".to_string();
            let c_last = "last_name".to_string();
            let weight = 1;

            let base_vals: Vec<
                Tup2<Tup21<_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _>, _>,
            > = (0..input.base_size)
                .into_iter()
                .map(|i| {
                    (
                        (
                            Some(c_id),
                            Some(c_d_id),
                            Some(c_w_id),
                            Some(i.to_string()), // first name
                            Some(c_middle.clone()),
                            Some(c_last.clone()),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                            .into(),
                        weight,
                    )
                        .into()
                })
                .collect();

            let update_vals: Vec<
                Tup2<Tup21<_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _>, _>,
            > = (0..input.updates_size)
                .into_iter()
                .map(|i| {
                    (
                        (
                            Some(c_id),
                            Some(c_d_id),
                            Some(c_w_id),
                            Some((input.base_size + i).to_string()),
                            Some(c_middle.clone()),
                            Some(c_last.clone()),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        )
                            .into(),
                        weight,
                    )
                        .into()
                })
                .collect();

            let txn_id = 1;
            let transaction_paramters: Tup10<_, _, _, _, _, _, _, _, _, _> = (
                Some(txn_id),
                None,
                None,
                None,
                Some(c_w_id),
                Some(c_d_id),
                Some(c_last),
                None,
                None,
                None,
            )
                .into();

            // Create references to be passed into the benchmark
            let base_vals = RefCell::new(base_vals);
            let base_vals_mut_ref = base_vals.borrow_mut();
            let update_vals = RefCell::new(update_vals);
            let update_vals_mut_ref = update_vals.borrow_mut();
            let transaction_paramters = RefCell::new(transaction_paramters);
            let transaction_paramters_ref = transaction_paramters.borrow();

            b.iter_batched(
                || {
                    // Initialize a new circuit
                    let cconf = CircuitConfig::with_workers(num_workers);
                    let (circuit, handles) = byname_max_sql::circuit(cconf).unwrap();
                    let (
                        _in_warehouse_static,
                        _in_warehouse,
                        _in_district_static,
                        _in_district_next_id,
                        _in_district_ytd,
                        in_customer,
                        in_transaction_parameters,
                        out_cust_max_order_by,
                    ) = handles;

                    // Return the relevant handles and references to the data
                    (
                        circuit,
                        in_customer,
                        in_transaction_parameters,
                        out_cust_max_order_by,
                        base_vals_mut_ref.clone(),
                        update_vals_mut_ref.clone(),
                        transaction_paramters_ref.clone(),
                    )
                },
                |(
                    mut circuit,
                    in_customer,
                    in_transaction_parameters,
                    out_cust_max_order_by,
                    mut base_vals,
                    mut update_vals,
                    transaction_paramters,
                )| {
                    // Do the initial load
                    // Send the base data
                    in_customer.append(base_vals.as_mut());

                    // Send the transaction parameters
                    in_transaction_parameters.push(transaction_paramters.clone(), 1);

                    // Execute the circuit on these inputs
                    circuit.step().unwrap();

                    // Get output
                    let out = out_cust_max_order_by.consolidate();

                    let result: Vec<_> = out.iter().collect();
                    black_box(result);

                    // Repeat the initial input for the for the non-incremental circuit
                    in_customer.append(base_vals.as_mut());
                    in_transaction_parameters.push(transaction_paramters.clone(), 1);

                    // Send the updates
                    in_customer.append(update_vals.as_mut());

                    // Execute the circuit on the updated inputs
                    circuit.step().unwrap();

                    // Get output
                    let out = out_cust_max_order_by.consolidate();

                    let result: Vec<_> = out.iter().collect();
                    black_box(result);
                },
                BatchSize::SmallInput,
            );
        });
    }
}

fn byname_incremental(
    group: &mut BenchmarkGroup<WallTime>,
    inputs: Vec<ExperimentInput>,
    num_workers: usize,
) {
    for input in inputs {
        group.throughput(criterion::Throughput::Elements(input.updates_size as u64));
        group.bench_with_input(
            BenchmarkId::new("Incremental", input),
            &input,
            |b, input| {
                /*
                c_id INT,
                c_d_id INT,
                c_w_id INT,
                c_first VARCHAR(16),
                c_middle CHAR(2),
                c_last VARCHAR(16),
                 */
                let c_id = 1;
                let c_d_id = 1;
                let c_w_id = 1;
                //let c_first = "first_name".to_string();
                let c_middle = "".to_string();
                let c_last = "last_name".to_string();
                let weight = 1;

                let base_vals: Vec<
                    Tup2<
                        byname_max_sql_incremental::Tup21<
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        >,
                        _,
                    >,
                > = (0..input.base_size)
                    .into_iter()
                    .map(|i| {
                        (
                            (
                                Some(c_id),
                                Some(c_d_id),
                                Some(c_w_id),
                                Some(i.to_string()), // first name
                                Some(c_middle.clone()),
                                Some(c_last.clone()),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                            )
                                .into(),
                            weight,
                        )
                            .into()
                    })
                    .collect();

                let update_vals: Vec<
                    Tup2<
                        byname_max_sql_incremental::Tup21<
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        >,
                        _,
                    >,
                > = (0..input.updates_size)
                    .into_iter()
                    .map(|i| {
                        (
                            (
                                Some(c_id),
                                Some(c_d_id),
                                Some(c_w_id),
                                Some((input.base_size + i).to_string()),
                                Some(c_middle.clone()),
                                Some(c_last.clone()),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                            )
                                .into(),
                            weight,
                        )
                            .into()
                    })
                    .collect();

                let txn_id = 1;
                let transaction_paramters: Tup10<_, _, _, _, _, _, _, _, _, _> = (
                    Some(txn_id),
                    None,
                    None,
                    None,
                    Some(c_w_id),
                    Some(c_d_id),
                    Some(c_last),
                    None,
                    None,
                    None,
                )
                    .into();

                // Create references to be passed into the benchmark
                let base_vals = RefCell::new(base_vals);
                let base_vals_mut_ref = base_vals.borrow_mut();
                let update_vals = RefCell::new(update_vals);
                let update_vals_mut_ref = update_vals.borrow_mut();
                let transaction_paramters = RefCell::new(transaction_paramters);
                let transaction_paramters_ref = transaction_paramters.borrow();

                b.iter_batched(
                    || {
                        // Initialize a new circuit
                        let cconf = CircuitConfig::with_workers(num_workers);
                        //let (circuit, handles) = byname_sql::circuit(cconf).unwrap();
                        let (circuit, handles) =
                            byname_max_sql_incremental::circuit(cconf).unwrap();
                        let (
                            _in_warehouse_static,
                            _in_warehouse,
                            _in_district_static,
                            _in_district_next_id,
                            _in_district_ytd,
                            in_customer,
                            in_transaction_parameters,
                            out_cust_max_order_by,
                        ) = handles;

                        // Return the relevant handles and references to the data
                        (
                            circuit,
                            in_customer,
                            in_transaction_parameters,
                            out_cust_max_order_by,
                            base_vals_mut_ref.clone(),
                            update_vals_mut_ref.clone(),
                            transaction_paramters_ref.clone(),
                        )
                    },
                    |(
                        mut circuit,
                        in_customer,
                        in_transaction_parameters,
                        out_cust_max_order_by,
                        mut base_vals,
                        mut update_vals,
                        transaction_paramters,
                    )| {
                        // Send the base data
                        in_customer.append(base_vals.as_mut());

                        // Send the transaction parameters
                        in_transaction_parameters.push(transaction_paramters.clone(), 1);

                        // Execute the circuit on these inputs
                        circuit.step().unwrap();

                        // Get output
                        let out = out_cust_max_order_by.consolidate();

                        let result: Vec<_> = out.iter().collect();
                        black_box(result);

                        // Repeat the initial input for the for the non-incremental circuit
                        in_customer.append(base_vals.as_mut());
                        in_transaction_parameters.push(transaction_paramters.clone(), 1);

                        // Send the updates
                        in_customer.append(update_vals.as_mut());

                        // Execute the circuit on the updated inputs
                        circuit.step().unwrap();

                        // Get output
                        let out = out_cust_max_order_by.consolidate();

                        let result: Vec<_> = out.iter().collect();
                        black_box(result);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
}

//criterion_group!(benches, byname_plain /* byname_incremental */,);
criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(30));
    //targets = byname_plain, byname_incremental
    targets = byname
}
criterion_main!(benches);
