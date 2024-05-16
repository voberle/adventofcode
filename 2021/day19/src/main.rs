use std::{
    collections::VecDeque,
    fmt,
    io::{self, Read},
    ops::Sub,
};

use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn build(s: &str) -> Self {
        let p: Vec<_> = s.split(',').map(|n| n.parse().unwrap()).collect();
        Self::new(p[0], p[1], p[2])
    }

    // Returns the position in one of the possible orientations.
    fn orientate(&self, n: u8) -> Self {
        // Hard-coding all options is simpler.
        match n {
            0 => Self::new(self.x, self.y, self.z),
            1 => Self::new(self.x, -self.z, self.y),
            2 => Self::new(self.x, self.z, -self.y),
            3 => Self::new(self.x, -self.y, -self.z),

            4 => Self::new(-self.x, self.z, self.y),
            5 => Self::new(-self.x, self.y, -self.z),
            6 => Self::new(-self.x, -self.y, self.z),
            7 => Self::new(-self.x, -self.z, -self.y),

            8 => Self::new(self.y, self.z, self.x),
            9 => Self::new(self.y, -self.x, self.z),
            10 => Self::new(self.y, -self.z, -self.x),
            11 => Self::new(self.y, self.x, -self.z),

            12 => Self::new(-self.y, self.x, self.z),
            13 => Self::new(-self.y, -self.x, -self.z),
            14 => Self::new(-self.y, -self.z, self.x),
            15 => Self::new(-self.y, self.z, -self.x),

            16 => Self::new(self.z, self.y, -self.x),
            17 => Self::new(self.z, self.x, self.y),
            18 => Self::new(self.z, -self.y, self.x),
            19 => Self::new(self.z, -self.x, -self.y),

            20 => Self::new(-self.z, self.y, self.x),
            21 => Self::new(-self.z, self.x, -self.y),
            22 => Self::new(-self.z, -self.y, -self.x),
            23 => Self::new(-self.z, -self.x, self.y),

            _ => panic!("Invalid n"),
        }
    }

    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

impl Sub for &Pos {
    type Output = Pos;

    fn sub(self, other: &Pos) -> Self::Output {
        Pos::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    scanner_number: usize,
    positions: FxHashSet<Pos>,
}

impl Scanner {
    fn build(scanner_number: usize, input: &str) -> Self {
        Self {
            scanner_number,
            positions: input.lines().skip(1).map(Pos::build).collect(),
        }
    }

    fn beacons_count(&self) -> usize {
        self.positions.len()
    }

    // Creates a new scanner with positions moved by an offset.
    fn move_positions(&self, offset: &Pos) -> Self {
        Scanner {
            scanner_number: self.scanner_number,
            positions: self.positions.iter().map(|b| b - offset).collect(),
        }
    }

    fn count_overlap(&self, other: &Self) -> usize {
        self.positions.intersection(&other.positions).count()
    }

    fn get_orientation(&self, n: u8) -> Self {
        Scanner {
            scanner_number: self.scanner_number,
            positions: self.positions.iter().map(|b| b.orientate(n)).collect(),
        }
    }
}

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "-- scanner {} --", self.scanner_number)?;
        for p in &self.positions {
            writeln!(f, "{}", p)?;
        }
        Ok(())
    }
}

fn build(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, p)| Scanner::build(i, p))
        .collect()
}

// Scanner 0 is at (0, 0, 0). We don't rotate it.
// - Take first beacon of scanner 0: Ref Beacon.
// -- For each beacon in scanner 1:
// --- Align all beacons of scanner 1 with Ref Beacon.
// --- Count how many other beacons of both scanners match.
//     If >= 12, they overlap. We then know pos of scanner 1. It's the offset.
// - If no overlap found, take next beacon of scanner 0 as ref.
// Also explore all orientations.
//
// If overlap, we should be able to merge scanner 1 into 0, and proceed with next scanner.
// Once all merged, we have the final list of beacons.

fn find_overlaping_scanner(
    ref_scanner: &Scanner,
    other_scanner: &Scanner,
) -> Option<(Scanner, Pos)> {
    for ref_beacon in &ref_scanner.positions {
        for beacon in &other_scanner.positions {
            let offset = beacon - ref_beacon;
            let aligned_scanner = other_scanner.move_positions(&offset);
            if aligned_scanner.count_overlap(ref_scanner) >= 12 {
                // They overlap
                return Some((aligned_scanner, offset));
            }
        }
    }
    None
}

fn merge_scanners(report: &[Scanner]) -> (Scanner, Vec<Pos>) {
    let mut scanners_to_check: VecDeque<Scanner> = report.iter().cloned().collect();
    let mut ref_scanner = scanners_to_check.pop_front().unwrap();

    let mut found_scanners_positions: Vec<Pos> = Vec::new();
    found_scanners_positions.push(Pos::new(0, 0, 0));

    'outer: while let Some(scanner) = scanners_to_check.pop_front() {
        for orientation in 0..24 {
            let orientated_scanner = scanner.get_orientation(orientation);

            if let Some((overlaping_scanner, scanner_pos)) =
                find_overlaping_scanner(&ref_scanner, &orientated_scanner)
            {
                ref_scanner.positions.extend(overlaping_scanner.positions);

                found_scanners_positions.push(scanner_pos);

                continue 'outer;
            }
        }
        // This scanner doesn't overlap, trying later again
        scanners_to_check.push_back(scanner);
    }

    (ref_scanner, found_scanners_positions)
}

fn beacons_count(merged_scanner: &Scanner) -> usize {
    merged_scanner.beacons_count()
}

fn largest_dist(scanners_positions: &[Pos]) -> u32 {
    scanners_positions
        .iter()
        .permutations(2)
        .map(|p| p[0].distance(p[1]))
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let scanners = build(&input);

    let (merged_scanner, scanners_positions) = merge_scanners(&scanners);

    println!("Part 1: {}", beacons_count(&merged_scanner));
    println!("Part 2: {}", largest_dist(&scanners_positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1_2() {
        let (merged_scanner, scanners_positions) = merge_scanners(&build(INPUT_TEST));
        assert_eq!(beacons_count(&merged_scanner), 79);
        assert_eq!(largest_dist(&scanners_positions), 3621);
    }
}
