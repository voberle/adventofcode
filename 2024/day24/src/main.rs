use std::io::{self, Read};

use circuit::Gate;
use fxhash::FxHashMap;
use itertools::Itertools;
use optimized::Circuit;

mod circuit;
mod optimized;

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

// fn part2(wires: &FxHashMap<String, u8>, gates: &[Gate]) -> i64 {
//     0
// }

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (wires, gates) = build(&input);

    // for _ in 0..1000 {
    //     circuit::z_output_number(&wires, &gates);
    // }
    // let circuit = Circuit::new(&wires, &gates);
    // for _ in 0..1000 {
    //     circuit.z_output_number();
    // }

    // println!("Part 1: {}", circuit::z_output_number(&wires, &gates));

    let circuit = Circuit::new(&wires, &gates);
    println!("Part 1: {}", circuit.z_output_number());

    println!("Part 2: {}", circuit::swap_gates(&wires, &gates));
}

#[cfg(test)]
mod tests {
    pub const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    pub const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
}
