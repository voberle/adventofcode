use std::io::{self, Read};

use fxhash::FxHashSet;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

#[derive(Debug, Clone)]
struct Scan {
    values: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl Scan {
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

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                print!("{}", if c { '#' } else { '.' });
            }
            println!();
        }
    }

    fn is_bug_in_dir(&self, pos: usize, direction: Direction) -> bool {
        match direction {
            North => {
                if pos < self.cols {
                    false
                } else {
                    self.values[pos - self.cols]
                }
            }
            East => {
                if pos % self.cols == self.cols - 1 {
                    false
                } else {
                    self.values[pos + 1]
                }
            }
            South => {
                if pos / self.cols == self.rows - 1 {
                    false
                } else {
                    self.values[pos + self.cols]
                }
            }
            West => {
                if pos % self.cols == 0 {
                    false
                } else {
                    self.values[pos - 1]
                }
            }
        }
    }

    fn get_adjacent_bugs_count(&self, pos: usize) -> usize {
        [North, East, South, West]
            .iter()
            .filter(|dir| self.is_bug_in_dir(pos, **dir))
            .count()
    }

    fn next_minute(&self) -> Scan {
        let mut next = self.clone();
        for p in 0..self.values.len() {
            let adjacent_bug_counts = self.get_adjacent_bugs_count(p);
            if self.values[p] && adjacent_bug_counts != 1 {
                // Bug dies
                next.values[p] = false;
            }
            if !self.values[p] && [1, 2].contains(&adjacent_bug_counts) {
                // Empty space gets infected
                next.values[p] = true;
            }
        }
        next
    }

    fn get_bio_diversity_rating(&self) -> u64 {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, is_bug)| {
                if *is_bug {
                    Some(2_u64.pow(i.try_into().unwrap()))
                } else {
                    None
                }
            })
            .sum()
    }
}

fn bio_diversity_rating_double_layout(scan: &Scan) -> u64 {
    // Storing the diversity ratings instead of the full scan, it's faster.
    let mut generated_scans: FxHashSet<u64> = FxHashSet::default();

    let mut scan = scan.clone();
    loop {
        let next_scan = scan.next_minute();

        let diversity_rating = next_scan.get_bio_diversity_rating();
        if !generated_scans.insert(diversity_rating) {
            return diversity_rating;
        }

        scan = next_scan;
    }
}

fn part2(scan: &Scan) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let scan = Scan::build(&input);

    println!("Part 1: {}", bio_diversity_rating_double_layout(&scan));
    println!("Part 2: {}", part2(&scan));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(
            bio_diversity_rating_double_layout(&Scan::build(INPUT_TEST)),
            2129920
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Scan::build(INPUT_TEST)), 0);
    }
}
