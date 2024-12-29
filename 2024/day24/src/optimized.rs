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
    // name_to_index: FxHashMap<String, usize>,
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
            // name_to_index,
            initial_wires,
            gates,
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
        let range = match prefix {
            'x' => self.get_range('x'),
            'y' => self.get_range('y'),
            'z' => self.get_range('z'),
            _ => panic!("Unsupported prefix"),
        };

        wires[range]
            .iter()
            .rev()
            .map(|v| u64::from(v.unwrap()))
            .fold(0, |acc, v| acc * 2 + v)
    }

    pub fn z_output_number(&self) -> u64 {
        let mut wires = self.initial_wires.clone();
        self.exec(&mut wires);
        self.extract_number(&wires, 'z')
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
}
