use std::io::{self, Read};

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

fn build(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

fn checksum(spreadsheet: &[Vec<u32>]) -> u32 {
    spreadsheet
        .iter()
        .map(|row| {
            if let MinMax(x, y) = row.iter().minmax() {
                y - x
            } else {
                panic!("Invalid input")
            }
        })
        .sum()
}

fn part2(spreadsheet: &[Vec<u32>]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let spreadsheet = build(&input);

    println!("Part 1: {}", checksum(&spreadsheet));
    println!("Part 2: {}", part2(&spreadsheet));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(checksum(&build(INPUT_TEST)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
