//! Generation of a circuit implementing a working adder.
#![allow(clippy::many_single_char_names)]

use fxhash::{FxHashMap, FxHashSet};

use crate::circuit::{Gate, rename_gate_output};

// Generate a name for the wires.
// We prefix the ones that may be swapped with $.
fn gen_name(c: char, n: usize) -> String {
    if c == 'x' || c == 'y' {
        format!("{c}{n:0>2}")
    } else {
        format!("${c}{n:0>2}")
    }
}

// Generate an adder for 2 bits.
// Names: Letters, followed by numbers.
fn generate_adder(n: usize, carry_over: &str) -> (Vec<String>, Vec<Gate>, String) {
    let x = gen_name('x', n);
    let y = gen_name('y', n);
    let z = gen_name('z', n);
    let a = gen_name('a', n);
    let b = gen_name('b', n);
    let c = gen_name('c', n);
    let d = gen_name('d', n);
    let gates = vec![
        Gate::Xor(x.clone(), y.clone(), a.clone()),
        Gate::And(x.clone(), y.clone(), b.clone()),
        Gate::Xor(carry_over.to_string(), a.clone(), z.clone()),
        Gate::And(carry_over.to_string(), a.clone(), c.clone()),
        Gate::Or(c.clone(), b.clone(), d.clone()),
    ];
    let carry_over = d.clone();
    (vec![x, y, z, a, b, c, d], gates, carry_over)
}

// Generate a functional adder circuit.
fn generate_circuit(bits_count: usize) -> (FxHashMap<String, u8>, Vec<Gate>) {
    let mut wires: Vec<String> = Vec::new();
    let mut gates: Vec<Gate> = Vec::new();

    // Initial gate
    let n = 0;
    let x = gen_name('x', n);
    let y = gen_name('y', n);
    let z = gen_name('z', n);
    let mut carry = gen_name('b', n);
    wires.extend([x.clone(), y.clone(), z.clone(), carry.clone()]);
    gates.extend([
        Gate::Xor(x.clone(), y.clone(), z.clone()),
        Gate::And(x.clone(), y.clone(), carry.clone()),
    ]);

    for n in 1..bits_count {
        let (w, g, c) = generate_adder(n, &carry);
        wires.extend(w);
        gates.extend(g);
        carry = c;
    }

    // Rename last carry to z
    let last_z = gen_name('z', bits_count);
    let p = wires.iter().position(|w| *w == carry).unwrap();
    wires[p].clone_from(&last_z);

    rename_gate_output(&mut gates, &carry, &last_z);

    let wires_map = wires.iter().map(|w| (w.clone(), 0)).collect();
    (wires_map, gates)
}

// Renames all instances of the wire in the circuit.
fn rename(gates: &mut [Gate], old_name: &str, new_name: &str) {
    for gate in gates {
        match gate {
            Gate::And(i1, i2, o) | Gate::Or(i1, i2, o) | Gate::Xor(i1, i2, o) => {
                if old_name == i1 {
                    *i1 = new_name.to_string();
                }
                if old_name == i2 {
                    *i2 = new_name.to_string();
                }
                if old_name == o {
                    *o = new_name.to_string();
                }
            }
        }
    }
}

// To find the swapped wires, we first generate a working adder.
// Then we go through the working circuit. If there are correct wires as input,
// we rename the output, and so on.
// We do this until no renaming can happen anymore. This means that we hit a swapped wire
// in the input. By comparing the working adder with the input at this stage, we can find
// the swapped wire. We swap them and restart the process.
pub fn find_swapped_wires(gates: &[Gate]) {
    // Given our circuit with swapped wires, and a working adder,
    // we rename all gates we can and find the remaining wrong ones.

    const BITS_COUNT: usize = 45;
    let (_, mut working_adder) = generate_circuit(BITS_COUNT);

    let mut correct_wires = FxHashSet::default();
    // All the x and y wires are correct.
    for n in 0..BITS_COUNT {
        correct_wires.insert(gen_name('x', n));
        correct_wires.insert(gen_name('y', n));
    }

    loop {
        let mut something_to_rename = None;

        for w_gate in &working_adder {
            let (in1, in2) = w_gate.get_inputs();
            if correct_wires.contains(&in1) && correct_wires.contains(&in2) {
                let current_name = w_gate.get_output();
                if !current_name.starts_with('$') {
                    // Already correct
                    continue;
                }

                // Find gate in the original circuit: Type of gate and inputs must match.
                if let Some(p) = gates.iter().position(|gate| match gate {
                    Gate::And(i1, i2, _) => {
                        matches!(w_gate, Gate::And(_, _, _))
                            && ((*i1 == in1 && *i2 == in2) || (*i1 == in2 && *i2 == in1))
                    }
                    Gate::Or(i1, i2, _) => {
                        matches!(w_gate, Gate::Or(_, _, _))
                            && ((*i1 == in1 && *i2 == in2) || (*i1 == in2 && *i2 == in1))
                    }
                    Gate::Xor(i1, i2, _) => {
                        matches!(w_gate, Gate::Xor(_, _, _))
                            && ((*i1 == in1 && *i2 == in2) || (*i1 == in2 && *i2 == in1))
                    }
                }) {
                    let name_to_swap = gates[p].get_output();

                    if current_name == name_to_swap {
                        continue;
                    }

                    // The borrow checker won't let us rename here, so we get out of the loop and rename there.
                    something_to_rename = Some((current_name, name_to_swap));
                    break;
                }
            }
        }

        if let Some((old, new)) = something_to_rename {
            rename(&mut working_adder, &old, &new);

            correct_wires.insert(new.clone());
        } else {
            break;
        }
    }

    // print_gates(&working_adder);
}
