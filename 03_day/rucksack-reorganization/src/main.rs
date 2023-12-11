use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const ALPHABET_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ALPHABET_UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    let current_file = Path::new(file!());
    let data_filepath = current_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("data.txt");

    let rucksacks = read_inventory(data_filepath);

    let mut group_badges: Vec<char> = Vec::new();
    let mut elf_group: Vec<HashSet<char>> = Vec::new();
    for (idx, inventory) in rucksacks.enumerate() {
        if let Ok(inventory) = inventory {
            let elf_unique_items: HashSet<char> = inventory.chars().collect();
            elf_group.push(elf_unique_items);
        }

        if (idx + 1) % 3 == 0 {
            let intersection_1_2: HashSet<char> = elf_group[0]
                .intersection(&elf_group[1])
                .map(|x| *x)
                .collect();

            let intersection_all: Vec<char> = intersection_1_2
                .intersection(&elf_group[2])
                .map(|x| *x)
                .collect();

            if intersection_all.len() == 1 {
                group_badges.push(intersection_all[0]);
            } else {
                panic!("Unable to find unique group identifier!")
            }

            elf_group = Vec::new();
        }
    }

    let total_priority = get_total_item_priority(group_badges);
    println!("Total badge priority is {total_priority}.");
}

fn read_inventory<P>(file: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    /*!
     * Reads the inventory data from the TXT file.
     */

    let file_conn = File::open(file).unwrap();
    return BufReader::new(file_conn).lines();
}

fn get_total_item_priority(wrongly_sorted_items: Vec<char>) -> usize {
    /*!
     * Finds the sum of priorities of all wrongly sorted items found in the
     * data.
     */

    let total_priority: usize = wrongly_sorted_items
        .iter()
        .map(|item| {
            if let Ok(idx) = ALPHABET_LOWERCASE.binary_search(&item) {
                return idx + 1;
            }

            if let Ok(idx) = ALPHABET_UPPERCASE.binary_search(&item) {
                return ALPHABET_UPPERCASE.len() + 1 + idx;
            }

            return 0;
        })
        .sum();

    return total_priority;
}
