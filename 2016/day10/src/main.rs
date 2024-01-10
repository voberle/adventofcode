use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Element {
    Bot(usize),
    Output(usize),
}

impl Element {
    fn build(elt_type: &str, number: &str) -> Self {
        match elt_type {
            "bot" => Element::Bot(number.parse().unwrap()),
            "output" => Element::Output(number.parse().unwrap()),
            _ => panic!("Invalid element"),
        }
    }
}
#[derive(Debug)]
enum Instruction {
    Value {
        chip_value: usize,
        target: Element,
    }, // target is always bot
    LowHigh {
        src: Element,
        target_low: Element,
        target_high: Element,
    }, // src is always bot
}

fn build(input: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref RE_VALUE: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
        static ref RE_LOWHIGH: Regex =
            Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)")
                .unwrap();
    }
    input
        .lines()
        .map(|line| {
            if let Some(parts) = RE_VALUE.captures(line) {
                Instruction::Value {
                    chip_value: parts[1].parse().unwrap(),
                    target: Element::build("bot", &parts[2]),
                }
            } else if let Some(parts) = RE_LOWHIGH.captures(line) {
                Instruction::LowHigh {
                    src: Element::build("bot", &parts[1]),
                    target_low: Element::build(&parts[2], &parts[3]),
                    target_high: Element::build(&parts[4], &parts[5]),
                }
            } else {
                panic!("Invalid input")
            }
        })
        .collect()
}

fn part1(instructions: &[Instruction]) -> i64 {
    0
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);
    // println!("{:?}", instructions);

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
