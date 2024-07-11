pub mod byname_sql;
pub mod byname_sql_incremental;
pub mod payment_sql;
pub mod payment_sql_incremental;

#[cfg(test)]
mod test {
    use super::*;
    use dbsp::circuit::CircuitConfig;
    // use rust_decimal::Decimal;

    #[test]
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
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

    #[test]
    #[allow(unused_variables)]
    fn test_byname_sql() {
        let cconf = CircuitConfig::with_workers(1);
        let (mut circuit, handles) = byname_sql::circuit(cconf).unwrap();
        let (
            in_warehouse_static,
            in_warehouse,
            in_district_static,
            in_district_next_id,
            in_district_ytd,
            in_customer,
            in_transaction_parameters,
            out_cust_agg,
            out_cust_byname,
        ) = handles;
        let mut customers: Vec<_> = (0..5)
            .into_iter()
            .map(|i| {
                (
                    (
                        Some(i),             // c_id
                        Some(1),             // c_d_id
                        Some(1),             // c_w_id
                        Some(i.to_string()), // c_first
                        None,
                        Some("Public".to_string()), // c_last
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
                    1,
                )
                    .into()
            })
            .collect();
        in_customer.append(&mut customers);

        circuit.step().unwrap();
        let byname = out_cust_byname.consolidate();
        let agg = out_cust_agg.consolidate();

        println!("Loaded customers (nothing to join, should be empty):");
        byname
            .iter()
            .for_each(|(tup, _, z_weight)| println!("{:?}: {:?}", tup.0.unwrap(), z_weight));
        println!("cust_agg:");
        agg.iter().for_each(|(tup, _, z_weight)| {
            println!(
                "{:?}: {:?}",
                tup.0
                    .iter()
                    .map(|x| match x {
                        Some(value) => format!("{:?}, ", value),
                        None => "_,".to_string(),
                    })
                    .collect::<String>(),
                z_weight
            )
        });

        in_transaction_parameters.push(
            (
                Some(4344),                 // txn_id
                None,                       // w_id
                None,                       // d_id
                None,                       // c_id
                Some(1),                    // c_w_id
                Some(1),                    // c_w_id
                Some("Public".to_string()), // c_last
                None,                       // h_amount
                None,                       // h_date
                None,                       // datetime_
            )
                .into(),
            1,
        );
        circuit.step().unwrap();
        let byname = out_cust_byname.consolidate().merge(&byname);
        let agg = out_cust_agg.consolidate().merge(&agg);
        println!("Result");
        byname
            .iter()
            .for_each(|(tup, _, z_weight)| println!("{:?}: {:?}", tup.0.unwrap(), z_weight));
        println!("cust_agg:");
        agg.iter().for_each(|(tup, _, z_weight)| {
            println!(
                "{:?}: {:?}",
                tup.0
                    .iter()
                    .map(|x| match x {
                        Some(value) => format!("{:?}, ", value),
                        None => "_,".to_string(),
                    })
                    .collect::<String>(),
                z_weight
            )
        });
    }
}
