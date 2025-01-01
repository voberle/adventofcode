//! Initial implementation of the gates and circuit, using strings and all, not optimized.

use std::fmt;

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl Gate {
    pub fn new(line: &str) -> Self {
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

    #[allow(dead_code)]
    pub fn get_inputs(&self) -> (String, String) {
        match self {
            Gate::And(i1, i2, _) | Gate::Or(i1, i2, _) | Gate::Xor(i1, i2, _) => {
                ((*i1).to_string(), (*i2).to_string())
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_output(&self) -> String {
        match self {
            Gate::And(_, _, o) | Gate::Or(_, _, o) | Gate::Xor(_, _, o) => (*o).to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn set_output(&mut self, name: &str) {
        match self {
            Gate::And(_, _, o) | Gate::Or(_, _, o) | Gate::Xor(_, _, o) => *o = name.to_string(),
        }
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gate::And(i1, i2, o) => write!(f, "{i1} AND {i2} -> {o}"),
            Gate::Or(i1, i2, o) => write!(f, "{i1} OR {i2} -> {o}"),
            Gate::Xor(i1, i2, o) => write!(f, "{i1} XOR {i2} -> {o}"),
        }
    }
}

fn find_gate_position(gates: &[Gate], name: &str) -> usize {
    gates
        .iter()
        .position(|gate| match gate {
            Gate::And(_, _, o) | Gate::Or(_, _, o) | Gate::Xor(_, _, o) => o == name,
        })
        .unwrap()
}

pub fn rename_gate_output(gates: &mut [Gate], old_name: &str, new_name: &str) {
    let p = find_gate_position(gates, old_name);
    gates[p].set_output(new_name);
}

fn print_wires(wires: &FxHashMap<String, u8>) {
    for (wire_name, wire_val) in wires {
        println!("{wire_name}: {wire_val}");
    }
}

pub fn print_gates(gates: &[Gate]) {
    for gate in gates {
        println!("{gate}");
    }
}

// Prints the wires and gates in the same format as the input.
#[allow(dead_code)]
pub fn print_input(wires: &FxHashMap<String, u8>, gates: &[Gate]) {
    print_wires(wires);
    println!();
    print_gates(gates);
}

fn extract_number(wires: &FxHashMap<String, u8>, prefix: char) -> u64 {
    wires
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_unstable_by_key(|(k, _)| &k[1..])
        .rev()
        .map(|(_, v)| u64::from(*v))
        .fold(0, |acc, v| acc * 2 + v)
}

fn exec(gates: &[Gate], wires: &mut FxHashMap<String, u8>) {
    loop {
        let mut changed = false;
        for gate in gates {
            changed |= gate.exec(wires);
        }
        if !changed {
            break;
        }
    }
}

#[allow(dead_code)]
pub fn z_output_number(wires: &FxHashMap<String, u8>, gates: &[Gate]) -> u64 {
    let mut wires = wires.clone();
    exec(gates, &mut wires);
    extract_number(&wires, 'z')
}

// Set the wires value to the specified number, in binary.
fn set_wires(wires: &mut FxHashMap<String, u8>, number: u64, digits_count: usize, prefix: char) {
    let mut n = number;
    for index in 0..digits_count {
        let b = n & 1;
        wires.insert(format!("{prefix}{index:0>2}"), u8::try_from(b).unwrap());
        n >>= 1;
    }
}

#[allow(dead_code)]
pub fn test_addition(gates: &[Gate], n1: u64, n2: u64) -> bool {
    let mut wires = FxHashMap::default();
    set_wires(&mut wires, n1, 45, 'x');
    set_wires(&mut wires, n2, 45, 'y');
    exec(gates, &mut wires);
    let result = extract_number(&wires, 'z');

    // println!("{} + {}", n1, n2);
    // println!("{:#045b}", n1);
    // println!("{:#045b}", n2);
    // println!("{:#045b}", n1 + n2);
    // println!("{:#045b}", result);
    n1 + n2 == result
}

pub fn create_swapped_wires_string(gates_to_swap: &[(&str, &str)]) -> String {
    gates_to_swap
        .iter()
        .flat_map(|(n1, n2)| [n1, n2])
        .sorted_unstable()
        .join(",")
}

pub fn swap_wires(gates: &[Gate], gates_to_swap: &[(&str, &str)]) -> Vec<Gate> {
    let mut gates = gates.to_vec();

    for (n1, n2) in gates_to_swap {
        let p1 = find_gate_position(&gates, n1);
        let p2 = find_gate_position(&gates, n2);
        gates[p1].set_output(n2);
        gates[p2].set_output(n1);
    }

    gates
}

// The approach here is to observe that a bunch of Z wires were connected to the wrong gate type.
// This gives already 3 pairs of wires to swap.
// The last one can just be found via brute-forcing.
#[allow(dead_code)]
pub fn swap_wires_investigation(gates: &[Gate]) -> String {
    // All the Z wires need to be output of an XOR.
    let wrong_z_gates = gates
        .iter()
        .filter(|gate| match gate {
            Gate::Xor(_, _, _) => false,
            Gate::And(_, _, o) | Gate::Or(_, _, o) => o.starts_with('z'),
        })
        .collect_vec();
    println!("Incorrect Z wires:");
    for g in wrong_z_gates {
        println!("{g}");
    }
    println!();

    // We found 3 pairs to swap.
    let gates_to_swap = [("z14", "vhm"), ("z27", "mps"), ("z39", "msq")];
    let mut gates = swap_wires(gates, &gates_to_swap);

    // Brute force the last pair.
    let mut last_pair_to_swap = None;
    'outer: for p1 in 0..gates.len() {
        println!("{p1}..");
        for p2 in 0..gates.len() {
            if p1 == p2 {
                continue;
            }
            let mut copy_gates = gates.clone();
            let n1 = copy_gates[p1].get_output();
            let n2 = copy_gates[p2].get_output();
            copy_gates[p1].set_output(&n2);
            copy_gates[p2].set_output(&n1);
            if test_addition(&copy_gates, 41421, 35252)
                && test_addition(&copy_gates, 234, 6436)
                && test_addition(&copy_gates, 4_398_046_511_103, 0)
            {
                println!("FOUND PAIR TO SWAP: {n1} {n2}");
                // Real swap
                gates[p1].set_output(&n2);
                gates[p2].set_output(&n1);
                last_pair_to_swap = Some((n1.to_string(), n2.to_string()));
                break 'outer;
            }
        }
    }

    // Tests it's ok.
    println!("Test: {}", test_addition(&gates, 41421, 35252));
    println!("Test: {}", test_addition(&gates, 234, 6436));
    println!("Test: {}", test_addition(&gates, 235, 6436));
    println!("Test: {}", test_addition(&gates, 0, 0));
    // 0b0111111111111111111111111111111111111111111
    println!("Test: {}", test_addition(&gates, 4_398_046_511_103, 0));
    println!("Test: {}", test_addition(&gates, 3, 0));

    let mut result = gates_to_swap.to_vec();
    let pair = last_pair_to_swap.unwrap();
    result.push((&pair.0, &pair.1));
    create_swapped_wires_string(&result)
}

#[cfg(test)]
mod tests {
    use crate::build;
    use crate::tests::INPUT_TEST_1;
    use crate::tests::INPUT_TEST_2;

    use super::*;

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
    fn test_set_wires() {
        let mut wires = FxHashMap::default();
        set_wires(&mut wires, 13, 4, 'x');
        assert_eq!(*wires.get("x00").unwrap(), 1);
        assert_eq!(*wires.get("x01").unwrap(), 0);
        assert_eq!(*wires.get("x02").unwrap(), 1);
        assert_eq!(*wires.get("x03").unwrap(), 1);
        assert!(!wires.contains_key("x04"));
    }
}
