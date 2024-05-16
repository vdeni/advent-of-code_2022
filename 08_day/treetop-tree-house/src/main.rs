use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Index;
use std::path::Path;

fn main() {
    let input_data = read_data(&Path::new("data/data.txt"));
    let visibilities = input_data.get_tree_visibilities();

    println!(
        "{}",
        visibilities.iter().map(|eval| *eval as u32).sum::<u32>()
    );
}

fn read_data(file_path: &Path) -> Matrix {
    let data_reader = get_data_reader(file_path);

    let mut input_data = vec![];

    for line in data_reader {
        let line_parsed: Vec<u32> = line
            .unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        input_data.push(line_parsed)
    }

    let data_matrix = Matrix::from(input_data);

    return data_matrix;
}

fn get_data_reader(file_path: &Path) -> Lines<BufReader<File>> {
    let data_file = File::open(file_path).unwrap();

    return BufReader::new(data_file).lines();
}

#[derive(Debug)]
struct Matrix {
    num_rows: usize,
    num_cols: usize,
    data: Vec<Vec<u32>>,
}

impl Matrix {
    fn from(vec_of_vecs: Vec<Vec<u32>>) -> Self {
        let col_lengths: Vec<usize> = vec_of_vecs.iter().map(|col_vec| col_vec.len()).collect();

        if col_lengths.iter().min().unwrap() != col_lengths.iter().max().unwrap() {
            panic!("Supplied rows don't have the same number of columns!")
        }

        return Matrix {
            num_rows: vec_of_vecs.len(),
            num_cols: col_lengths[0],
            data: vec_of_vecs,
        };
    }

    fn get_column(&self, idx: usize) -> Vec<u32> {
        let column: Vec<u32> = self.data.iter().map(|row| row[idx - 1]).collect();
        return column;
    }

    fn get_row(&self, idx: usize) -> Vec<u32> {
        let row = self.data[idx - 1].clone();
        return row;
    }

    fn determine_tree_visibility(&self, tree_row: usize, tree_col: usize) -> bool {
        // return early if it's a tree in the top or bottom row
        if tree_row == 1 || tree_row == self.num_rows {
            return true;
        }

        // return early if it's a tree in the left or right column
        if tree_col == 1 || tree_col == self.num_cols {
            return true;
        }

        let target_tree_row = self.get_row(tree_row);
        let tree_row_left = &target_tree_row[..(tree_col - 1)];
        let tree_row_right = &target_tree_row[(tree_col)..];

        let target_tree_col = self.get_column(tree_col);
        let tree_col_up = &target_tree_col[..(tree_row - 1)];
        let tree_col_down = &target_tree_col[(tree_row)..];

        let target_tree = &self[[tree_row, tree_col]];

        for grid_part in [tree_row_left, tree_row_right, tree_col_up, tree_col_down] {
            if grid_part.iter().max().unwrap() < target_tree {
                return true;
            }
        }

        return false;
    }

    fn get_tree_visibilities(&self) -> Vec<bool> {
        let mut visibilities: Vec<bool> = vec![];
        for i in 1..=self.num_rows {
            for j in 1..=self.num_cols {
                visibilities.push(self.determine_tree_visibility(i, j));
            }
        }

        return visibilities;
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = u32;

    fn index(&self, ij: [usize; 2]) -> &Self::Output {
        return &self.data[ij[0] - 1][ij[1] - 1];
    }
}
