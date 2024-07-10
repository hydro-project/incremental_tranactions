pub mod byname_max_sql;
pub mod byname_max_sql_incremental;
pub mod byname_sql;
pub mod byname_sql_incremental;
pub mod payment_sql;
pub mod payment_sql_incremental;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[allow(non_upper_case_globals)]
#[export_name = "malloc_conf"]
pub static malloc_conf: &[u8] = b"prof:true,prof_active:true,lg_prof_sample:19\0";

#[cfg(test)]
mod test {
    use super::*;
    use dbsp::circuit::CircuitConfig;
    use rust_decimal::Decimal;

    #[test]
    fn test_payment_sql() {
        let cconf = CircuitConfig::with_workers(1);
        let (mut circuit, handles) = payment_sql::circuit(cconf).unwrap();

        let (
            in_warehouse_static,
            in_warehouse,
            in_district_static,
            in_district_next_id,
            in_district_ytd,
            in_customer,
            in_transaction_parameters,
            out_warehouse_updates,
            out_warehouse_select,
            out_district_updates,
            out_district_select,
            out_customer_select,
            out_customer_update,
            out_history_insert,
        ) = handles;

        // TODO: Push input into the circuit
        // in_warehouse_static.push(...)

        circuit.step().unwrap();

        // TODO: Read output from the circuit
    }

    #[test]
    fn test_payment_sql_incremental() {
        let cconf = CircuitConfig::with_workers(1);
        let (mut circuit, handles) = payment_sql_incremental::circuit(cconf).unwrap();

        let (
            in_warehouse_static,
            in_warehouse,
            in_district_static,
            in_district_next_id,
            in_district_ytd,
            in_customer,
            in_transaction_parameters,
            out_warehouse_updates,
            out_warehouse_select,
            out_district_updates,
            out_district_select,
            out_customer_select,
            out_customer_update,
            out_history_insert,
        ) = handles;

        circuit.step().unwrap();
    }
}
