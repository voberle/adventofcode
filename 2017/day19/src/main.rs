use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

const ALL_DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

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
            Up => pos < self.cols,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
            Right => pos % self.cols == self.cols - 1,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `allowed`.
    pub fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Down => pos + self.cols,
            Left => pos - 1,
            Right => pos + 1,
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

fn find_start(grid: &Grid) -> usize {
    grid.values.iter().position(|c| *c == '|').unwrap()
}

fn seen_letters(grid: &Grid) -> String {
    let mut letters = String::new();

    let mut pos = find_start(grid);
    let mut dir = Direction::Down;
    while let Some(next_pos) = grid.try_next_pos(pos, dir) {
        pos = next_pos;
        let c = grid.values[pos];
        match c {
            '|' | '-' => {
                // Direction doesn't change
            }
            '+' => {
                // Need to look at next char
                match dir {
                    Up | Down => {
                        if let Some(n_pos) = grid.try_next_pos(pos, Left) {
                            // anything except space should be ok
                            if grid.values[n_pos] != ' ' {
                                dir = Left;
                            }
                        }
                        if let Some(n_pos) = grid.try_next_pos(pos, Right) {
                            if grid.values[n_pos] != ' ' {
                                assert_ne!(dir, Left);
                                dir = Right;
                            }
                        }
                    }
                    Left | Right => {
                        if let Some(n_pos) = grid.try_next_pos(pos, Up) {
                            if grid.values[n_pos] != ' ' {
                                dir = Up;
                            }
                        }
                        if let Some(n_pos) = grid.try_next_pos(pos, Down) {
                            if grid.values[n_pos] != ' ' {
                                assert_ne!(dir, Up);
                                dir = Down;
                            }
                        }
                    }
                }
            }
            'A'..='Z' => {
                // Direction doesn't change, just store letter
                letters.push(c);
            }
            ' ' => {
                // If there is no bug, it means we got to the end
                break;
            }
            _ => panic!("Invalid char in grid {}", c),
        }
        // println!("Next dir {:?} pos {}", dir, pos);
        // grid.print_with_pos(&vec![pos]);
    }

    letters
}

fn part2(grid: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);
    // grid.print();

    println!("Part 1: {}", seen_letters(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(seen_letters(&Grid::build(INPUT_TEST)), "ABCDEF");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
