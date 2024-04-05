use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().map(|c| c == '#').collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn is_tree(&self, row: usize, col: usize) -> bool {
        self.values[row * self.cols + col]
    }
}

fn tree_count(grid: &Grid, right: usize, down: usize) -> usize {
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut trees = 0;

    while row < grid.rows {
        if grid.is_tree(row, col) {
            trees += 1;
        }
        // Moving
        col = (col + right).rem_euclid(grid.cols);
        row += down;
    }
    trees
}

fn tree_count_simple(grid: &Grid) -> usize {
    tree_count(grid, 3, 1)
}

fn tree_count_complex(grid: &Grid) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| tree_count(grid, *right, *down))
        .product()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", tree_count_simple(&map));
    println!("Part 2: {}", tree_count_complex(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(tree_count_simple(&Grid::build(INPUT_TEST)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(tree_count_complex(&Grid::build(INPUT_TEST)), 336);
    }
}
