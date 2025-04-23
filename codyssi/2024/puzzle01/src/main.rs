use std::io::{self, Read};

fn build(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(prices: &[u64]) -> u64 {
    prices.iter().sum()
}

fn part2(prices: &[u64], items_for_free_cnt: usize) -> u64 {
    let mut sorted = prices.to_vec();
    sorted.sort_unstable();
    sorted[..sorted.len() - items_for_free_cnt].iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let prices = build(&input);

    println!("Part 1: {}", part1(&prices));
    println!("Part 2: {}", part2(&prices, 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 2895391);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST), 2), 1261624);
    }
}
