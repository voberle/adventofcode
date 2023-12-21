// https://adventofcode.com/2023/day/21

use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build<R>(reader: &mut R) -> Self
    where
        R: BufRead,
    {
        let mut rows = 0;
        let values: Vec<_> = reader
            .lines()
            .filter_map(|result| result.ok())
            .map(|l| {
                rows += 1;
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print(&self) {
        for row in 0..self.rows {
            println!(
                "{}",
                self.values[row * self.cols..(row + 1) * self.cols]
                    .iter()
                    .collect::<String>()
            );
        }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    // Check we don't go outside grid.
    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `can_go`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn try_next_pos(&self, pos: usize, direction: Direction) -> Option<usize> {
        if self.allowed(pos, direction) {
            Some(self.next_pos(pos, direction))
        } else {
            None
        }
    }
}

#[test]
fn test_grid() {
    let input = "123\n456";
    let grid = Grid::build(&mut input.as_bytes());
    assert_eq!(grid.cols, 3);
    assert_eq!(grid.rows, 2);
    assert_eq!(grid.pos(0, 1), 1);
    assert_eq!(grid.pos(1, 2), 5);

    assert!(grid.allowed(5, North));
    assert_eq!(grid.next_pos(5, North), 2);
    assert!(grid.allowed(5, West));
    assert_eq!(grid.next_pos(5, West), 4);
    assert!(!grid.allowed(5, East));
    assert!(!grid.allowed(5, South));
}

fn walk_one_step(before: &Grid, after: &mut Grid) {
    for i in 0..before.values.len() {
        if before.values[i] != 'O' {
            continue;
        }
        for d in ALL_DIRECTIONS {
            if let Some(n) = before.try_next_pos(i, d) {
                if before.values[n] == '.' {
                    after.values[n] = 'O';
                }
            }
        }
    }
}

fn get_initial_pos(grid: &Grid) -> Option<usize> {
    grid.values.iter().position(|v| *v == 'S')
}

fn initial_step(grid: &mut Grid, pos: usize) {
    grid.values[pos] = 'O';
}

fn clean_grid(grid: &mut Grid, pos: usize) {
    grid.values[pos] = '.';
}

fn garden_plots_count(grid: &Grid, target_step_count: u32) -> usize {
    let initial_pos = get_initial_pos(grid).unwrap();

    let mut before = grid.clone();
    initial_step(&mut before, initial_pos);

    for _ in 0..target_step_count {
        let mut after = grid.clone();
        clean_grid(&mut after, initial_pos);

        walk_one_step(&before, &mut after);

        std::mem::swap(&mut before, &mut after);
    }

    before.print();
    before.values.iter().filter(|v| **v == 'O').count()
}

// const STEPS_COUNT_TEST: u32 = 6;
const STEPS_COUNT_TEST: u32 = 64;

fn main() {
    let stdin = io::stdin();

    let grid = Grid::build(&mut stdin.lock());
    grid.print();

    println!("Part 1: {}", garden_plots_count(&grid, STEPS_COUNT_TEST));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let grid = Grid::build(&mut reader);

        assert_eq!(garden_plots_count(&grid, STEPS_COUNT_TEST), 16);
    }
}
