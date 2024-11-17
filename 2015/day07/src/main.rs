use std::{
    collections::HashMap,
    fmt::{self, Debug},
    io::{self, Read},
};

type Wire = String;
type Signal = u16; // Type is important here
type ActiveSignals = HashMap<Wire, Signal>;

trait Gate: Debug {
    fn print_graphviz(&self, id: usize);

    fn exec(&self, signals: &ActiveSignals) -> Option<(Wire, Signal)>;

    fn is_initial_to(&self, _target: &str) -> bool {
        false
    }
}

#[derive(Debug)]
enum Input {
    AsWire(Wire),
    AsSignal(Signal),
}

impl Input {
    fn new(s: &str) -> Input {
        if let Ok(val) = s.parse::<Signal>() {
            Input::AsSignal(val)
        } else {
            Input::AsWire(s.to_string())
        }
    }

    fn signal(&self, signals: &ActiveSignals) -> Option<Signal> {
        match self {
            Input::AsWire(val) => signals.get(val).copied(),
            Input::AsSignal(val) => Some(*val),
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Input::AsWire(v) => write!(f, "{v}"),
            Input::AsSignal(v) => write!(f, "{v}"),
        }
    }
}

// Integer constants to be used in the generics (enum would be cleaner but it's not supported).
const AND: u8 = 1;
const OR: u8 = 2;

#[derive(Debug)]
struct BinaryOp<const TYPE: u8> {
    inputs: [Input; 2],
    target: Wire,
}

impl<const TYPE: u8> BinaryOp<TYPE> {
    fn new(in1: &str, in2: &str, target: &str) -> Self {
        assert!(TYPE == OR || TYPE == AND);
        Self {
            inputs: [Input::new(in1), Input::new(in2)],
            target: target.to_string(),
        }
    }
}

impl<const TYPE: u8> Gate for BinaryOp<TYPE> {
    fn print_graphviz(&self, id: usize) {
        let label = if TYPE == AND { "AND" } else { "OR" };
        println!("    gate{id} [shape=triangle, label=\"{label}\"]");
        println!("    {} -> gate{id}", self.inputs[0]);
        println!("    {} -> gate{id}", self.inputs[1]);
        println!("    gate{id} -> {}", self.target);
    }

    fn exec(&self, signals: &ActiveSignals) -> Option<(Wire, Signal)> {
        if let Some(s1) = self.inputs[0].signal(signals) {
            if let Some(s2) = self.inputs[1].signal(signals) {
                if TYPE == AND {
                    return Some((self.target.clone(), s1 & s2));
                }
                return Some((self.target.clone(), s1 | s2));
            }
        }
        None
    }
}

const LEFT: u8 = 3;
const RIGHT: u8 = 4;

#[derive(Debug)]
struct Shift<const TYPE: u8> {
    input: Input,
    value: u32,
    target: Wire,
}

impl<const DIR: u8> Shift<DIR> {
    fn new(input: &str, value: u32, target: &str) -> Self {
        assert!(DIR == LEFT || DIR == RIGHT);
        Self {
            input: Input::new(input),
            value,
            target: target.to_string(),
        }
    }
}

impl<const DIR: u8> Gate for Shift<DIR> {
    fn print_graphviz(&self, id: usize) {
        let label = if DIR == LEFT { "LSHIFT" } else { "RSHIFT" };
        println!(
            "    gate{id} [shape=rectangle, label=\"{label} {}\"]",
            self.value
        );
        println!("    {} -> gate{id}", self.input);
        println!("    gate{id} -> {}", self.target);
    }

    fn exec(&self, signals: &ActiveSignals) -> Option<(Wire, Signal)> {
        if let Some(s1) = self.input.signal(signals) {
            if DIR == LEFT {
                return Some((self.target.clone(), s1 << self.value));
            }
            return Some((self.target.clone(), s1 >> self.value));
        }
        None
    }
}

const NOT: u8 = 5;
const FORWARD: u8 = 6;

#[derive(Debug)]
struct Unary<const TYPE: u8> {
    input: Input,
    target: Wire,
}

impl<const TYPE: u8> Unary<TYPE> {
    fn new(input: &str, target: &str) -> Self {
        assert!(TYPE == NOT || TYPE == FORWARD);
        Self {
            input: Input::new(input),
            target: target.to_string(),
        }
    }
}

impl<const TYPE: u8> Gate for Unary<TYPE> {
    fn print_graphviz(&self, id: usize) {
        let label = if TYPE == NOT { "NOT" } else { "" };
        println!("    gate{id} [shape=circle, label=\"{label}\"]");
        println!("    {} -> gate{id}", self.input);
        println!("    gate{id} -> {}", self.target);
    }

    fn exec(&self, signals: &ActiveSignals) -> Option<(Wire, Signal)> {
        if let Some(s1) = self.input.signal(signals) {
            if TYPE == NOT {
                return Some((self.target.clone(), !s1));
            }
            return Some((self.target.clone(), s1));
        }
        None
    }
}

#[derive(Debug)]
struct Initial {
    signal: u16,
    target: Wire,
}

