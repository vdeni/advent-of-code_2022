use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(lines) = read_lines("data/data.txt") {
        let all_items: Vec<String> = lines.map(|x| x.unwrap()).collect();

        let mut elf_calories: HashMap<u8, u32> = HashMap::new();

        let mut elf_counter: u8 = 1;

        for entry in all_items {
            if entry == "" {
                elf_counter += 1;
                continue;
            }

            let calories = entry.parse::<u32>().unwrap();

            if elf_calories.contains_key(&elf_counter) {
                *elf_calories.get_mut(&elf_counter).unwrap() += calories;
            } else {
                elf_calories.insert(elf_counter, calories);
            }
        }

        let mut elf_max_cal_id = 0;
        let mut elf_max_cal_amt = 0;
        for elf_cals in elf_calories {
            if elf_cals.1 > elf_max_cal_amt {
                elf_max_cal_amt = elf_cals.1;
                elf_max_cal_id = elf_cals.0;
            }
        }

        println!(
        "The elf carrying the most calories is {elf_max_cal_id}, with {elf_max_cal_amt} calories."
            );
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open::<&str>(filename).unwrap();
    return Ok(io::BufReader::new(file).lines());
}
