use std::io::{self, Read};

fn build(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(prices: &[u64]) -> u64 {
    prices.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let prices = build(&input);

    println!("Part 1: {}", part1(&prices));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(part1(&build(INPUT_TEST)), 2895391);
    }
}
