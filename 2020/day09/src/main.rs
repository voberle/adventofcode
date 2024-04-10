use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn first_invalid_number(numbers: &[u64], preamble_size: usize) -> u64 {
    *numbers
        .iter()
        .enumerate()
        .skip(preamble_size)
        .find(|(i, number)| {
            numbers[i - preamble_size..*i]
                .iter()
                .filter(|p| p <= number) // small optimization, doesn't change much.
                .combinations(2)
                .map(|combi| combi[0] + combi[1])
                .all(|sum| sum != **number)
        })
        .unwrap()
        .1
}

fn part2(numbers: &[u64]) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(&input);

    println!("Part 1: {}", first_invalid_number(&numbers, 25));
    println!("Part 2: {}", part2(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(first_invalid_number(&build(INPUT_TEST), 5), 127);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
