use std::io::{self, Read};

use circuit::{create_swapped_wires_string, swap_wires, Gate};
use fxhash::FxHashMap;
use itertools::Itertools;

mod circuit;
mod generate;
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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (wires, gates) = build(&input);

    // Comparing the performance of optimized implementation.
    // for _ in 0..1000 {
    //     circuit::z_output_number(&wires, &gates);
    // }
    // let circuit = optimized::Circuit::new(&wires, &gates);
    // for _ in 0..1000 {
    //     circuit.z_output_number();
    // }

    println!("Part 1: {}", circuit::z_output_number(&wires, &gates));

    // This is the version where we analyzed the circuit by hand + a bit of brute force.
    // println!("Part 2: {}", circuit::swap_wires_investigation(&gates));

    // This is the version where we generate the correct adder and use it to find the swapped wires.
    let wires_to_swap = [
        ("qwf", "cnk"),
        ("z14", "vhm"),
        ("z27", "mps"),
        ("z39", "msq"),
    ];
    let gates = swap_wires(&gates, &wires_to_swap);
    generate::find_swapped_wires(&gates);
    println!("Part 2: {}", create_swapped_wires_string(&wires_to_swap));
}

#[cfg(test)]
mod tests {
    pub const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    pub const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
}
