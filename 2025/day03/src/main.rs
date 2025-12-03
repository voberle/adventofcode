use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn max_joltage(bank: &[u32]) -> u32 {
    // How to do this with only one iterator?
    let d1 = bank[..bank.len() - 1].iter().max().unwrap();
    let d1_pos = bank.iter().position(|v| v == d1).unwrap();

    let d2 = bank[d1_pos + 1..].iter().max().unwrap();

    d1 * 10 + d2
}

fn total_output_joltage(banks: &[Vec<u32>]) -> u32 {
    banks.iter().map(|bank| max_joltage(bank)).sum()
}

fn part2(banks: &[Vec<u32>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let banks = build(&input);

    println!("Part 1: {}", total_output_joltage(&banks));
    println!("Part 2: {}", part2(&banks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(total_output_joltage(&build(INPUT_TEST)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
