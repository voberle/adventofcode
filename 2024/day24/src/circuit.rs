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
    fn get_output(&mut self) -> String {
        match self {
            Gate::And(_, _, o) | Gate::Or(_, _, o) | Gate::Xor(_, _, o) => (*o).to_string(),
        }
    }

    #[allow(dead_code)]
    fn set_output(&mut self, name: &str) {
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

// Prints the wires and gates in the same format as the input.
#[allow(dead_code)]
fn print_input(wires: &FxHashMap<String, u8>, gates: &[Gate]) {
    for (wire_name, wire_val) in wires {
        println!("{wire_name}: {wire_val}");
    }
    println!();
    for gate in gates {
        println!("{gate}");
    }
}

fn extract_number(wires: &FxHashMap<String, u8>, prefix: char) -> u64 {
    wires
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_unstable_by_key(|(k, _)| &k[1..])
        .rev()
        // .inspect(|(k, v)| println!("{k} : {v}"))
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
fn set_wires(wires: &mut FxHashMap<String, u8>, number: u64, prefix: char) {
    let mut n = number;
    for index in 0.. {
        let b = n & 1;
        wires.insert(format!("{}{:0>2}", prefix, index), u8::try_from(b).unwrap());
        n = n >> 1;
        if n == 0 {
            break;
        }
    }
}

#[allow(dead_code)]
fn test_addition(gates: &[Gate], n1: u64, n2: u64) -> bool {
    let mut wires = FxHashMap::default();
    set_wires(&mut wires, n1, 'x');
    set_wires(&mut wires, n2, 'y');
    exec(gates, &mut wires);
    let result = extract_number(&wires, 'z');
    // println!("{} + {}", n1, n2);
    // println!("{:#045b}", n1);
    // println!("{:#045b}", n2);
    // println!("{:#045b}", n1 + n2);
    // println!("{:#045b}", result);
    n1 + n2 == result
    // (n1 + n2) & 3 == result & 3
}

fn find_gate_position(gates: &[Gate], name: &str) -> usize {
    gates
        .iter()
        .position(|gate| match gate {
            Gate::And(_, _, o) | Gate::Or(_, _, o) | Gate::Xor(_, _, o) => o == name,
        })
        .unwrap()
}

pub fn swap_gates(wires: &FxHashMap<String, u8>, gates: &[Gate]) -> String {
    let mut gates = gates.to_vec();

    let gates_to_swap = [
        ("z14", "vhm"),
        ("z27", "mps"),
        ("z39", "msq"),

        ("z01", "gww"),

        // First 3 found by analyzing graph.
        // ("z14", "vhm"),
        // ("z27", "mps"),
        // ("z39", "msq"),
        // Last one brute forced.
        // ("z42", "rvm"),
    ];
    for (n1, n2) in gates_to_swap {
        let p1 = find_gate_position(&gates, n1);
        let p2 = find_gate_position(&gates, n2);
        gates[p1].set_output(n2);
        gates[p2].set_output(n1);
    }


    'outer:
    for p1 in 0..gates.len() {
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
            if test_addition(&copy_gates, 41421, 35252) && test_addition(&copy_gates, 234, 6436) {
                println!("SWAP {} {}", n1, n2);
                break 'outer;
            }
        }
    }

//     let candidates = [
// "fgw",
// "z00",
// "pjh",
// "bgb",
// "wwp",
// "gww",
// "dng",
// "z01",
// "z02",
//     ].iter().map(|n| find_gate_position(&gates, n)).collect_vec();
//     'outer:
//     for &p1 in &candidates {
//         println!("{p1}..");
//         for &p2 in &candidates {
//             if p1 == p2 {
//                 continue;
//             }
//             let mut copy_gates = gates.clone();
//             let n1 = copy_gates[p1].get_output();
//             let n2 = copy_gates[p2].get_output();
//             copy_gates[p1].set_output(&n2);
//             copy_gates[p2].set_output(&n1);
//             if (0..4).permutations(2).all(|pair| test_addition(&copy_gates, pair[0], pair[1]))
//             {
//                 println!("SWAP {} {}", n1, n2);
//                 break 'outer;
//             }
//         }
//     }


    // print_input(&wires, &gates);

    println!("Test: {}", test_addition(&gates, 41421, 35252));
    println!("Test: {}", test_addition(&gates, 234, 6436));
    println!("Test: {}", test_addition(&gates, 235, 6436));
    println!(
        "Test: {}",
        test_addition(
            &gates,
            0b0000000000000000000000000000000000000000000,
            0b0000000000000000000000000000000000000000000
        )
    );
    println!(
        "Test: {}",
        test_addition(
            &gates,
            0b0111111111111111111111111111111111111111111,
            0b0000000000000000000000000000000000000000000
        )
    );
    println!("Test: {}", test_addition(&gates, 3, 0));

    gates_to_swap
        .iter()
        .flat_map(|(n1, n2)| [n1, n2])
        .sorted_unstable()
        .join(",")
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
        set_wires(&mut wires, 13, 'x');
        assert_eq!(*wires.get("x00").unwrap(), 1);
        assert_eq!(*wires.get("x01").unwrap(), 0);
        assert_eq!(*wires.get("x02").unwrap(), 1);
        assert_eq!(*wires.get("x03").unwrap(), 1);
        assert!(!wires.contains_key("x04"));
    }
}
