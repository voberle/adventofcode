use std::io::{self, Read};

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

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn antinode_positions<const WITH_HARMONICS: bool>(map: &Grid, f1: usize, f2: usize) -> Vec<usize> {
    let mut results = Vec::new();

    if WITH_HARMONICS {
        // The frequencies themselves are locations.
        results.push(f1);
        results.push(f2);
    }

    let p1_row = map.row(f1);
    let p1_col = map.col(f1);
    let p2_row = map.row(f2);
    let p2_col = map.col(f2);

    let diff_row = p2_row as isize - p1_row as isize;
    let diff_col = p2_col as isize - p1_col as isize;

    let mut a1_row = (p1_row as isize - diff_row) as usize;
    let mut a1_col = (p1_col as isize - diff_col) as usize;
    let mut a2_row = (p2_row as isize + diff_row) as usize;
    let mut a2_col = (p2_col as isize + diff_col) as usize;

    while a1_row < map.rows && a1_col < map.cols {
        results.push(map.pos(a1_row, a1_col));
        a1_row = (a1_row as isize - diff_row) as usize;
        a1_col = (a1_col as isize - diff_col) as usize;

        if !WITH_HARMONICS {
            break;
        }
    }
    while a2_row < map.rows && a2_col < map.cols {
        results.push(map.pos(a2_row, a2_col));
        a2_row = (a2_row as isize + diff_row) as usize;
        a2_col = (a2_col as isize + diff_col) as usize;

        if !WITH_HARMONICS {
            break;
        }
    }

    results
}

fn unique_antinode_locations<const WITH_HARMONICS: bool>(map: &Grid) -> usize {
    let mut antinode_locations: FxHashSet<usize> = FxHashSet::default();
    // Find all different frequencies, and for each, create all pair permutations and get the anti-node positions.
    for f in map.values.iter().filter(|&&c| c != '.').unique() {
        for pair in map
            .values
            .iter()
            .enumerate()
            .filter_map(|(pos, c)| if c == f { Some(pos) } else { None })
            .combinations(2)
        {
            antinode_locations.extend(antinode_positions::<WITH_HARMONICS>(map, pair[0], pair[1]));
        }
    }

    // map.print(&antinode_locations.iter().copied().collect::<Vec<usize>>());

    antinode_locations.len()
}

fn unique_antinode_locs_base(map: &Grid) -> usize {
    unique_antinode_locations::<false>(map)
}

fn unique_antinode_locs_with_harmonics(map: &Grid) -> usize {
    unique_antinode_locations::<true>(map)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", unique_antinode_locs_base(&map));
    println!("Part 2: {}", unique_antinode_locs_with_harmonics(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(unique_antinode_locs_base(&Grid::build(INPUT_TEST_1)), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            unique_antinode_locs_with_harmonics(&Grid::build(INPUT_TEST_1)),
            34
        );
        assert_eq!(
            unique_antinode_locs_with_harmonics(&Grid::build(INPUT_TEST_2)),
            9
        );
    }
}
