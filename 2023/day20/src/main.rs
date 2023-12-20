// https://adventofcode.com/2023/day/20

// Traits:
// exec -> Returns bool if something was executed
//          Allows to loop through all modules and know if any did something
// is_original -> Tells if module is in original state

use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, BufRead},
};

use lazy_static::lazy_static;
use regex::Regex;

const LOW: bool = false;
const HIGH: bool = true;

trait Module: Debug {
    // Returns the name of the module.
    fn get_name(&self) -> &str;

    // Executes a pulse, received as ("sender", "low/high") pairs.
    // Returns the list of pulses sent as pairs ("destination", "low/high")
    fn exec(&mut self, from: (String, bool)) -> Vec<(String, bool)>;
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    next: String,
}

impl FlipFlop {
    fn new(name: String, next: String) -> Self {
        Self {
            name,
            state: LOW,
            next,
        }
    }

    fn build(line: &str) -> Self {
        lazy_static! {
            // %a -> b
            static ref FLIPFLOP: Regex = Regex::new(r"%(\w+) -> (\w+)").expect("Error parsing regex");
        }
        let captures = FLIPFLOP.captures(line).unwrap();
        Self::new(captures[1].to_string(), captures[2].to_string())
    }

    fn flip(&mut self) {
        self.state ^= true;
    }
}

impl Module for FlipFlop {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn exec(&mut self, from: (String, bool)) -> Vec<(String, bool)> {
        if from.1 == HIGH {
            // HIGH: ignore it
            Vec::new()
        } else {
            // LOW: flip and sends pulse matching state
            self.flip();
            vec![(self.next.clone(), self.state)]
        }
    }
}

#[test]
fn test_flipflop() {
    let mut m = FlipFlop::build("%a -> b");
    assert_eq!(m.get_name(), "a");
    assert_eq!(m.next, "b");
    assert_eq!(m.state, LOW);
    assert_eq!(m.exec(("irrelevant".to_string(), HIGH)), Vec::new());
    assert_eq!(
        m.exec(("irrelevant".to_string(), LOW)),
        [("b".to_string(), HIGH)]
    );
    assert_eq!(
        m.exec(("irrelevant".to_string(), LOW)),
        [("b".to_string(), LOW)]
    );
    assert_eq!(m.exec(("irrelevant".to_string(), HIGH)), Vec::new());
    assert_eq!(
        m.exec(("irrelevant".to_string(), LOW)),
        [("b".to_string(), HIGH)]
    );
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    previous_pulse: HashMap<String, bool>,
    next: String,
    // Queue should be a pair "connected input module" => "pulse"
}

impl Conjunction {
    fn new(name: String, next: String) -> Self {
        Self {
            name,
            previous_pulse: HashMap::new(),
            next,
        }
    }

    fn build(line: &str) -> Self {
        lazy_static! {
            // &inv -> a
            static ref CONJUNCTION: Regex = Regex::new(r"&(\w+) -> (\w+)").expect("Error parsing regex");
        }
        let captures = CONJUNCTION.captures(line).unwrap();
        Self::new(captures[1].to_string(), captures[2].to_string())
    }
}

impl Module for Conjunction {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn exec(&mut self, from: (String, bool)) -> Vec<(String, bool)> {
        // Update that memory to this input
        self.previous_pulse
            .entry(from.0)
            .and_modify(|mem| *mem = from.1)
            .or_insert(from.1);
        // If previous_pulse map is empty, it means we haven't received anything yet, so assuming all inputs are low
        if !self.previous_pulse.is_empty() && self.previous_pulse.values().all(|mem| *mem) {
            // if all input are high, send low pulse
            vec![(self.next.clone(), LOW)]
        } else {
            // else send high pulse
            vec![(self.next.clone(), HIGH)]
        }
    }
}

#[test]
fn test_conjunction() {
    let mut m = Conjunction::build("&inv -> b");
    assert_eq!(m.get_name(), "inv");
    assert_eq!(m.next, "b");
    assert!(m.previous_pulse.is_empty());
    assert_eq!(m.exec(("a".to_string(), HIGH)), [("b".to_string(), LOW)]);
    assert_eq!(m.exec(("c".to_string(), LOW)), [("b".to_string(), HIGH)]);
}

#[derive(Debug)]
struct Broadcast {
    next_modules: Vec<String>,
}

impl Broadcast {
    const NAME: &str = "broadcaster";

    fn new(next: Vec<String>) -> Self {
        Self { next_modules: next }
    }

    fn build(line: &str) -> Self {
        lazy_static! {
            // broadcaster -> a, b, c
            static ref BROADCAST: Regex = Regex::new(r"broadcaster -> (.+)").expect("Error parsing regex");
        }
        let captures = BROADCAST.captures(line).unwrap();
        Self::new(
            captures[1]
                .to_string()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )
    }
}

impl Module for Broadcast {
    fn get_name(&self) -> &str {
        &Self::NAME
    }

    fn exec(&mut self, from: (String, bool)) -> Vec<(String, bool)> {
        self.next_modules
            .iter()
            .map(|n| (n.clone(), from.1))
            .collect()
    }
}

#[test]
fn test_broadcast() {
    let mut m = Broadcast::build("broadcaster -> a, b, c");
    assert_eq!(m.get_name(), "broadcaster");
    assert_eq!(m.next_modules, vec!["a", "b", "c"]);
    assert_eq!(
        m.exec(("irrelevant".to_string(), HIGH)),
        [
            ("a".to_string(), HIGH),
            ("b".to_string(), HIGH),
            ("c".to_string(), HIGH)
        ]
    );
}

type Configuration = HashMap<String, Box<dyn Module>>;

fn press_button(configuration: &Configuration) {
    // configuration.get(Broadcast::NAME).unwrap().send(false);
}

fn total_pulses_count_product(configuration: &Configuration) -> u64 {
    0
}

fn build_configuration<R>(reader: &mut R) -> Configuration
where
    R: BufRead,
{
    let mut configuration: Configuration = HashMap::new();

    for l in reader.lines() {
        let line = l.unwrap();
        let m: Box<dyn Module> = if line.starts_with('%') {
            Box::new(FlipFlop::build(&line))
        } else if line.starts_with('&') {
            Box::new(Conjunction::build(&line))
        } else if line.starts_with(Broadcast::NAME) {
            Box::new(Broadcast::build(&line))
        } else {
            // Note that Button module is not listed in the configuration
            panic!("Invalid line: {}", line)
        };
        configuration.insert(m.get_name().to_string(), m);
    }
    configuration
}

fn main() {
    let stdin = io::stdin();

    let configuration = build_configuration(&mut stdin.lock());
    println!("{:#?}", configuration);

    println!("Part 1: {}", total_pulses_count_product(&configuration));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader1 = BufReader::new(File::open("resources/input_test_1").unwrap());
        let configuration1 = build_configuration(&mut reader1);

        assert_eq!(total_pulses_count_product(&configuration1), 32000000);

        let mut reader2 = BufReader::new(File::open("resources/input_test_2").unwrap());
        let configuration2 = build_configuration(&mut reader2);

        assert_eq!(total_pulses_count_product(&configuration2), 11687500);
    }
}
