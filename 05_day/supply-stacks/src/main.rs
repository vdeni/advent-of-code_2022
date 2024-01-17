use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

fn main() {
    let file_reader = read_data_file("data/data.txt").unwrap();

    let mut cargo_composition_lines: Vec<String> = Vec::new();
    let mut cargo_operation_lines: Vec<String> = Vec::new();

    let mut processing_operations = false;
    for line in file_reader {
        /*
         * flag to be switched when iteration reaches end of cargo processing and
         * moves onto cargo operations (movements) processing
         */

        if let Ok(ln) = line {
            if ln.split_whitespace().collect::<String>() == String::from("123456789") {
                processing_operations = true;
                continue;
            }
            if ln == String::from("") {
                continue;
            }

            if processing_operations {
                cargo_operation_lines.push(ln);
            } else {
                cargo_composition_lines.push(ln);
            }
        }
    }

    let mut cargo_composition = load_cargo_composition(cargo_composition_lines);
    let mut cargo_operations = load_cargo_operations(cargo_operation_lines);

    let cargo_final_state = execute_cargo_operations(cargo_composition, cargo_operations);

    let top_cargo = get_top_cargo(cargo_final_state);
    println!("{top_cargo}");
}

fn read_data_file(filename: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open::<&str>(filename).unwrap();
    return Ok(BufReader::new(file).lines());
}

fn load_cargo_composition(cargo_lines: Vec<String>) -> HashMap<usize, Vec<String>> {
    /*!
     * Read the data file and parse out the crate stack composition
     */

    let mut cargo_map: HashMap<usize, Vec<String>> = HashMap::new();

    for line in cargo_lines.iter() {
        let cargo_row = line.split(' ');

        for (cargo_idx, cargo) in cargo_row.enumerate() {
            let cargo = cargo.replace("[", "").replace("]", "");

            if cargo == String::from("") {
                continue;
            }

            if let Some(cargo_stack) = cargo_map.get_mut(&cargo_idx) {
                cargo_stack.push(cargo);
            } else {
                cargo_map.insert(cargo_idx, vec![cargo]);
            }
        }
    }

    // reverse the vec so that the top cargo is at the end
    for (_stack_idx, stack) in &mut cargo_map {
        stack.reverse()
    }

    return cargo_map;
}

fn load_cargo_operations(operation_lines: Vec<String>) -> Vec<CargoOperation> {
    /*!
     * Read the data file and parse the order of cargo operations.
     */

    let mut cargo_operations: Vec<CargoOperation> = Vec::new();

    let parse_operations = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for operation in operation_lines {
        let operation_parsed = parse_operations.captures(operation.as_str()).unwrap();
        cargo_operations.push(CargoOperation {
            quantity: operation_parsed
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            from: operation_parsed
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            to: operation_parsed
                .get(3)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
        })
    }

    return cargo_operations;
}

fn execute_cargo_operations(
    inital_cargo_state: HashMap<usize, Vec<String>>,
    cargo_operations: Vec<CargoOperation>,
) -> HashMap<usize, Vec<String>> {
    /*!
     * Take the initial cargo state and the cargo operations, and return the
     * end state after the operations have been conducted.
     */
    let mut cargo_state = inital_cargo_state.clone();

    for operation in cargo_operations {
        let cargo_from = cargo_state.get_mut(&(operation.from - 1)).unwrap();
        let cargo_from_split = cargo_from.split_at(cargo_from.len() - operation.quantity);

        let cargo_to_keep = cargo_from_split.0.to_vec();
        let mut cargo_to_move = cargo_from_split.1.to_vec();

        cargo_from.clear();
        cargo_from.extend(cargo_to_keep);

        let cargo_to = cargo_state.get_mut(&(operation.to - 1)).unwrap();
        cargo_to_move.reverse();
        cargo_to.extend(cargo_to_move);
    }

    return cargo_state;
}

fn get_top_cargo(cargo_state: HashMap<usize, Vec<String>>) -> String {
    /*!
     * Get the codes for the top crate of each stack.
     */

    let mut top_cargo = String::from("");

    for key in 0..cargo_state.len() {
        let last_elem = cargo_state.get(&key).unwrap().last().unwrap();
        top_cargo.insert_str(top_cargo.len(), last_elem.to_owned().as_str());
    }

    return top_cargo;
}

#[derive(Debug)]
struct CargoOperation {
    quantity: usize,
    from: usize,
    to: usize,
}
