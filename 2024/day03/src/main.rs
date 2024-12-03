use std::io::{self, Read};

use regex::Regex;

fn multiplication_result(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap())
        .sum()
}

fn part2(input: &str) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", multiplication_result(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(multiplication_result(INPUT_TEST), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEST), 0);
    }
}
