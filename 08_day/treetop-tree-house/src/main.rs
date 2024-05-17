use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Index;
use std::path::Path;

fn main() {
    let input_data = read_data(&Path::new("data/data.txt"));
    let scenic_scores = input_data.get_tree_scenic_scores();

    println!("{}", scenic_scores.iter().max().unwrap());
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

    fn determine_scenic_score(&self, tree_row: usize, tree_col: usize) -> usize {
        // return early if it's a tree in the top or bottom row
        if tree_row == 1 || tree_row == self.num_rows {
            return 0;
        }

        // return early if it's a tree in the left or right column
        if tree_col == 1 || tree_col == self.num_cols {
            return 0;
        }

        let target_tree_row = self.get_row(tree_row);

        // reversed so that trees nearer to the target are at the beginning of
        // the slice
        let mut tree_row_left: Vec<u32> = vec![];
        target_tree_row[..(tree_col - 1)].clone_into(&mut tree_row_left);
        tree_row_left.reverse();

        let mut tree_row_right: Vec<u32> = vec![];
        target_tree_row[(tree_col)..].clone_into(&mut tree_row_right);

        let target_tree_col = self.get_column(tree_col);

        let mut tree_col_up: Vec<u32> = vec![];
        target_tree_col[..(tree_row - 1)].clone_into(&mut tree_col_up);
        tree_col_up.reverse();

        let mut tree_col_down: Vec<u32> = vec![];
        target_tree_col[(tree_row)..].clone_into(&mut tree_col_down);

        let target_tree = &self[[tree_row, tree_col]];

        let mut scenic_score = 1;

        for grid_part in [tree_row_left, tree_row_right, tree_col_up, tree_col_down] {
            let mut num_trees = 0;
            for tree in &grid_part {
                num_trees += 1;

                if tree >= target_tree {
                    break;
                }
            }

            scenic_score *= num_trees;
        }

        return scenic_score;
    }

    fn get_tree_scenic_scores(&self) -> Vec<usize> {
        let mut scenic_scores: Vec<usize> = vec![];
        for i in 1..=self.num_rows {
            for j in 1..=self.num_cols {
                scenic_scores.push(self.determine_scenic_score(i, j));
            }
        }

        return scenic_scores;
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = u32;

    fn index(&self, ij: [usize; 2]) -> &Self::Output {
        return &self.data[ij[0] - 1][ij[1] - 1];
    }
}
