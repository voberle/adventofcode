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

fn evenly_divisible_sum(spreadsheet: &[Vec<u32>]) -> u32 {
    spreadsheet
        .iter()
        .map(|row| {
            let divisible_pair = row
                .iter()
                .permutations(2)
                .map(|v| {
                    if v[0] > v[1] {
                        (v[0], v[1])
                    } else {
                        (v[1], v[0])
                    }
                })
                // .inspect(|(&x, &y)| println!("{x} % {y} == {}", x % y) )
                .find(|(&x, &y)| x % y == 0)
                .expect("Should have found only one divisible pair");
            divisible_pair.0 / divisible_pair.1
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let spreadsheet = build(&input);

    println!("Part 1: {}", checksum(&spreadsheet));
    println!("Part 2: {}", evenly_divisible_sum(&spreadsheet));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(checksum(&build(INPUT_TEST_1)), 18);
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part2() {
        assert_eq!(evenly_divisible_sum(&build(INPUT_TEST_2)), 9);
    }
}
