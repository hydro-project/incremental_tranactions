pub mod people_example_sql;
pub mod people_example_sql_incremental;

#[cfg(test)]
mod tests {
    use dbsp::circuit::CircuitConfig;

    use super::people_example_sql::circuit;
    use super::people_example_sql_incremental::circuit as circuit_incremental;

    #[test]
    pub fn test_people_example() {
        // XXX: We need to create a circuit configuration! The quickstart is outdated?
        let cconfig = CircuitConfig::with_workers(2);

        // This circuit maintains a view of persons over 18
        // Note: The circuit function returns a tuple of input handles and output streams,
        // presumably in the order they were declared in the original SQL file
        let (mut circuit, (person, adult, num_adult, _some_output)) = circuit(cconfig).unwrap();
        // Feed two input records to the circuit.
        // First input has a count of "1"
        person.push(("Bob".to_string(), Some(12), Some(true)).into(), 1);
        person.push(("Bob".to_string(), Some(40), Some(true)).into(), 1);
        // Second input has a count of "2"
        person.push(("Tom".to_string(), Some(20), Some(false)).into(), 2);
        // Append a batch of inputs to the circuit
        let mut vals = vec![
            (("Alice".to_string(), Some(25), Some(true)).into(), 1).into(),
            (("Kate".to_string(), Some(19), Some(true)).into(), 2).into(),
            (("Bob".to_string(), Some(64), Some(true)).into(), 1).into(),
        ];
        person.append(&mut vals);

        // Execute the circuit on these inputs
        circuit.step().unwrap();

        // Read the produced output
        let out = adult.consolidate();
        let num_adult_output = num_adult.consolidate();

        // Print the produced output
        println!("Number of adults:");
        num_adult_output
            .iter()
            .for_each(|(output_tuple, _, z_weight)| {
                println!("{:?} ({})", output_tuple.0, z_weight)
            });
        println!("Adults:");
        out.iter()
            .for_each(|(tup, _, z_weight)| println!("{}: {}", tup.0, z_weight));

        // Reexecute circuit with new inputs
        person.push(("Bob".to_string(), Some(41), Some(true)).into(), 1);
        // Execute the circuit on these inputs
        circuit.step().unwrap();

        // Read the new output of this iteration (non-incremental)
        let out = adult.consolidate();
        let num_adult_output = num_adult.consolidate();

        // Print the produced output
        println!("Number of adults:");
        num_adult_output
            .iter()
            .for_each(|(output_tuple, _, z_weight)| {
                println!("{:?} ({})", output_tuple.0, z_weight)
            });
        println!("Adults:");
        out.iter()
            .for_each(|(tup, _, z_weight)| println!("{}: {}", tup.0, z_weight));
    }

    #[test]
    pub fn test_people_example_incremental() {
        // XXX: We need to create a circuit configuration! The quickstart is outdated?
        let cconfig = CircuitConfig::with_workers(2);

        // This circuit maintains a view of persons over 18
        // Note: The circuit function returns a tuple of input handles and output streams,
        // presumably in the order they were declared in the original SQL file
        let (mut circuit, (person, adult, num_adult, _some_output)) =
            circuit_incremental(cconfig).unwrap();
        // Feed two input records to the circuit.
        // First input has a count of "1"
        person.push(("Bob".to_string(), Some(12), Some(true)).into(), 1);
        person.push(("Bob".to_string(), Some(40), Some(true)).into(), 1);
        // Second input has a count of "2"
        person.push(("Tom".to_string(), Some(20), Some(false)).into(), 2);
        // Append a batch of inputs to the circuit
        let mut vals = vec![
            (("Alice".to_string(), Some(25), Some(true)).into(), 1).into(),
            (("Kate".to_string(), Some(19), Some(true)).into(), 2).into(),
            (("Bob".to_string(), Some(64), Some(true)).into(), 1).into(),
        ];
        person.append(&mut vals);

        // Execute the circuit on these inputs
        circuit.step().unwrap();

        // Read the produced output
        let out = adult.consolidate();
        let num_adult_output = num_adult.consolidate();

        // Print the produced output
        println!("Number of adults:");
        num_adult_output
            .iter()
            .for_each(|(output_tuple, _, z_weight)| {
                println!("{:?} ({})", output_tuple.0, z_weight)
            });
        println!("Adults:");
        out.iter()
            .for_each(|(tup, _, z_weight)| println!("{}: {}", tup.0, z_weight));

        // Reexecute circuit with new inputs
        let mut vals = vec![
            (("Alice".to_string(), Some(25), Some(true)).into(), -1).into(),
            (("Bob".to_string(), Some(61), Some(true)).into(), 1).into(),
        ];
        person.append(&mut vals);
        // Execute the circuit on these inputs
        circuit.step().unwrap();

        // Read the updated output by merging with the prior output
        let out = adult.consolidate().merge(&out);
        let num_adult_output = num_adult.consolidate().merge(&num_adult_output);

        // Print the produced output
        println!("Number of adults:");
        num_adult_output
            .iter()
            .for_each(|(output_tuple, _, z_weight)| {
                println!("{:?} ({})", output_tuple.0, z_weight)
            });
        println!("Adults:");
        out.iter()
            .for_each(|(tup, _, z_weight)| println!("{}: {}", tup.0, z_weight));
    }
}