impl Initial {
    fn new(signal: u16, target: &str) -> Self {
        Self {
            signal,
            target: target.to_string(),
        }
    }
}

impl Gate for Initial {
    fn print_graphviz(&self, id: usize) {
        println!("    gate{id} [shape=square, label=\"{}\"]", self.signal);
        println!("    gate{id} -> {}", self.target);
    }

    fn exec(&self, _signals: &ActiveSignals) -> Option<(Wire, Signal)> {
        Some((self.target.clone(), self.signal))
    }

    fn is_initial_to(&self, target: &str) -> bool {
        self.target == target
    }
}

type Circuit = Vec<Box<dyn Gate>>;

fn build(input: &str) -> Circuit {
    input
        .lines()
        .map(|line| {
            // println!("{line}");
            let parts: Vec<_> = line.split(" -> ").collect();
            if parts[0].contains(" AND ") {
                let wires: Vec<_> = parts[0].split(" AND ").collect();
                Box::new(BinaryOp::<AND>::new(wires[0], wires[1], parts[1])) as _
            } else if parts[0].contains(" OR ") {
                let wires: Vec<_> = parts[0].split(" OR ").collect();
                Box::new(BinaryOp::<OR>::new(wires[0], wires[1], parts[1])) as _
            } else if parts[0].contains(" LSHIFT ") {
                let p: Vec<_> = parts[0].split(" LSHIFT ").collect();
                Box::new(Shift::<LEFT>::new(p[0], p[1].parse().unwrap(), parts[1])) as _
            } else if parts[0].contains(" RSHIFT ") {
                let p: Vec<_> = parts[0].split(" RSHIFT ").collect();
                Box::new(Shift::<RIGHT>::new(p[0], p[1].parse().unwrap(), parts[1])) as _
            } else if parts[0].starts_with("NOT ") {
                let p = parts[0].trim_start_matches("NOT ");
                Box::new(Unary::<NOT>::new(p, parts[1])) as _
            } else if let Ok(signal) = parts[0].parse() {
                Box::new(Initial::new(signal, parts[1])) as _
            } else {
                Box::new(Unary::<FORWARD>::new(parts[0], parts[1])) as _
            }
        })
        .collect()
}

// View with dot -Tpdf -Kdot input.gv > input.pdf
#[allow(dead_code)]
fn print_graphviz(circuit: &Circuit) {
    println!("digraph {{");
    circuit.iter().enumerate().for_each(|(id, gate)| {
        gate.print_graphviz(id);
    });
    println!("}}");
}

// Have a map Wire -> Signal
// Go through all the gates, asking if these signals produce something
// If they do, add to the map
// When a full loop hasn't produced anything, stop
fn run_circuit(circuit: &Circuit, signals: &mut ActiveSignals, wire_to_monitor: &str) {
    let mut change = true;
    while change || !signals.contains_key(wire_to_monitor) {
        change = false;
        // println!("Looping.....");
        for gate in circuit {
            if let Some(result) = gate.exec(signals) {
                // println!("{:?}  => {:?}", gate, result);
                if let Some(old_key) = signals.insert(result.0, result.1) {
                    if old_key != result.1 {
                        change = true;
                    }
                } else {
                    change = true;
                }
            }
        }
    }
    // println!("{:?}", signals);
}

fn signal_to_wire(circuit: &Circuit, wire_to_monitor: &str) -> Signal {
    let mut signals: ActiveSignals = HashMap::new();
    run_circuit(circuit, &mut signals, wire_to_monitor);
    *signals.get(wire_to_monitor).unwrap()
}

fn signal_to_wire_part2(
    circuit: &mut Circuit,
    signal_on_a: Signal,
    wire_to_monitor: &str,
) -> Signal {
    const B_WIRE: &str = "b";
    let b_idx = circuit
        .iter()
        .position(|g| g.is_initial_to(B_WIRE))
        .unwrap();
    circuit[b_idx] = Box::new(Initial::new(signal_on_a, B_WIRE));
    signal_to_wire(circuit, wire_to_monitor)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut circuit = build(&input);
    // println!("{:#?}", circuit);
    // print_graphviz(&circuit);

    let signal_on_a = signal_to_wire(&circuit, "a");
    println!("Part 1: {signal_on_a}");
    println!(
        "Part 2: {}",
        signal_to_wire_part2(&mut circuit, signal_on_a, "a")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let circuit = build(INPUT_TEST);
        let mut signals: ActiveSignals = HashMap::new();
        run_circuit(&circuit, &mut signals, "d");

        assert_eq!(*signals.get("x").unwrap(), 123);
        assert_eq!(*signals.get("d").unwrap(), 72);
        assert_eq!(*signals.get("y").unwrap(), 456);
        assert_eq!(*signals.get("f").unwrap(), 492);
        assert_eq!(*signals.get("e").unwrap(), 507);
        assert_eq!(*signals.get("h").unwrap(), 65412);
        assert_eq!(*signals.get("g").unwrap(), 114);
        assert_eq!(*signals.get("i").unwrap(), 65079);
        assert_eq!(signal_to_wire(&circuit, "d"), 72);
    }
}
