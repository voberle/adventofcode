use std::io::{self, Read};

struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn build(s: &str) -> Self {
        let parts: Vec<u32> = s.split('-').map(|n| n.parse().unwrap()).collect();
        Self {
            low: parts[0],
            high: parts[1],
        }
    }

    fn count(&self) -> u32 {
        self.high - self.low + 1
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.high >= other.low && self.low <= other.high
    }
}

fn build(input: &str) -> Vec<Vec<Range>> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(Range::build).collect())
        .collect()
}

fn non_overlapping_range_sum(ranges: &[Range]) -> u32 {
    ranges.iter().map(Range::count).sum()
}

fn line_boxes_count(ranges: &[Range]) -> u32 {
    assert_eq!(ranges.len(), 2);
    if ranges[0].overlaps_with(&ranges[1]) {
        let merged_range = Range {
            low: ranges[0].low.min(ranges[1].low),
            high: ranges[0].high.max(ranges[1].high),
        };
        merged_range.count()
    } else {
        non_overlapping_range_sum(ranges)
    }
}

fn boxes_count_1(inventory: &[Vec<Range>]) -> u32 {
    inventory
        .iter()
        .map(|ranges| non_overlapping_range_sum(ranges))
        .sum()
}

fn boxes_count_2(inventory: &[Vec<Range>]) -> u32 {
    inventory
        .iter()
        .map(|ranges| line_boxes_count(ranges))
        .sum()
}

use std::cmp::Ordering;

/// Takes a list of ranges and simplifies it into an ordered non-overlapping list.
/// Works only with ranges that have inclusive start and exclusive end.
fn simplify_ranges<T>(ranges: &[(T, T)]) -> Vec<(T, T)>
where
    T: PartialOrd + Copy,
{
    // Algorith from https://cs.stackexchange.com/a/106978
    // List of beginning and ending points. The boolean tells the type (true is start, false is end).
    let mut positions: Vec<(bool, T)> = ranges
        .iter()
        .flat_map(|r| [(true, r.0), (false, r.1)])
        .collect();

    // Sort by position, with starting points comparing below ending points when positions are equal.
    positions.sort_by(|a, b| {
        if a.1 == b.1 {
            if a.0 && !b.0 {
                // a is start, b is end
                Ordering::Less
            } else if !a.0 && b.0 {
                // a is end, b is start
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    });
    assert!(positions.len() > 1 && positions.first().unwrap().0);

    let mut simplified_ranges = Vec::new();
    let mut start: T = positions.first().unwrap().1; // we need a value to initialize start with.
    let mut c = 0;
    for (pos_type, pos) in positions {
        if pos_type {
            // 0->1 transition
            if c == 0 {
                start = pos;
            }
            c += 1;
        } else {
            // 1->0 transition
            if c == 1 {
                let end = pos;
                simplified_ranges.push((start, end));
            }
            c -= 1;
        }
    }
    simplified_ranges
}

fn boxes_count_3(inventory: &[Vec<Range>]) -> u32 {
    inventory
        .windows(2)
        .map(|ranges| {
            // Convert the ranges into a list that simplify_ranges accepts.
            let norm_ranges: Vec<(u32, u32)> = ranges
                .iter()
                .flatten()
                .map(|r| (r.low, r.high + 1))
                .collect();
            let non_overlapping = simplify_ranges(&norm_ranges);
            non_overlapping.iter().map(|r| r.1 - r.0).sum()
        })
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let inventory = build(&input);

    println!("Part 1: {}", boxes_count_1(&inventory));
    println!("Part 2: {}", boxes_count_2(&inventory));
    println!("Part 3: {}", boxes_count_3(&inventory));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_simplify_ranges() {
        let ranges = [(5, 9), (0, 3), (4, 8)];
        assert_eq!(simplify_ranges(&ranges), &[(0, 3), (4, 9)]);
    }

    #[test]
    fn test_part1() {
        let inventory = build(&INPUT_TEST);
        assert_eq!(boxes_count_1(&inventory), 43);
    }

    #[test]
    fn test_part2() {
        let inventory = build(&INPUT_TEST);
        assert_eq!(boxes_count_2(&inventory), 35);
    }

    #[test]
    fn test_part3() {
        let inventory = build(&INPUT_TEST);
        assert_eq!(boxes_count_3(&inventory), 9);
    }
}
