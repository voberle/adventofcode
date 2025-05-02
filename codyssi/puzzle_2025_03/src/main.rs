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

fn boxes_count_1(inventory: &[Vec<Range>]) -> u32 {
    inventory
        .iter()
        .map(|ranges| non_overlapping_range_sum(ranges))
        .sum()
}

fn boxes_count_2(inventory: &[Vec<Range>]) -> u32 {
    inventory
        .iter()
        .map(|ranges| {
            if ranges[0].overlaps_with(&ranges[1]) {
                let merged_range = Range {
                    low: ranges[0].low.min(ranges[1].low),
                    high: ranges[0].high.max(ranges[1].high),
                };
                merged_range.count()
            } else {
                non_overlapping_range_sum(ranges)
            }
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let inventory = build(&input);

    println!("Part 1: {}", boxes_count_1(&inventory));
    println!("Part 2: {}", boxes_count_2(&inventory));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

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
}
