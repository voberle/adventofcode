//! Generation of a circuit implementing a working adder.

use fxhash::{FxHashMap, FxHashSet};

use crate::circuit::{print_gates, print_input, rename_gate_output, Gate};

fn gen_name(c: char, n: usize) -> String {
    if c == 'x' || c == 'y' {
        format!("{c}{n:0>2}")
    } else {
        format!("${c}{n:0>2}")
    }
}

// Names: Letters, followed by numbers.
fn generate_adder(n: usize, carry_over: &str) -> (String, Vec<String>, Vec<Gate>) {
    let x = gen_name('x', n);
    let y = gen_name('y', n);
    let z = gen_name('z', n);
    let a = gen_name('a', n);
    let b = gen_name('b', n);
    let c = gen_name('c', n);
    let d = gen_name('d', n);
    let wires = vec![
        x.clone(),
        y.clone(),
        z.clone(),
        a.clone(),
        b.clone(),
        c.clone(),
        d.clone(),
    ];
    let gates = vec![
        Gate::Xor(x.to_string(), y.to_string(), a.to_string()),
        Gate::And(x.to_string(), y.to_string(), b.to_string()),
        Gate::Xor(carry_over.to_string(), a.to_string(), z.to_string()),
        Gate::And(carry_over.to_string(), a.to_string(), c.to_string()),
        Gate::Or(c.to_string(), b.to_string(), d.to_string()),
    ];
    (d.to_string(), wires, gates)
}

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
        Gate::Xor(x.to_string(), y.to_string(), z.to_string()),
        Gate::And(x.to_string(), y.to_string(), carry.to_string()),
    ]);

    for n in 1..bits_count {
        let (c, w, g) = generate_adder(n, &carry);
        wires.extend(w);
        gates.extend(g);
        carry = c;
    }

    // Rename last carry to z
    let last_z = gen_name('z', bits_count);
    let p = wires.iter().position(|w| *w == carry).unwrap();
    wires[p] = last_z.to_string();

    rename_gate_output(&mut gates, &carry, &last_z);

    let wires_map = wires.iter().map(|w| (w.clone(), 0)).collect();
    (wires_map, gates)
}

fn rename_gate(gates: &mut [Gate], old_name: &str, new_name: &str){
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

pub fn find_swapped_wires(wires: &FxHashMap<String, u8>, gates: &[Gate]) {
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

    // Go through the working circuit. If there are correct wires as input,
    // then rename the output if needed.
    loop {
        let mut something_to_rename = None;

        for w_gate in & working_adder {
            let (in1, in2) = w_gate.get_inputs();
            if correct_wires.contains(&in1) && correct_wires.contains(&in2) {
                let current_name = w_gate.get_output();
                // if !current_name.ends_with(|c: char| c.is_ascii_digit()) {
                if !current_name.starts_with('$') {
                    // Already correct
                    continue;
                }

                // Find gate by inputs
                if let Some(p) = gates.iter().position(|gate| 
                    // matches!(w_gate, Gate::And(i1, i2, _) if *i1 == in1 && *i2 == in2) ||
                    // matches!(w_gate, Gate::Xor(i1, i2, _) if *i1 == in1 && *i2 == in2) ||
                    // matches!(w_gate, Gate::Or(i1, i2, _) if *i1 == in1 && *i2 == in2)
                    match gate {
                        Gate::And(i1, i2, _) => matches!(w_gate, Gate::And(_, _, _)) && ((*i1 == in1 && *i2 == in2) || (*i1 == in2 && *i2 == in1)),
                        Gate::Or(i1, i2, _) =>matches!(w_gate, Gate::Or(_, _, _)) && ((*i1 == in1 && *i2 == in2) || (*i1 == in2 && *i2 == in1)),
                        Gate::Xor(i1, i2, _) =>matches!(w_gate, Gate::Xor(_, _, _)) && ((*i1 == in1 && *i2 == in2) || (*i1 == in2 && *i2 == in1)),
                    }
                ) {
                    let name_to_swap = gates[p].get_output();

                    if current_name == name_to_swap {
                        continue;
                    }

                    something_to_rename = Some((current_name, name_to_swap));
                    break;
                }
            }
        }

        if let Some((old, new)) = something_to_rename {
            // Rename
            // println!("Renaming {} => {}", old, new);
            rename_gate(&mut working_adder, &old, &new);

            correct_wires.insert(new.to_string());
        } else {
            break;
        }
    }

    print_gates(&working_adder);
}
