use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

impl Direction {
    pub fn index(self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub values: Vec<char>,
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    pub fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("{RED}{}{RESET}", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }

    pub fn print(&self) {
        self.print_with_pos(&[]);
    }

    pub fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    pub fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    // Check we don't go outside grid.
    pub fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `allowed`.
    pub fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    pub fn try_next_pos(&self, pos: usize, direction: Direction) -> Option<usize> {
        if self.allowed(pos, direction) {
            Some(self.next_pos(pos, direction))
        } else {
            None
        }
    }
}

fn part1(grid: &Grid) -> String {
    "".to_string()
}

fn part2(grid: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);
    grid.print();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&Grid::build(INPUT_TEST)), "ABCDEF");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
