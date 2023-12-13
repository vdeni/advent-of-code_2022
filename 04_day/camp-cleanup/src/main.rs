use csv;
use std::collections::hash_set::HashSet;
use std::fs::File;
use std::ops::RangeInclusive;
use std::path::Path;

fn main() {
    let mut sector_duties = get_file_reader("data/data.csv");

    let mut range_evaluations: Vec<bool> = Vec::new();
    for elf_pair in sector_duties.records() {
        if let Ok(sectors) = elf_pair {
            let elf_sectors = parse_record(sectors);
            let is_fully_contained = are_ranges_overlapping(elf_sectors);
            range_evaluations.push(is_fully_contained);
        }
    }

    let count_fully_contained: usize = range_evaluations
        .iter()
        .map(|contained_eval| *contained_eval as usize)
        .sum();

    println!("The number of records where one range is fully contained in another is {count_fully_contained}.");
}

fn get_file_reader<P>(file: P) -> csv::Reader<File>
where
    P: AsRef<Path>,
{
    /*!
     * Read in the data file, returning a Reader object.
     */

    let csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)
        .unwrap();

    return csv_reader;
}

fn parse_record(record: csv::StringRecord) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    /*!
     * Takes each record from the CSV file, parses both fields and returns a
     * tuple of range objects.
     */
    let sectors_elf_1: Vec<String> = record
        .get(0)
        .unwrap()
        .split('-')
        .map(|part| part.to_string())
        .collect();
    let range_elf_1 = if let (Some(bound_lower), Some(bound_upper)) =
        (sectors_elf_1.get(0), sectors_elf_1.get(1))
    {
        let bound_lower = bound_lower.parse::<usize>().unwrap();
        let bound_upper = bound_upper.parse::<usize>().unwrap();
        bound_lower..=bound_upper
    } else {
        panic!("Invalid range listed for elf 1.");
    };

    let sectors_elf_2: Vec<String> = record
        .get(1)
        .unwrap()
        .split('-')
        .map(|part| part.to_string())
        .collect();
    let range_elf_2 = if let (Some(bound_lower), Some(bound_upper)) =
        (sectors_elf_2.get(0), sectors_elf_2.get(1))
    {
        let bound_lower = bound_lower.parse::<usize>().unwrap();
        let bound_upper = bound_upper.parse::<usize>().unwrap();
        bound_lower..=bound_upper
    } else {
        panic!("Invalid range listed for elf 2.");
    };

    return (range_elf_1, range_elf_2);
}

fn are_ranges_overlapping(range_pair: (RangeInclusive<usize>, RangeInclusive<usize>)) -> bool {
    /*!
     * Determines whether the two ranges overlap or not (irrespective of
     * whether the overlap is complete or not).
     */

    let elems_range_1: HashSet<usize> = range_pair.0.collect();
    let elems_range_2: HashSet<usize> = range_pair.1.collect();

    if elems_range_1.intersection(&elems_range_2).count() == 0 {
        return false;
    } else {
        return true;
    }
}
