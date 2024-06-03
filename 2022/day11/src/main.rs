use std::{
    collections::VecDeque,
    io::{self, Read},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u32),
    Mult(u32),
    Squared,
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: VecDeque<u32>,
    operation: Operation,
    test: u32,
    on_true: usize,
    on_false: usize,
    // Number of times a monkey inspects items.
    inspect_count: u32,
}

impl From<&str> for Monkey {
    fn from(monkey: &str) -> Self {
        let mut it = monkey.lines();
        assert!(it.next().unwrap().starts_with("Monkey"));
        let starting_items: VecDeque<u32> = it
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|v| v.parse().unwrap())
            .collect();
        let (op, op_val) = it
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split(' ')
            .collect_tuple()
            .unwrap();
        let operation = if op_val == "old" {
            Operation::Squared
        } else {
            let val = op_val.parse().unwrap();
            match op {
                "+" => Operation::Add(val),
                "*" => Operation::Mult(val),
                _ => panic!("Unknown operation"),
            }
        };
        let test = it
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let on_true = it
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let on_false = it
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            starting_items,
            operation,
            test,
            on_true,
            on_false,
            inspect_count: 0,
        }
    }
}

fn build(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Into::into).collect()
}

fn exec_round(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        while let Some(worry_level) = monkeys[i].starting_items.pop_front() {
            let worry_level = match monkeys[i].operation {
                Operation::Add(v) => worry_level + v,
                Operation::Mult(v) => worry_level * v,
                Operation::Squared => worry_level * worry_level,
            } / 3;
            let next_monkey_id = if worry_level % monkeys[i].test == 0 {
                monkeys[i].on_true
            } else {
                monkeys[i].on_false
            };
            monkeys[next_monkey_id]
                .starting_items
                .push_back(worry_level);

            monkeys[i].inspect_count += 1;
        }
    }
}

fn monkey_business_after_20(monkeys: &[Monkey]) -> u32 {
    let mut monkeys = monkeys.to_vec();
    for _ in 0..20 {
        exec_round(&mut monkeys);
    }

    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys[monkeys.len() - 1].inspect_count * monkeys[monkeys.len() - 2].inspect_count
}

fn part2(monkeys: &[Monkey]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let monkeys = build(&input);
    // println!("{:?}", monkeys);

    println!("Part 1: {}", monkey_business_after_20(&monkeys));
    println!("Part 2: {}", part2(&monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(monkey_business_after_20(&build(INPUT_TEST)), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
