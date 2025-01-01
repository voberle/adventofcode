//! Optimized implementation of the circuit.

use std::ops::Range;

use fxhash::FxHashMap;
use itertools::Itertools;

use crate::circuit::Gate;

// Values are indexes in the wires vector.
struct OptGate {
    input1: usize,
    input2: usize,
    output: usize,
    function: fn(u8, u8) -> u8,
    // gate_type: Gate,
}

impl OptGate {
    fn new(gate: &Gate, name_to_index: &FxHashMap<String, usize>) -> Self {
        let (input1, input2, output) = match gate {
            Gate::And(i1, i2, o) | Gate::Or(i1, i2, o) | Gate::Xor(i1, i2, o) => (
                *name_to_index.get(i1).unwrap(),
                *name_to_index.get(i2).unwrap(),
                *name_to_index.get(o).unwrap(),
            ),
        };
        let function = match gate {
            Gate::And(_, _, _) => |i1, i2| i1 & i2,
            Gate::Or(_, _, _) => |i1, i2| i1 | i2,
            Gate::Xor(_, _, _) => |i1, i2| i1 ^ i2,
        };
        Self {
            input1,
            input2,
            output,
            function,
            // gate_type: gate.clone(),
        }
    }

    fn exec(&self, wires: &mut [Option<u8>]) -> bool {
        if wires[self.output].is_some() {
            return false;
        }
        if let Some(in1_val) = wires[self.input1] {
            if let Some(in2_val) = wires[self.input2] {
                wires[self.output] = Some((self.function)(in1_val, in2_val));
                return true;
            }
        }
        false
    }
}

pub struct Circuit {
    index_to_name: Vec<String>,
    name_to_index: FxHashMap<String, usize>,
    initial_wires: Vec<Option<u8>>,
    gates: Vec<OptGate>,
}

impl Circuit {
    fn get_range(&self, prefix: char) -> Range<usize> {
        let range_start = self
            .index_to_name
            .iter()
            .position(|n| n.starts_with(prefix))
            .unwrap();
        let range_end = range_start
            + self.index_to_name[range_start..]
                .iter()
                .position(|n| !n.starts_with(prefix))
                .unwrap_or(self.index_to_name[range_start..].len());
        range_start..range_end
    }

    #[allow(dead_code)]
    pub fn new(wires_string: &FxHashMap<String, u8>, gates_string: &[Gate]) -> Self {
        // Not all wires are listed in the wires list, so we need to look into the gates list to get them.
        let index_to_name = gates_string
            .iter()
            .flat_map(|gate| match gate {
                Gate::And(i1, i2, o) | Gate::Or(i1, i2, o) | Gate::Xor(i1, i2, o) => [i1, i2, o],
            })
            .unique()
            .cloned()
            .sorted_unstable()
            .collect_vec();

        let mut name_to_index = FxHashMap::default();
        for (i, name) in index_to_name.iter().enumerate() {
            name_to_index.insert(name.to_string(), i);
        }

        let initial_wires = index_to_name
            .iter()
            .map(|name| wires_string.get(name).copied())
            .collect_vec();

        let gates = gates_string
            .iter()
            .map(|gate_string| OptGate::new(gate_string, &name_to_index))
            .collect_vec();

        Circuit {
            index_to_name,
            name_to_index,
            initial_wires,
            gates,
        }
    }

    #[allow(dead_code)]
    fn get_wire_val(&self, name: &str) -> Option<u8> {
        if let Some(i) = self.name_to_index.get(name) {
            self.initial_wires[*i]
        } else {
            None
        }
    }

    fn exec(&self, wires: &mut [Option<u8>]) {
        loop {
            let mut changed = false;
            for gate in &self.gates {
                changed |= gate.exec(wires);
            }
            if !changed {
                break;
            }
        }
    }

    fn extract_number(&self, wires: &[Option<u8>], prefix: char) -> u64 {
        wires[self.get_range(prefix)]
            .iter()
            .rev()
            .map(|v| u64::from(v.unwrap()))
            .fold(0, |acc, v| acc * 2 + v)
    }

    #[allow(dead_code)]
    pub fn z_output_number(&self) -> u64 {
        let mut wires = self.initial_wires.clone();
        self.exec(&mut wires);
        self.extract_number(&wires, 'z')
    }

    // Set the wires value to the specified number, in binary.
    fn set_wires(&self, wires: &mut Vec<Option<u8>>, number: u64, prefix: char) {
        // Reset the wires.
        *wires = vec![None; self.initial_wires.len()];

        let range = self.get_range(prefix);
        let mut n = number;
        for index in range {
            let v = u8::try_from(n & 1).unwrap();
            wires[index] = Some(v);
            n >>= 1;
        }
    }

    #[allow(dead_code)]
    fn test_addition(&self, n1: u64, n2: u64) -> bool {
        let mut wires = self.initial_wires.clone();
        self.set_wires(&mut wires, n1, 'x');
        self.set_wires(&mut wires, n2, 'y');

        self.exec(&mut wires);
        let result = self.extract_number(&wires, 'z');
        n1 + n2 == result
    }
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
        let circuit = Circuit::new(&wires, &gates);
        assert_eq!(circuit.z_output_number(), 4);
    }

    #[test]
    fn test_part1_2() {
        let (wires, gates) = build(INPUT_TEST_2);
        let circuit = Circuit::new(&wires, &gates);
        assert_eq!(circuit.z_output_number(), 2024);
    }

    #[test]
    fn test_set_wires() {
        let (wires, gates) = build(INPUT_TEST_2);
        let circuit = Circuit::new(&wires, &gates);

        let mut wires = circuit.initial_wires.clone();
        circuit.set_wires(&mut wires, 13, 'x');
        // println!("{:?}", circuit.index_to_name);
        // println!("{:?}", wires);

        assert_eq!(circuit.get_wire_val("x00").unwrap(), 1);
        assert_eq!(circuit.get_wire_val("x01").unwrap(), 0);
        assert_eq!(circuit.get_wire_val("x02").unwrap(), 1);
        assert_eq!(circuit.get_wire_val("x03").unwrap(), 1);
        assert_eq!(circuit.get_wire_val("x04").unwrap(), 0);
    }
}
