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
    let mut char_counter: usize = 0;
    let stream_as_chars: Vec<char> = stream.chars().collect();
    loop {
        let candidate = &stream_as_chars[char_counter..(char_counter + 14)];

        let candidate_uniques: HashSet<char> = HashSet::from_iter(candidate.iter().cloned());

        if candidate_uniques.len() == 14 {
            println!(
                "Found message marker: {candidate_uniques:?}, after character {char_cnt}",
                char_cnt = char_counter + 14
            );
            break;
        } else {
            char_counter += 1;
        }
    }
}
