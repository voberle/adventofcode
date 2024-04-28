use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn depth_increase_count(depths: &[u32]) -> usize {
    depths.windows(2).filter(|d| d[1] > d[0]).count()
}

fn sum_increase_count(depths: &[u32]) -> usize {
    depths
        .windows(3)
        .map(|d| d.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let depths = build(&input);

    println!("Part 1: {}", depth_increase_count(&depths));
    println!("Part 2: {}", sum_increase_count(&depths));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(depth_increase_count(&build(INPUT_TEST)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_increase_count(&build(INPUT_TEST)), 5);
    }
}
