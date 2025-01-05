use std::io::{self, Read};

use itertools::{Either, Itertools};

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

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn is_lock(&self) -> bool {
        self.values[0..self.cols] == vec!['#'; self.cols]
    }

    // The first parameter indicates if it's a lock.
    fn get_heights(&self) -> (bool, Vec<usize>) {
        let is_lock = self.is_lock();
        let heights = (0..self.cols)
            .map(|c| {
                if is_lock {
                    (0..self.rows)
                        .take_while(|&r| self.values[self.pos(r, c)] == '#')
                        .count()
                        - 1
                } else {
                    (0..self.rows)
                        .rev()
                        .take_while(|&r| self.values[self.pos(r, c)] == '#')
                        .count()
                        - 1
                }
            })
            .collect();
        (is_lock, heights)
    }
}

fn build(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(Grid::build)
        .collect()
}

// Convert the schematics into heights, locks and keys.
fn schematics_to_heights(schematics: &[Grid]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    schematics
        .iter()
        .map(Grid::get_heights)
        .partition_map(|(is_lock, heights)| {
            if is_lock {
                Either::Left(heights)
            } else {
                Either::Right(heights)
            }
        })
}

#[allow(dead_code)]
fn print_heights(lock_heights: &[Vec<usize>], key_heights: &[Vec<usize>]) {
    println!("Locks");
    for h in lock_heights {
        println!("{h:?}");
    }
    println!("Keys");
    for h in key_heights {
        println!("{h:?}");
    }
}

const LOCK_HEIGHT: usize = 7;

fn are_fitting(lock: &[usize], key: &[usize]) -> bool {
    lock.iter()
        .zip(key.iter())
        .all(|(l, k)| l + k < LOCK_HEIGHT - 1)
}

fn unique_pairs(schematics: &[Grid]) -> usize {
    let (lock_heights, key_heights) = schematics_to_heights(schematics);
    // print_heights(&lock_heights, &key_heights);

    lock_heights
        .iter()
        .map(|lock| {
            key_heights
                .iter()
                .filter(|key| are_fitting(lock, key))
                .count()
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let schematics = build(&input);

    println!("Part 1: {}", unique_pairs(&schematics));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(unique_pairs(&build(INPUT_TEST)), 3);
    }
}
