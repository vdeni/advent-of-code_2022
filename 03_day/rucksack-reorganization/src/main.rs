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

    let mut wrongly_sorted_items: Vec<char> = Vec::new();
    for inventory in rucksacks {
        if let Ok(inventory) = inventory {
            wrongly_sorted_items.append(&mut find_wrongly_sorted_items(&inventory));
        }
    }

    let total_priority = get_total_item_priority(wrongly_sorted_items);
    println!("Total item priority is {total_priority}.");
}

fn read_inventory<P>(file: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    /*!
     * Reads the invenrotry data from the TXT file.
     */

    let file_conn = File::open(file).unwrap();
    return BufReader::new(file_conn).lines();
}

fn find_wrongly_sorted_items(rucksack: &String) -> Vec<char> {
    /*!
     * Find items that appear in both containers of a given rucksack.
     */

    let num_items = rucksack.len();

    let (container_1, container_2) = rucksack.split_at(num_items / 2);

    let mut c1_items_in_c2: Vec<char> = container_1
        .chars()
        .filter(|item| return container_2.contains(*item))
        .collect();
    c1_items_in_c2.dedup();

    return c1_items_in_c2;
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
