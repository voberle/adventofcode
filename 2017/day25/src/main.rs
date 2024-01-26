use std::io::{self, Read};

use fxhash::FxHashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    value_to_write: bool, // 0 or 1
    moving_dir: bool,     // false = left, true = right
    next_state: char,
}

#[derive(Debug)]
struct Blueprint {
    begin_state: char,
    checksum_steps_level: usize,
    // state to instruction for value 0, for value 1
    state_instructions: FxHashMap<char, (Instruction, Instruction)>,
}

fn build(input: &str) -> Blueprint {
    let mut it = input.lines();

    let begin_state = Regex::new(r"Begin in state (\w).")
        .unwrap()
        .captures(it.next().unwrap())
        .unwrap()[1]
        .chars()
        .next()
        .unwrap();
    let checksum_steps_level = Regex::new(r"Perform a diagnostic checksum after (\d+) steps.")
        .unwrap()
        .captures(it.next().unwrap())
        .unwrap()[1]
        .parse()
        .unwrap();

    let mut state_instructions: FxHashMap<char, (Instruction, Instruction)> = FxHashMap::default();
    let re_state = Regex::new(r"In state (\w):").unwrap();
    let re_next_state = Regex::new(r"    - Continue with state (\w).").unwrap();
    while it.next().is_some() {
        let state = re_state.captures(it.next().unwrap()).unwrap()[1]
            .chars()
            .next()
            .unwrap();

        assert_eq!(it.next().unwrap(), "  If the current value is 0:");
        let value_to_write_0 = it.next().unwrap() == "    - Write the value 1.";
        let moving_dir_0 = it.next().unwrap() == "    - Move one slot to the right.";
        let next_state_0 = re_next_state.captures(it.next().unwrap()).unwrap()[1]
            .chars()
            .next()
            .unwrap();
        assert_eq!(it.next().unwrap(), "  If the current value is 1:");
        let value_to_write_1 = it.next().unwrap() == "    - Write the value 1.";
        let moving_dir_1 = it.next().unwrap() == "    - Move one slot to the right.";
        let next_state_1 = re_next_state.captures(it.next().unwrap()).unwrap()[1]
            .chars()
            .next()
            .unwrap();
        state_instructions.insert(
            state,
            (
                Instruction {
                    value_to_write: value_to_write_0,
                    moving_dir: moving_dir_0,
                    next_state: next_state_0,
                },
                Instruction {
                    value_to_write: value_to_write_1,
                    moving_dir: moving_dir_1,
                    next_state: next_state_1,
                },
            ),
        );
    }

    Blueprint {
        begin_state,
        checksum_steps_level,
        state_instructions,
    }
}

struct Machine {
    tape: FxHashMap<i32, bool>,
    cursor: i32,
    state: char,
}

impl Machine {
    fn new(initial_state: char) -> Self {
        Self {
            tape: FxHashMap::default(),
            cursor: 0,
            state: initial_state,
        }
    }

    fn exec(&mut self, blueprint: &Blueprint) {
        let current_val = self.tape.entry(self.cursor).or_default();
        let (instruction_0, instruction_1) = blueprint.state_instructions.get(&self.state).unwrap();
        let instruction = if *current_val {
            instruction_1
        } else {
            instruction_0
        };

        *current_val = instruction.value_to_write;
        self.cursor += if instruction.moving_dir { 1 } else { -1 };
        self.tape.entry(self.cursor).or_default(); // not necessary, but makes printing nicer
        self.state = instruction.next_state;
    }

    #[allow(dead_code)]
    fn tape_to_string(&self) -> String {
        self.tape
            .iter()
            .sorted_unstable_by_key(|(k, _)| *k)
            .map(|(k, v)| {
                let mut s = String::new();
                s += if *k == self.cursor { "[" } else { " " };
                s += if *v { "1" } else { "0" };
                s += if *k == self.cursor { "]" } else { " " };
                s
            })
            .join(" ")
    }
}

fn diagnostic_checksum(blueprint: &Blueprint) -> usize {
    let mut machine = Machine::new(blueprint.begin_state);
    // println!("Before: {}", machine.tape_to_string());
    for _step in 1..=blueprint.checksum_steps_level {
        machine.exec(blueprint);
        // println!("Step {}: {}", step, machine.tape_to_string());
    }
    machine.tape.values().filter(|v| **v).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let blueprint = build(&input);
    // println!("{:#?}", blueprint);

    println!("Part 1: {}", diagnostic_checksum(&blueprint));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(diagnostic_checksum(&build(INPUT_TEST)), 3);
    }
}
