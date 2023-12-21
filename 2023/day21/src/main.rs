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

impl Direction {
    fn index(&self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }
}

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

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
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
    assert_eq!(grid.row(5), 1);
    assert_eq!(grid.col(5), 2);
    assert_eq!(grid.row(1), 0);
    assert_eq!(grid.col(1), 1);

    assert!(grid.allowed(5, North));
    assert_eq!(grid.next_pos(5, North), 2);
    assert!(grid.allowed(5, West));
    assert_eq!(grid.next_pos(5, West), 4);
    assert!(!grid.allowed(5, East));
    assert!(!grid.allowed(5, South));
}

fn walk_one_step(grid: &mut Grid) {
    grid.values
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == 'O' { Some(i) } else { None })
        .collect::<Vec<usize>>()
        .iter_mut()
        .for_each(|i| {
            for d in ALL_DIRECTIONS {
                if let Some(n) = grid.try_next_pos(*i, d) {
                    if grid.values[n] == '.' {
                        grid.values[n] = 'O';
                    }
                }
            }
            grid.values[*i] = '.';
        });
}

#[test]
fn test_walk_one_step() {
    let s = "...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let r = "...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.O.####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........";
    let mut grid = Grid::build(&mut s.as_bytes());
    walk_one_step(&mut grid);
    assert_eq!(grid, Grid::build(&mut r.as_bytes()));
}

fn get_initial_pos(grid: &Grid) -> Option<usize> {
    grid.values.iter().position(|v| *v == 'S')
}

fn prep_grid(grid: &mut Grid, pos: usize) {
    grid.values[pos] = 'O';
}

fn plots_count(grid: &Grid) -> usize {
    grid.values.iter().filter(|v| **v == 'O').count()
}

fn find_filled_grid(grid: &mut Grid, target_step_count: u64) -> (u64, u64, u64) {
    let mut counts: Vec<usize> = Vec::new();

    let mut steps: u64 = 0;
    loop {
        if steps == target_step_count {
            break;
        }
        steps += 1;

        walk_one_step(grid);

        let plot_count = plots_count(grid);
        let maybe_second_last = counts.len().checked_sub(2).map(|i| counts[i]);
        counts.push(plot_count);

        if let Some(last) = maybe_second_last {
            // println!("------- Last {}, curr {}", last, plot_count);
            if plot_count == last {
                println!("Found period after {} steps", steps + 1);
                break;
            }
        }
        grid.print();
    }
    // println!("{:?}", saved_counts);
    let plot_count = *counts.last().unwrap();
    let other_count = counts.len().checked_sub(2).map(|i| counts[i]).unwrap();

    (steps, plot_count as u64, other_count as u64)
}

fn garden_plots_count_after(grid: &Grid, target_step_count: u64) -> u64 {
    let initial_pos = get_initial_pos(grid).unwrap();
    let mut initial = grid.clone();
    prep_grid(&mut initial, initial_pos);

    let (mut step_count, mut plot_count, mut other_count) =
        find_filled_grid(&mut initial, target_step_count);

    loop {
        if step_count == target_step_count {
            break;
        }
        step_count += 1;
        std::mem::swap(&mut plot_count, &mut other_count);
    }
    plot_count
}

const STEPS_COUNT_TEST: u64 = 6;
const STEPS_COUNT_PART1: u64 = 64;

fn main() {
    let stdin = io::stdin();

    let grid = Grid::build(&mut stdin.lock());
    // grid.print();

    println!(
        "Part 1: {}",
        garden_plots_count_after(&grid, STEPS_COUNT_PART1)
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let grid = Grid::build(&mut reader);

        assert_eq!(garden_plots_count_after(&grid, STEPS_COUNT_TEST), 16);
        assert_eq!(garden_plots_count_after(&grid, STEPS_COUNT_PART1), 42);
    }
}
