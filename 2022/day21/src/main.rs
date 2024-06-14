use std::{
    collections::VecDeque,
    io::{self, Read},
};

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Element {
    Monkey(String),
    Number(u64),
}

#[derive(Debug, Clone)]
enum Operation {
    Value(Element),
    Addition(Element, Element),
    Subtraction(Element, Element),
    Multiplication(Element, Element),
    Division(Element, Element),
}

impl From<&str> for Operation {
    #[allow(clippy::match_on_vec_items)]
    fn from(value: &str) -> Self {
        if let Ok(n) = value.parse() {
            Operation::Value(Element::Number(n))
        } else {
            let p: Vec<_> = value.split_ascii_whitespace().collect();
            let m1 = Element::Monkey(p[0].to_string());
            let m2 = Element::Monkey(p[2].to_string());
            match p[1] {
                "+" => Operation::Addition(m1, m2),
                "-" => Operation::Subtraction(m1, m2),
                "*" => Operation::Multiplication(m1, m2),
                "/" => Operation::Division(m1, m2),
                _ => panic!("Unknown operation"),
            }
        }
    }
}

fn build(input: &str) -> Vec<(String, Operation)> {
    input
        .lines()
        .map(|line| {
            let (m, op) = line.split(": ").collect_tuple().unwrap();
            (m.to_string(), op.into())
        })
        .collect()
}

fn get_val(e: &Element, monkeys: &FxHashMap<String, u64>) -> Option<u64> {
    match e {
        Element::Monkey(n) => monkeys.get(n).copied(),
        Element::Number(n) => Some(*n),
    }
}

fn root_number(operations: &[(String, Operation)]) -> u64 {
    let mut monkeys: FxHashMap<String, u64> = FxHashMap::default();
    let mut operations: VecDeque<(String, Operation)> = operations.to_vec().into();

    while let Some((m, op)) = operations.pop_front() {
        match &op {
            Operation::Value(v) => {
                let n = match v {
                    Element::Number(n) => n,
                    Element::Monkey(_) => panic!("Should have been a number"),
                };
                assert!(monkeys.insert(m, *n).is_none());
            }
            Operation::Addition(m1, m2) => {
                if let (Some(v1), Some(v2)) = (get_val(m1, &monkeys), get_val(m2, &monkeys)) {
                    monkeys.insert(m, v1 + v2);
                } else {
                    operations.push_back((m, op));
                }
            }
            Operation::Subtraction(m1, m2) => {
                if let (Some(v1), Some(v2)) = (get_val(m1, &monkeys), get_val(m2, &monkeys)) {
                    monkeys.insert(m, v1 - v2);
                } else {
                    operations.push_back((m, op));
                }
            }
            Operation::Multiplication(m1, m2) => {
                if let (Some(v1), Some(v2)) = (get_val(m1, &monkeys), get_val(m2, &monkeys)) {
                    monkeys.insert(m, v1 * v2);
                } else {
                    operations.push_back((m, op));
                }
            }
            Operation::Division(m1, m2) => {
                if let (Some(v1), Some(v2)) = (get_val(m1, &monkeys), get_val(m2, &monkeys)) {
                    monkeys.insert(m, v1 / v2);
                } else {
                    operations.push_back((m, op));
                }
            }
        }
    }
    *monkeys.get("root").expect("Didn't find root")
}

fn part2(operations: &[(String, Operation)]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let operations = build(&input);

    println!("Part 1: {}", root_number(&operations));
    println!("Part 2: {}", part2(&operations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(root_number(&build(INPUT_TEST)), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
