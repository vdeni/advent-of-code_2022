use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let data_filepath = Path::new("data/data.txt");

    let data = read_data(data_filepath);

    find_marker(data);
}

fn read_data(path: &Path) -> String {
    return read_to_string(path).unwrap();
}

fn find_marker(stream: String) -> () {
    let mut marker_tracker: HashSet<char> = HashSet::new();

    for (idx, char) in stream.chars().enumerate() {
        if marker_tracker.contains(&char) {
            println!("Found: {char}. Clearing marker_tracker.");
            marker_tracker.clear();
            marker_tracker.insert(char);
        } else {
            marker_tracker.insert(char);
            println!("inserted {char}");

            if marker_tracker.len() == 4 {
                println!("Marker found at character number {}.", idx + 1);
                break;
            }
        }
    }
}
