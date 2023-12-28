// https://adventofcode.com/2023/day/17

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

    fn opposite(&self) -> Self {
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
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
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

    fn print_with_pos(&self, positions: &[usize]) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("\x1b[91m{}\x1b[0m", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }

    const fn pos(&self, row: usize, col: usize) -> usize {
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

fn minimal_heat_loss(grid: &Grid) -> u32 {
    0
}

fn main() {
    let stdin = io::stdin();
    let grid = Grid::build(&mut stdin.lock());

    println!("Part 1: {}", minimal_heat_loss(&grid));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    fn get_grid() -> Grid {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        Grid::build(&mut reader)
    }

    #[test]
    fn test_part1_2() {
        let grid = get_grid();
        assert_eq!(minimal_heat_loss(&grid), 102);
    }
}
