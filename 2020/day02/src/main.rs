use std::io::{self, Read};

use regex::Regex;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

fn build(input: &str) -> Vec<(Policy, String)> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    input
        .lines()
        .map(|line| {
            let parts = re.captures(line).unwrap();
            (
                Policy {
                    min: parts[1].parse().unwrap(),
                    max: parts[2].parse().unwrap(),
                    letter: parts[3].chars().next().unwrap(),
                },
                parts[4].to_string(),
            )
        })
        .collect()
}

fn valid_passwords_count(list: &[(Policy, String)]) -> usize {
    list.iter()
        .filter(|(policy, password)| {
            let count = password.chars().filter(|c| *c == policy.letter).count();
            policy.min <= count && count <= policy.max
        })
        .count()
}

fn part2(list: &[(Policy, String)]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let list = build(&input);

    println!("Part 1: {}", valid_passwords_count(&list));
    println!("Part 2: {}", part2(&list));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(valid_passwords_count(&build(INPUT_TEST)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
