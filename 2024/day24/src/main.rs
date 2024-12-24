use std::io::{self, Read};

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl Gate {
    fn new(line: &str) -> Self {
        // x00 AND y00 -> z00
        let parts: Vec<_> = line.split_whitespace().collect();
        let (in1, in2, out) = (
            parts[0].to_string(),
            parts[2].to_string(),
            parts[4].to_string(),
        );
        match parts.get(1) {
            Some(&"AND") => Gate::And(in1, in2, out),
            Some(&"OR") => Gate::Or(in1, in2, out),
            Some(&"XOR") => Gate::Xor(in1, in2, out),
            _ => panic!("Invalid gate"),
        }
    }

    // Executes the gate both inputs are available and there is no output.
    fn exec_with(
        in1: &str,
        in2: &str,
        out: &str,
        wires: &mut FxHashMap<String, u8>,
        op_fn: fn(u8, u8) -> u8,
    ) -> bool {
        if wires.contains_key(out) {
            return false;
        }
        if let Some(in1_val) = wires.get(in1) {
            if let Some(in2_val) = wires.get(in2) {
                wires.insert(out.to_string(), op_fn(*in1_val, *in2_val));
                return true;
            }
        }
        false
    }

    fn exec(&self, wires: &mut FxHashMap<String, u8>) -> bool {
        match self {
            Gate::And(in1, in2, out) => Self::exec_with(in1, in2, out, wires, |i1, i2| i1 & i2),
            Gate::Or(in1, in2, out) => Self::exec_with(in1, in2, out, wires, |i1, i2| i1 | i2),
            Gate::Xor(in1, in2, out) => Self::exec_with(in1, in2, out, wires, |i1, i2| i1 ^ i2),
        }
    }
}

fn build(input: &str) -> (FxHashMap<String, u8>, Vec<Gate>) {
    let mut wires = FxHashMap::default();
    let mut gates = Vec::new();

    let mut it = input.lines();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        let (wire, val) = line.split(": ").collect_tuple().unwrap();
        wires.insert(wire.to_string(), val.parse::<u8>().unwrap());
    }
    for line in it {
        gates.push(Gate::new(line));
    }
    (wires, gates)
}

fn create_z_number(wires: &FxHashMap<String, u8>) -> u64 {
    wires
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted_unstable_by_key(|(k, _)| &k[1..])
        .rev()
        // .inspect(|(k, v)| println!("{k} : {v}"))
        .map(|(_, v)| u64::from(*v))
        .fold(0, |acc, v| acc * 2 + v)
}

fn z_output_number(wires: &FxHashMap<String, u8>, gates: &[Gate]) -> u64 {
    let mut wires = wires.clone();
    loop {
        let mut changed = false;
        for gate in gates {
            changed |= gate.exec(&mut wires);
        }
        if !changed {
            break;
        }
    }
    // println!("{:?}", wires);
    create_z_number(&wires)
}

fn part2(wires: &FxHashMap<String, u8>, gates: &[Gate]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (wires, gates) = build(&input);

    println!("Part 1: {}", z_output_number(&wires, &gates));
    println!("Part 2: {}", part2(&wires, &gates));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1_1() {
        let (wires, gates) = build(INPUT_TEST_1);
        assert_eq!(z_output_number(&wires, &gates), 4);
    }

    #[test]
    fn test_part1_2() {
        let (wires, gates) = build(INPUT_TEST_2);
        assert_eq!(z_output_number(&wires, &gates), 2024);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&wires, &gates), 0);
    }
}
