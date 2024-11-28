use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let data_path = Path::new("data/data.txt");
    let instructions = read_data(data_path);

    let mut rope = Rope {
        head_position: Position(0, 0),
        tail_position: Position(0, 0),
        visited_tiles: vec![VisitedBridgeTile(0, 0)],
    };

    for instr in instructions {
        println!("Move: {instr:?}");
        rope = execute_movement(rope, instr);
    }

    let mut visited_tiles = rope.visited_tiles;
    visited_tiles.sort();
    visited_tiles.dedup();

    let num_visited_tiles = visited_tiles.len();

    println!("Visited {num_visited_tiles} tiles.");
}

fn read_data(file_path: &Path) -> Vec<Movement> {
    let data_reader = get_data_reader(file_path);

    let mut movements = vec![];

    for line in data_reader {
        let line_parsed = parse_data(line.unwrap());
        movements.push(line_parsed)
    }

    return movements;
}

fn get_data_reader(file_path: &Path) -> Lines<BufReader<File>> {
    let data_file = File::open(file_path).unwrap();

    return BufReader::new(data_file).lines();
}

fn parse_data(instruction: String) -> Movement {
    let element = instruction.split_once(' ').unwrap();

    let instruction_parsed = match element.0 {
        "R" => Movement::R(element.1.parse::<isize>().unwrap()),
        "L" => Movement::L(element.1.parse::<isize>().unwrap()),
        "U" => Movement::U(element.1.parse::<isize>().unwrap()),
        "D" => Movement::D(element.1.parse::<isize>().unwrap()),
        _ => panic!("Unknown movement received in instructions."),
    };

    return instruction_parsed;
}

/**
 * Executes a Movement on the Rope, modyfing the Rope's current position
 * and returning a new Rope.
 */
fn execute_movement(rope: Rope, movement: Movement) -> Rope {
    let mut moved_rope = rope;

    match movement {
        Movement::D(steps) => moved_rope.head_position.0 -= steps,
        Movement::U(steps) => moved_rope.head_position.0 += steps,
        Movement::L(steps) => moved_rope.head_position.1 -= steps,
        Movement::R(steps) => moved_rope.head_position.1 += steps,
    }

    let mut head_tail_row_distance = moved_rope.head_position.0 - moved_rope.tail_position.0;

    let mut head_tail_column_distance = moved_rope.head_position.1 - moved_rope.tail_position.1;

    println!("row distance is: {head_tail_row_distance}");
    println!("col distance is: {head_tail_column_distance}");

    // TODO: biljezenje posjecenih ploca

    while (head_tail_row_distance.abs() > 1) | (head_tail_column_distance.abs() > 1) {
        println!("=== iteration start ===");

        if (head_tail_row_distance.abs() > 1) & (head_tail_column_distance == 0) {
            println!("moving rows");

            let tiles_to_move = head_tail_row_distance - (head_tail_row_distance.signum() * 1);

            let mut tile_range: Vec<isize> = if tiles_to_move.signum() == 1 {
                (1..=tiles_to_move).collect()
            } else {
                (tiles_to_move..=-1).collect()
            };

            if tiles_to_move.signum() == -1 {
                tile_range.reverse();
            }

            for step in tile_range {
                println!("{step}");
                moved_rope.visited_tiles.push(VisitedBridgeTile(
                    moved_rope.tail_position.0 + step,
                    moved_rope.tail_position.1,
                ));
            }

            let last_visited = moved_rope.visited_tiles.last().unwrap().to_owned();

            moved_rope.tail_position.0 = last_visited.0;
            moved_rope.tail_position.1 = last_visited.1;
        } else if (head_tail_row_distance == 0) & (head_tail_column_distance.abs() > 1) {
            println!("moving cols");

            let tiles_to_move =
                head_tail_column_distance - (head_tail_column_distance.signum() * 1);

            println!("tiles to move: {tiles_to_move}");

            let mut tile_range: Vec<isize> = if tiles_to_move.signum() == 1 {
                (1..=tiles_to_move).collect()
            } else {
                (tiles_to_move..=-1).collect()
            };

            if tiles_to_move.signum() == -1 {
                tile_range.reverse();
            }

            println!("tile range: {tile_range:?}");

            for step in tile_range {
                println!("{step}");
                moved_rope.visited_tiles.push(VisitedBridgeTile(
                    moved_rope.tail_position.0,
                    moved_rope.tail_position.1 + step,
                ));
            }

            let last_visited = moved_rope.visited_tiles.last().unwrap().to_owned();

            moved_rope.tail_position.0 = last_visited.0;
            moved_rope.tail_position.1 = last_visited.1;
        } else if (head_tail_column_distance.abs() > 1) | (head_tail_row_distance.abs() > 1) {
            println!("moving diagonally");

            moved_rope.visited_tiles.push(VisitedBridgeTile(
                moved_rope.tail_position.0 + head_tail_row_distance.signum() * 1,
                moved_rope.tail_position.1 + head_tail_column_distance.signum() * 1,
            ));

            let last_visited = moved_rope.visited_tiles.last().unwrap().to_owned();

            moved_rope.tail_position.0 = last_visited.0;
            moved_rope.tail_position.1 = last_visited.1;
        }

        println!("recalculating distances");

        head_tail_row_distance = moved_rope.head_position.0 - moved_rope.tail_position.0;

        head_tail_column_distance = moved_rope.head_position.1 - moved_rope.tail_position.1;

        println!("row distance is: {head_tail_row_distance}");
        println!("col distance is: {head_tail_column_distance}");

        println!("=== iteration end ===");
    }

    println!("State after move: {moved_rope:?}");

    return moved_rope;
}

#[derive(Debug, Clone)]
struct Rope {
    head_position: Position,
    tail_position: Position,
    visited_tiles: Vec<VisitedBridgeTile>,
}

#[derive(Debug)]
enum Movement {
    R(isize),
    L(isize),
    U(isize),
    D(isize),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord)]
struct VisitedBridgeTile(isize, isize);

#[derive(Debug, Clone)]
struct Position(isize, isize);
