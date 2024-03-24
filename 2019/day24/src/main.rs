use std::{
    collections::VecDeque,
    io::{self, Read},
};

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
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            values: vec![false; rows * cols],
            rows,
            cols,
        }
    }

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
    fn print(&self, spaces: bool) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                if spaces && p == 12 {
                    print!("?");
                } else {
                    let c = self.values[p];
                    print!("{}", if c { '#' } else { '.' });
                }
            }
            println!();
        }
    }

    fn is_bug(&self, pos: usize) -> bool {
        self.values[pos]
    }

    fn count_bugs(&self) -> usize {
        self.values.iter().filter(|b| **b).count()
    }

    fn is_bug_in_dir(&self, pos: usize, direction: Direction) -> bool {
        match direction {
            North => {
                if pos < self.cols {
                    false
                } else {
                    self.is_bug(pos - self.cols)
                }
            }
            East => {
                if pos % self.cols == self.cols - 1 {
                    false
                } else {
                    self.is_bug(pos + 1)
                }
            }
            South => {
                if pos / self.cols == self.rows - 1 {
                    false
                } else {
                    self.is_bug(pos + self.cols)
                }
            }
            West => {
                if pos % self.cols == 0 {
                    false
                } else {
                    self.is_bug(pos - 1)
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

fn next_minute_for_scan(scan: &Scan) -> Scan {
    let mut next = scan.clone();
    for p in 0..scan.values.len() {
        let adjacent_bug_counts = scan.get_adjacent_bugs_count(p);
        if scan.values[p] && adjacent_bug_counts != 1 {
            // Bug dies
            next.values[p] = false;
        }
        if !scan.values[p] && [1, 2].contains(&adjacent_bug_counts) {
            // Empty space gets infected
            next.values[p] = true;
        }
    }
    next
}

fn bio_diversity_rating_double_layout(scan: &Scan) -> u64 {
    // Storing the diversity ratings instead of the full scan, it's faster.
    let mut generated_scans: FxHashSet<u64> = FxHashSet::default();

    let mut scan = scan.clone();
    loop {
        let next_scan = next_minute_for_scan(&scan);

        let diversity_rating = next_scan.get_bio_diversity_rating();
        if !generated_scans.insert(diversity_rating) {
            return diversity_rating;
        }

        scan = next_scan;
    }
}

#[allow(
    dead_code,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
fn print_space(space: &VecDeque<Scan>) {
    for (d, level) in space.iter().enumerate() {
        println!("Depth {}:", d as i32 - space.len() as i32 / 2);
        level.print(true);
        println!();
    }
}

// Counting bugs in 3D space.
#[allow(clippy::manual_range_patterns)]
fn get_adjacent_bugs_count(space: &VecDeque<Scan>, level: usize, pos: usize) -> usize {
    let adj_on_level = space[level].get_adjacent_bugs_count(pos);

    // Two closures that convert the bug boolean to an integer.
    let cnt_down = |p: usize| {
        if level > 0 {
            usize::from(space[level - 1].is_bug(p - 1))
        } else {
            0
        }
    };
    let cnt_up = |p: usize| {
        if level < space.len() - 1 {
            usize::from(space[level + 1].is_bug(p - 1))
        } else {
            0
        }
    };

    // Matching on pos + 1 to align with the drawing in the description.
    let on_other_levels = match pos + 1 {
        7 | 9 | 17 | 19 => 0,
        2 | 3 | 4 => cnt_down(8),
        10 | 15 | 20 => cnt_down(14),
        6 | 11 | 16 => cnt_down(12),
        22 | 23 | 24 => cnt_down(18),
        1 => cnt_down(8) + cnt_down(12),
        5 => cnt_down(8) + cnt_down(14),
        21 => cnt_down(12) + cnt_down(18),
        25 => cnt_down(14) + cnt_down(18),
        8 => cnt_up(1) + cnt_up(2) + cnt_up(3) + cnt_up(4) + cnt_up(5),
        12 => cnt_up(1) + cnt_up(6) + cnt_up(11) + cnt_up(16) + cnt_up(21),
        14 => cnt_up(5) + cnt_up(10) + cnt_up(15) + cnt_up(20) + cnt_up(25),
        18 => cnt_up(21) + cnt_up(22) + cnt_up(23) + cnt_up(24) + cnt_up(25),
        // 13 => 0, // middle tile, skipped
        _ => panic!("Bug, position not handled: {}", pos + 1),
    };

    adj_on_level + on_other_levels
}

// Make sure that levels at both ends are empty.
fn ensure_first_last_empty(space: &mut VecDeque<Scan>) {
    if space.front().unwrap().count_bugs() != 0 {
        space.push_front(Scan::new(5, 5));
    }
    if space.back().unwrap().count_bugs() != 0 {
        space.push_back(Scan::new(5, 5));
    }
}

fn next_minute_for_space(space: &VecDeque<Scan>) -> VecDeque<Scan> {
    // Here we guarantee that space first and last levels are empty,
    // that way next is initially an exact copy of space, ensuring that levels have the same indexes in both.

    let mut next = space.clone();

    for level in 0..next.len() {
        for p in 0..next[level].values.len() {
            if p == 12 {
                // Skip center
                continue;
            }

            let adjacent_bug_counts = get_adjacent_bugs_count(space, level, p);
            if space[level].values[p] && adjacent_bug_counts != 1 {
                // Bug dies
                next[level].values[p] = false;
            }
            if !space[level].values[p] && [1, 2].contains(&adjacent_bug_counts) {
                // Empty space gets infected
                next[level].values[p] = true;
            }
        }
    }

    ensure_first_last_empty(&mut next);

    next
}

fn bug_counts_after(scan: &Scan, time: usize) -> usize {
    let mut space: VecDeque<Scan> = VecDeque::new();
    space.push_back(scan.clone());
    ensure_first_last_empty(&mut space);

    for _ in 0..time {
        space = next_minute_for_space(&space);
    }
    // print_space(&space);

    space.iter().map(Scan::count_bugs).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let scan = Scan::build(&input);

    println!("Part 1: {}", bio_diversity_rating_double_layout(&scan));
    println!("Part 2: {}", bug_counts_after(&scan, 200));
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
        assert_eq!(bug_counts_after(&Scan::build(INPUT_TEST), 10), 99);
    }
}
