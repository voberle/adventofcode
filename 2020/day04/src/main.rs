use std::{io::{self, Read}, string::ToString};

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Passport {
    fields: FxHashMap<String, String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            fields: FxHashMap::default(),
        }
    }
}

fn build(input: &str) -> Vec<Passport> {
    let mut passports = vec![Passport::new()];
    for line in input.lines() {
        // Passport data can be on several lines.
        if line.is_empty() {
            passports.push(Passport::new());
        }
        passports.last_mut().unwrap().fields.extend(
            line.split_whitespace()
                .map(|f| f.split(':').map(ToString::to_string).collect_tuple().unwrap()),
        );
    }
    passports
}

fn valid_passports_count(passports: &[Passport]) -> usize {
    const MANDATORY_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    passports
        .iter()
        .filter(|p| {
            MANDATORY_FIELDS
                .iter()
                .all(|name| p.fields.contains_key(*name))
        })
        .count()
}

fn part2(passports: &[Passport]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let passports = build(&input);
    // println!("{:?}", passports);
    println!("{}", passports.len());

    println!("Part 1: {}", valid_passports_count(&passports));
    println!("Part 2: {}", part2(&passports));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(valid_passports_count(&build(INPUT_TEST)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
