use std::{
    fmt,
    io::{self, Read},
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// "hydrogen", "lithium" etc
type Element = String;

enum Object {
    Generator(Element),
    Microchip(Element),
}
use Object::{Generator, Microchip};

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generator(e) => write!(f, "{}-G", e),
            Self::Microchip(e) => write!(f, "{}-M", e),
        }
    }
}

// First floor is index 0
fn build(input: &str) -> Vec<Vec<Object>> {
    lazy_static! {
        static ref RE_GEN: Regex = Regex::new(r"\W(\w+) generator").unwrap();
        static ref RE_CHIP: Regex = Regex::new(r"\W(\w+)-compatible microchip").unwrap();
    }
    input
        .lines()
        .map(|line| {
            let mut floor = Vec::new();
            for (_, [p]) in RE_GEN.captures_iter(line).map(|c| c.extract()) {
                floor.push(Generator(p.to_string()));
            }
            for (_, [p]) in RE_CHIP.captures_iter(line).map(|c| c.extract()) {
                floor.push(Microchip(p.to_string()));
            }
            floor
        })
        .collect()
}

fn print_floors(floors: &[Vec<Object>]) {
    for (level, floor) in floors.into_iter().rev().enumerate() {
        println!(
            "F{}: {}",
            floors.len() - level,
            floor.iter().map(|e| e.to_string()).join(" ")
        )
    }
}

fn part1(floors: &[Vec<Object>]) -> i64 {
    0
}

fn part2(floors: &[Vec<Object>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let floors = build(&input);
    print_floors(&floors);

    println!("Part 1: {}", part1(&floors));
    println!("Part 2: {}", part2(&floors));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
