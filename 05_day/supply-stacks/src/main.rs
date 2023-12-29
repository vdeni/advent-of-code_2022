use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

fn main() {
    let file_reader = read_data_file("data/data.txt").unwrap();

    let mut cargo_composition_lines: Vec<String> = Vec::new();

    for line in file_reader {
        if let Ok(cargo) = line {
            /*
             * end iteration delimiter between cargo composition and operation
             * order was reached
             */
            if cargo.split_whitespace().collect::<String>() == String::from("123") {
                break;
            }

            cargo_composition_lines.push(cargo);
        }
    }

    let mut cargo_composition = load_cargo_composition(cargo_composition_lines);
    println!("{cargo_composition:?}");
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

    return cargo_map;
}

fn load_cargo_operations() {
    /*!
     * Read the data file and parse the order of cargo operations.
     */
}
