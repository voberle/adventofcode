use std::{
    collections::HashMap,
    io::{self, Read},
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        chip_value: u32,
        target: Element,
    }, // target is always bot
    LowHigh {
        src: Element,
        low_target: Element,
        high_target: Element,
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
                    low_target: Element::build(&parts[2], &parts[3]),
                    high_target: Element::build(&parts[4], &parts[5]),
                }
            } else {
                panic!("Invalid input")
            }
        })
        .collect()
}

struct BotCore {
    low_target: Element,
    high_target: Element,
    values: Vec<u32>, // using a vec for values, as we don't know which one is low/high before we have both
}

impl BotCore {
    fn new(low_target: &Element, high_target: &Element) -> Self {
        BotCore {
            low_target: *low_target,
            high_target: *high_target,
            values: Vec::with_capacity(2),
        }
    }

    fn give_value(&mut self, value: u32) {
        // Bot must not be full
        assert!(self.values.len() < 2);
        self.values.push(value);
    }

    fn is_ready(&self) -> bool {
        self.values.len() == 2
    }

    fn exec(&mut self) -> [(u32, Element); 2] {
        assert!(self.is_ready());
        let (l, h) = if self.values[0] < self.values[1] {
            (self.values[0], self.values[1])
        } else {
            (self.values[1], self.values[0])
        };
        self.values.clear();
        [(l, self.low_target), (h, self.high_target)]
    }
}

fn values_has_bots_dest(values: &[(u32, Element)]) -> bool {
    values
        .iter()
        .any(|(_, dest)| matches!(dest, Element::Bot(_)))
}

fn execute<const SEARCH: bool>(
    instructions: &[Instruction],
    values_searched: (u32, u32),
) -> (Option<usize>, Option<Vec<(usize, u32)>>) {
    // List of value -> destination pairs.
    // This list comes from both value instructions and from bot execution.
    let mut values: Vec<(u32, Element)> = Vec::new();
    // Bots
    let mut bots: HashMap<Element, BotCore> = HashMap::new();

    // Initialize
    for ins in instructions {
        match ins {
            Instruction::Value { chip_value, target } => {
                values.push((*chip_value, *target));
            }
            Instruction::LowHigh {
                src,
                low_target,
                high_target,
            } => {
                bots.insert(*src, BotCore::new(low_target, high_target));
            }
        }
    }

    let mut outputs: Vec<(usize, u32)> = Vec::new();

    // Now we loop, emptying the values first, followed by bot execution, and so on
    while values_has_bots_dest(&values) {
        while let Some((value, dest)) = values.pop() {
            match dest {
                Element::Bot(_) => {
                    let bot_core = bots.get_mut(&dest).expect("Bot must exist");
                    bot_core.give_value(value);
                }
                Element::Output(id) => {
                    outputs.push((id, value));
                }
            }
        }
        for (src, b) in &mut bots {
            if !b.is_ready() {
                continue;
            }
            let r = b.exec();
            if SEARCH && r[0].0 == values_searched.0 && r[1].0 == values_searched.1 {
                // Found it
                if let Element::Bot(id) = src {
                    return (Some(*id), None);
                }
            }
            values.extend(r);
        }
    }
    (None, Some(outputs))
}

fn comparing_bot_id(instructions: &[Instruction], values_searched: (u32, u32)) -> usize {
    assert!(values_searched.0 < values_searched.1);
    execute::<true>(instructions, values_searched)
        .0
        .expect("Didn't find a result")
}

fn mult_output_123(instructions: &[Instruction]) -> u32 {
    let values = execute::<false>(instructions, (0, 0))
        .1
        .expect("Didn't find a result");
    values
        .iter()
        .filter_map(|(id, val)| {
            if [0, 1, 2].contains(id) {
                Some(val)
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", comparing_bot_id(&instructions, (17, 61)));
    println!("Part 2: {}", mult_output_123(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(comparing_bot_id(&build(INPUT_TEST), (2, 5)), 2);
    }
}
