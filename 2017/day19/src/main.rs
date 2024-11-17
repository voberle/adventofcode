use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

struct Grid {
    values: Vec<char>,
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
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    // Check we don't go outside grid.
    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            Up => pos < self.cols,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
            Right => pos % self.cols == self.cols - 1,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `allowed`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Down => pos + self.cols,
            Left => pos - 1,
            Right => pos + 1,
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

fn find_start(grid: &Grid) -> usize {
    grid.values.iter().position(|c| *c == '|').unwrap()
}

fn walk(grid: &Grid) -> (String, usize) {
    let mut letters = String::new();
    let mut steps = 0;

    let mut pos = find_start(grid);
    let mut dir = Direction::Down;
    while let Some(next_pos) = grid.try_next_pos(pos, dir) {
        steps += 1;
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
            _ => panic!("Invalid char in grid {c}"),
        }
    }

    (letters, steps)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    let (seen_letters, steps) = walk(&grid);

    println!("Part 1: {seen_letters}");
    println!("Part 2: {steps}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1_2() {
        let (seen_letters, steps) = walk(&Grid::build(INPUT_TEST));
        assert_eq!(seen_letters, "ABCDEF");
        assert_eq!(steps, 38);
    }
}
