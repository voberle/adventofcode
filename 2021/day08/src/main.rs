use std::io::{self, Read};

use itertools::Itertools;

struct Display {
    signal_patters: Vec<Vec<char>>,
    output_values: Vec<Vec<char>>,
}

impl Display {
    fn build(line: &str) -> Self {
        let (signal_patters, output_value) = line
            .split(" | ")
            .map(|s| s.split_whitespace().map(|p| p.chars().collect()).collect())
            .collect_tuple()
            .unwrap();
        Self {
            signal_patters,
            output_values: output_value,
        }
    }
}

fn build(input: &str) -> Vec<Display> {
    input.lines().map(Display::build).collect()
}

// how many times 1, 4, 7, or 8 appear
fn subset_digits_count(displays: &[Display]) -> usize {
    let segment_counts = [
        2, // 1
        3, // 7
        4, // 4
        7, // 8
    ];
    displays
        .iter()
        .map(|d| {
            d.output_values
                .iter()
                .filter(|v| segment_counts.contains(&v.len()))
                .count()
        })
        .sum()
}

fn part2(displays: &[Display]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let displays = build(&input);

    println!("Part 1: {}", subset_digits_count(&displays));
    println!("Part 2: {}", part2(&displays));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(subset_digits_count(&build(INPUT_TEST)), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
