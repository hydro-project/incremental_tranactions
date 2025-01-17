#!/bin/bash
#
# Usage (to test): $ SMOKE=true bash scripts/bench.bash
#
# - `SMOKE`: Run benchmarks quickly but not representative
#
set -ex

# Clean-up old results
if [ "$SMOKE" = "" ]; then
rm -rf gh-pages
fi
NEXMARK_CSV_FILE='nexmark_results.csv'
NEXMARK_DRAM_CSV_FILE='dram_nexmark_results.csv'
NEXMARK_SQL_CSV_FILE='sql_nexmark_results.csv'
NEXMARK_SQL_METRICS_CSV_FILE='sql_nexmark_metrics.csv'
NEXMARK_SQL_STORAGE_CSV_FILE='sql_storage_nexmark_results.csv'
NEXMARK_SQL_STORAGE_METRICS_CSV_FILE='sql_storage_nexmark_metrics.csv'
NEXMARK_PERSISTENCE_CSV_FILE='persistence_nexmark_results.csv'
GALEN_CSV_FILE='galen_results.csv'
LDBC_CSV_FILE='ldbc_results.csv'
rm -f crates/nexmark/${NEXMARK_CSV_FILE} crates/nexmark/${NEXMARK_SQL_CSV_FILE} crates/dbsp/${GALEN_CSV_FILE} crates/dbsp/${LDBC_CSV_FILE} crates/nexmark/${NEXMARK_DRAM_CSV_FILE} crates/nexmark/${NEXMARK_PERSISTENCE_CSV_FILE}

# Run nexmark benchmark
EVENT_RATE=10000000
MAX_EVENTS=100000000
GENERATORS=8
CORES=6
if [ "$SMOKE" != "" ]; then
  EVENT_RATE=10000000
  MAX_EVENTS=1000000
fi

FILES=( "q0" "q1" "q2" "q3" "q4" "q5" "q6" "q7" "q8" "q9" "q12" "q13" "q14" "q15" "q16" "q17" "q18" "q19" "q20" "q21" "q22" )
for FILE in "${FILES[@]}"
  do cargo bench --bench nexmark -- --first-event-rate=${EVENT_RATE} --max-events=${MAX_EVENTS} --cpu-cores ${CORES}  --num-event-generators ${GENERATORS} --source-buffer-size 10000 --input-batch-size 40000 --csv ${NEXMARK_CSV_FILE} --query $FILE
done

# Run nexmark SQL benchmark
# This test requires a running instance of redpanda and pipeline-manager.
# The Earthfile should run those.
# 100M events causes out of memory problems with SQL tests
MAX_EVENTS=10000000
if [ "$SMOKE" != "" ]; then
  MAX_EVENTS=1000000
fi
KAFKA_BROKER=localhost:9092
rpk topic -X brokers=$KAFKA_BROKER delete bid auction person
cargo run  -p dbsp_nexmark --example generate --features with-kafka -- --max-events ${MAX_EVENTS} -O bootstrap.servers=$KAFKA_BROKER

FELDERA_API=http://localhost:8080
nexmark_sql_benchmark() {
    local csv=$1 metrics=$2; shift; shift
    python3 benchmark/feldera-sql/run.py \
	    --api-url $FELDERA_API \
	    -O bootstrap.servers=$KAFKA_BROKER \
	    --csv "crates/nexmark/$csv" \
	    --csv-metrics "crates/nexmark/$metrics" \
	    --metrics-interval 1 \
	    --poller-threads 10 \
	    "$@"
}
nexmark_sql_benchmark "${NEXMARK_SQL_CSV_FILE}" "${NEXMARK_SQL_METRICS_CSV_FILE}"
nexmark_sql_benchmark "${NEXMARK_SQL_STORAGE_CSV_FILE}" "${NEXMARK_SQL_STORAGE_METRICS_CSV_FILE}" --storage

# Run galen benchmark
cargo bench --bench galen -- --workers 10 --csv ${GALEN_CSV_FILE}

# Run ldbc benchmarks
DATASET_SMALL='graph500-22'
DATASET_MEDIUM='datagen-8_4-fb'
if [ "$SMOKE" != "" ]; then
    DATASET_SMALL='wiki-Talk'
    DATASET_MEDIUM='kgs'
fi
#cargo bench --bench ldbc-graphalytics -- bfs ${DATASET_SMALL} --threads 1 --csv ${LDBC_CSV_FILE}
#cargo bench --bench ldbc-graphalytics -- bfs ${DATASET_MEDIUM} --threads 6 --csv ${LDBC_CSV_FILE}
#cargo bench --bench ldbc-graphalytics -- pagerank ${DATASET_SMALL} --threads 1 --csv ${LDBC_CSV_FILE}
#cargo bench --bench ldbc-graphalytics -- pagerank ${DATASET_MEDIUM} --threads 6 --csv ${LDBC_CSV_FILE}

# Run nexmark benchmark with persistence
EVENT_RATE=5000000
MAX_EVENTS=3000000
CORES=1
if [ "$SMOKE" != "" ]; then
  EVENT_RATE=5000000
  MAX_EVENTS=100000
fi
cargo bench --bench nexmark -- --first-event-rate=${EVENT_RATE} --max-events=${MAX_EVENTS} --cpu-cores ${CORES} --num-event-generators 6 --source-buffer-size 10000 --input-batch-size 40000 --csv ${NEXMARK_DRAM_CSV_FILE}
#cargo bench --bench nexmark --features persistence -- --first-event-rate=${EVENT_RATE} --max-events=${MAX_EVENTS} --cpu-cores ${CORES} --num-event-generators 6 --source-buffer-size 10000 --input-batch-size 40000 --csv ${NEXMARK_PERSISTENCE_CSV_FILE}
