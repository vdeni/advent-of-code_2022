use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(all_items) = read_lines("data/data.txt") {
        // find the elf carrying the most calories
        let mut elf_calories: HashMap<u8, u32> = HashMap::new();
        let mut elf_counter: u8 = 1;

        for entry in all_items {
            let entry_contents = entry.unwrap();
            if entry_contents == "" {
                elf_counter += 1;
                continue;
            }

            let calories = entry_contents.parse::<u32>().unwrap();

            if elf_calories.contains_key(&elf_counter) {
                *elf_calories.get_mut(&elf_counter).unwrap() += calories;
            } else {
                elf_calories.insert(elf_counter, calories);
            }
        }

        let mut elf_max_cal_id = 0;
        let mut elf_max_cal_amt = 0;
        for elf_cals in &elf_calories {
            if *elf_cals.1 > elf_max_cal_amt {
                elf_max_cal_amt = *elf_cals.1;
                elf_max_cal_id = *elf_cals.0;
            }
        }

        println!(
        "The elf carrying the most calories is {elf_max_cal_id}, with {elf_max_cal_amt} calories."
            );

        // find the total amount of calories carried by the 3 elves that have the most
        let mut all_calories: Vec<&u32> = elf_calories.values().collect();
        all_calories.sort();
        println!(
            "The three elves carrying the most calories carry {} calories in total.",
            all_calories[all_calories.len() - 3..all_calories.len()]
                .iter()
                .map(|x| **x)
                .sum::<u32>()
        );
    }
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open::<&str>(filename).unwrap();
    return Ok(io::BufReader::new(file).lines());
}
