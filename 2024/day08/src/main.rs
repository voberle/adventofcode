use std::{
    io::{self, Read},
    mem::swap,
};

use fxhash::FxHashSet;
use itertools::Itertools;

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

    #[allow(dead_code)]
    fn print(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    if c == '.' {
                        print!("{RED}#{RESET}");
                    } else {
                        print!("{RED}{c}{RESET}");
                    }
                } else {
                    print!("{c}");
                }
            }
            println!();
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
}

// Given two numbers, returns the number on both sides.
// If the number would be negative, it overlaps, to a very big positive one.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn get_antinode_side_pos(p1: usize, p2: usize) -> (usize, usize) {
    let diff = p1.abs_diff(p2) as isize;
    (
        (p1.min(p2) as isize - diff) as usize,
        (p1.max(p2) as isize + diff) as usize,
    )
}

fn antinode_positions(map: &Grid, f1: usize, f2: usize) -> impl Iterator<Item = usize> {
    let (up_row, down_row) = get_antinode_side_pos(map.row(f1), map.row(f2));
    let (mut left_col, mut right_col) = get_antinode_side_pos(map.col(f1), map.col(f2));

    // If the antinode positions are like:
    //   ..a
    //   a..
    // instead of:
    //   a..
    //   ..a
    // then swap the columns.
    if map.col(f1) > map.col(f2) {
        swap(&mut left_col, &mut right_col);
    }

    let up = if up_row < map.rows && left_col < map.cols {
        Some(map.pos(up_row, left_col))
    } else {
        None
    };
    let down = if down_row < map.rows && right_col < map.cols {
        Some(map.pos(down_row, right_col))
    } else {
        None
    };

    up.into_iter().chain(down)
}

fn unique_antinode_locations(map: &Grid) -> usize {
    // Find all different frequencies and their occurences count.
    let mut frequencies: FxHashSet<char> = FxHashSet::default();
    for f in map.values.iter().filter(|&&c| c != '.') {
        frequencies.insert(*f);
    }

    // For each, create all pair permutations and get the anti-node positions.
    let mut antinode_locations: FxHashSet<usize> = FxHashSet::default();
    for f in frequencies {
        for pair in map
            .values
            .iter()
            .enumerate()
            .filter_map(|(pos, c)| if *c == f { Some(pos) } else { None })
            .combinations(2)
        {
            antinode_locations.extend(antinode_positions(map, pair[0], pair[1]));
        }
    }

    // map.print(&antinode_locations.iter().copied().collect::<Vec<usize>>());

    antinode_locations.len()
}

fn part2(map: &Grid) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", unique_antinode_locations(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(unique_antinode_locations(&Grid::build(INPUT_TEST)), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
