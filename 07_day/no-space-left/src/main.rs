use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

fn main() {
    let data_path = Path::new("data/data.txt");

    let data_lines = read_input(data_path);

    let lines_parsed: Vec<LineType> = data_lines.map(|ln| parse_line(ln.unwrap())).collect();

    let file_tree = populate_file_tree(lines_parsed);

    let dir_sizes = calculate_total_directory_sizes(file_tree);

    let root_dir_size = dir_sizes.get("/").unwrap();

    let unused_space = TOTAL_SPACE - root_dir_size;

    let space_needed = NEEDED_SPACE - unused_space;

    let smallest_dir = dir_sizes
        .iter()
        .filter(|dir| dir.1 >= &space_needed)
        .min_by_key(|dir| dir.1);

    println!("{smallest_dir:?}");
}

fn read_input(file_path: &Path) -> io::Lines<BufReader<File>> {
    let data_file = File::open(file_path).unwrap();
    return BufReader::new(data_file).lines();
}

fn parse_line(cli_line: String) -> LineType {
    /*!
     * Parses the input line and returns a structured object representing the
     * line's contents. May be a command or command output.
     */

    // determine whether the output line is a command or command output
    let line_elems: Vec<String> = cli_line.split_whitespace().map(|x| x.to_string()).collect();

    let line_elem_parsed: LineType = match line_elems[0].as_str() {
        "$" => match line_elems[1].as_str() {
            "cd" => LineType::CMD(Command::CD(line_elems[2].clone())),
            "ls" => LineType::CMD(Command::LS),
            _ => panic!("Unknown CMD provided in input: {}", line_elems[0].as_str()),
        },
        "dir" => LineType::OUT(CommandOut::DIR(FSDir {
            name: line_elems[1].clone(),
        })),
        _ => LineType::OUT(CommandOut::FIILE(FSFile {
            name: line_elems[1].clone(),
            size: line_elems[0].parse::<usize>().unwrap(),
        })),
    };

    return line_elem_parsed;
}

fn populate_file_tree(cli_lines: Vec<LineType>) -> HashMap<String, FSTree> {
    let mut file_tree: HashMap<String, FSTree> = HashMap::new();
    file_tree.insert(
        String::from("/"),
        FSTree {
            children: HashMap::new(),
            fs_elem: FSDir {
                name: String::from("/"),
            },
        },
    );

    let mut current_path: Vec<String> = Vec::new();
    // current_path.push(String::from("/"));

    let mut current_dir = file_tree.get_mut("/").unwrap();

    for line in cli_lines {
        match line {
            LineType::CMD(cmd) => match cmd {
                Command::CD(dir) => {
                    if dir == String::from("..") {
                        current_path.pop();
                        current_dir = file_tree.get_mut(&current_path.join("/")).unwrap();
                    } else {
                        current_path.push(dir.clone());
                        file_tree.insert(
                            current_path.join("/").clone(),
                            FSTree {
                                children: HashMap::new(),
                                fs_elem: FSDir { name: dir.clone() },
                            },
                        );
                        current_dir = file_tree.get_mut(&current_path.join("/")).unwrap();
                    }
                }
                Command::LS => {
                    continue;
                }
            },
            LineType::OUT(out) => match out {
                CommandOut::DIR(dir) => {
                    current_dir
                        .children
                        .insert(dir.name.clone(), FSElem::Dir(dir));
                }
                CommandOut::FIILE(file) => {
                    current_dir
                        .children
                        .insert(file.name.clone(), FSElem::File(file));
                }
            },
        }
    }

    return file_tree;
}

fn calculate_directory_size(
    directory_name: String,
    directory: &FSTree,
    fs_map: &HashMap<String, FSTree>,
) -> usize {
    let mut dir_size = 0;

    for (_fs_elem_name, fs_elem) in &directory.children {
        match fs_elem {
            FSElem::File(file) => {
                dir_size += file.size;
            }
            FSElem::Dir(dir) => {
                let mut target_path = directory_name.clone();
                target_path.push_str("/");
                target_path.push_str(&dir.name);

                let target_dir = fs_map.get(&target_path).unwrap();
                let target_dir_size = calculate_directory_size(target_path, target_dir, fs_map);
                dir_size += target_dir_size;
            }
        }
    }

    return dir_size;
}

fn calculate_total_directory_sizes(file_tree: HashMap<String, FSTree>) -> HashMap<String, usize> {
    /*!
     * Calculate the total size for each directory.
     */

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for (dir_name, dir) in &file_tree {
        let dirsize = calculate_directory_size(dir_name.clone(), dir, &file_tree);
        dir_sizes.insert(dir_name.clone(), dirsize);
    }

    return dir_sizes;
}

#[derive(Debug)]
struct FSFile {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct FSDir {
    name: String,
}

#[derive(Debug)]
enum FSElem {
    File(FSFile),
    Dir(FSDir),
}

#[derive(Debug)]
enum Command {
    LS,
    CD(String),
}

#[derive(Debug)]
enum CommandOut {
    DIR(FSDir),
    FIILE(FSFile),
}

#[derive(Debug)]
enum LineType {
    CMD(Command),
    OUT(CommandOut),
}

#[derive(Debug)]
struct FSTree {
    children: HashMap<String, FSElem>,
    fs_elem: FSDir,
}
