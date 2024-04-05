use std::io::{self, Read};

use regex::Regex;

#[derive(Debug)]
struct Policy {
    number1: usize,
    number2: usize,
    letter: char,
}

impl Policy {
    fn check_policy1(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| *c == self.letter).count();
        self.number1 <= count && count <= self.number2
    }

    fn check_policy2(&self, password: &str) -> bool {
        let letter1 = password.chars().nth(self.number1 - 1).unwrap();
        let letter2 = password.chars().nth(self.number2 - 1).unwrap();
        (self.letter == letter1) != (self.letter == letter2)
    }
}

fn build(input: &str) -> Vec<(Policy, String)> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    input
        .lines()
        .map(|line| {
            let parts = re.captures(line).unwrap();
            (
                Policy {
                    number1: parts[1].parse().unwrap(),
                    number2: parts[2].parse().unwrap(),
                    letter: parts[3].chars().next().unwrap(),
                },
                parts[4].to_string(),
            )
        })
        .collect()
}

fn valid_password_count(list: &[(Policy, String)], policy_fn: fn(&Policy, &str) -> bool) -> usize {
    list.iter()
        .filter(|(policy, password)| policy_fn(policy, password))
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let list = build(&input);

    println!(
        "Part 1: {}",
        valid_password_count(&list, Policy::check_policy1)
    );
    println!(
        "Part 2: {}",
        valid_password_count(&list, Policy::check_policy2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(
            valid_password_count(&build(INPUT_TEST), Policy::check_policy1),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            valid_password_count(&build(INPUT_TEST), Policy::check_policy2),
            1
        );
    }
}
