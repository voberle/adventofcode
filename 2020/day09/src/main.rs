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
                .tuple_combinations() // tuple_combinations() instead of combinations() is quite a bit faster.
                .map(|(a, b)| a + b)
                .all(|sum| sum != **number)
        })
        .unwrap()
        .1
}

fn encryption_weakness(numbers: &[u64], invalid_number: u64) -> u64 {
    for i in 0..numbers.len() {
        let (mut min, mut max) = (numbers[i], numbers[i]);
        let mut sum: u64 = 0;
        for &n in &numbers[i..] {
            sum += n;
            if sum > invalid_number {
                continue;
            }

            min = min.min(n);
            max = max.max(n);

            if sum == invalid_number {
                return min + max;
            }
        }
    }
    panic!("Didnb't find weakness")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(&input);

    let invalid_number = first_invalid_number(&numbers, 25);
    println!("Part 1: {}", invalid_number);
    println!("Part 2: {}", encryption_weakness(&numbers, invalid_number));
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
        let invalid_number = first_invalid_number(&build(INPUT_TEST), 5);
        assert_eq!(encryption_weakness(&build(INPUT_TEST), invalid_number), 62);
    }
}
