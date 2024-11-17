// https://adventofcode.com/2023/day/20

use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    io::{self, BufRead},
};

const LOW: bool = false;
const HIGH: bool = true;

// Information about a pulse: Sender, receiver and value.
#[derive(Debug, PartialEq)]
struct Pulse {
    from: String,
    value: bool,
    to: String,
}

impl Pulse {
    fn new(from: &str, value: bool, to: &str) -> Self {
        Self {
            from: from.to_string(),
            value,
            to: to.to_string(),
        }
    }
}

// Interface shared by all the modules.
trait Module: Debug {
    fn get_type(&self) -> &str;

    fn get_common(&self) -> &ModuleCommon;

    fn get_name(&self) -> &str {
        &self.get_common().name
    }

    fn get_next_modules(&self) -> &[String] {
        &self.get_common().next_modules
    }

    // Executes a pulse.
    // Returns the list of pulses sent.
    fn exec(&mut self, pulse: &Pulse) -> Vec<Pulse>;

    // Checks if the module is in its initial state.
    fn is_initial_state(&self) -> bool;

    // Only needed by Conjunction module
    fn update_previous_pulse(&mut self, _previous_pulse: HashMap<String, bool>) {}

    // Sets the module back to its initial state
    fn reset(&mut self);
}

// Implementation shared by all modules.
#[derive(Debug)]
struct ModuleCommon {
    name: String,
    next_modules: Vec<String>,
}

impl ModuleCommon {
    fn new(name: String, next_modules: Vec<String>) -> Self {
        Self { name, next_modules }
    }

    fn build(line: &str) -> Self {
        // module -> a, b, c
        let src_dest: Vec<String> = line
            .split(" -> ")
            .map(std::string::ToString::to_string)
            .collect();
        Self::new(
            src_dest[0].clone(),
            src_dest[1]
                .split(", ")
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>(),
        )
    }

    fn pulses_to_send(&self, pulse: bool) -> Vec<Pulse> {
        self.next_modules
            .iter()
            .map(|n| Pulse::new(&self.name, pulse, n))
            .collect()
    }
}

#[derive(Debug)]
struct FlipFlop {
    common: ModuleCommon,
    state: bool,
}

impl FlipFlop {
    fn build(line: &str) -> Self {
        Self {
            common: ModuleCommon::build(&line[1..]),
            state: false,
        }
    }

    fn flip(&mut self) {
        self.state ^= true;
    }
}

impl Module for FlipFlop {
    fn get_type(&self) -> &str {
        "FlipFlop"
    }

    fn get_common(&self) -> &ModuleCommon {
        &self.common
    }

    fn exec(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        if pulse.value {
            // HIGH: ignore it
            Vec::new()
        } else {
            // LOW: flip and sends pulse matching state
            self.flip();
            self.common.pulses_to_send(self.state)
        }
    }

    fn is_initial_state(&self) -> bool {
        !self.state
    }

    fn reset(&mut self) {
        self.state = false;
        assert!(self.is_initial_state());
    }
}

#[test]
fn test_flipflop() {
    const IR: &str = "irrelevant";
    let mut m = FlipFlop::build("%a -> b");
    assert_eq!(m.get_name(), "a");
    assert_eq!(m.common.next_modules, ["b"]);
    assert_eq!(m.state, LOW);
    assert_eq!(m.exec(&Pulse::new(IR, HIGH, "a")), Vec::new());
    assert!(m.is_initial_state());
    assert_eq!(
        m.exec(&Pulse::new(IR, LOW, "a")),
        [Pulse::new("a", HIGH, "b")]
    );
    assert_eq!(
        m.exec(&Pulse::new(IR, LOW, "a")),
        [Pulse::new("a", LOW, "b")]
    );
    assert!(m.is_initial_state());
    assert_eq!(m.exec(&Pulse::new(IR, HIGH, "a")), Vec::new());
    assert_eq!(
        m.exec(&Pulse::new(IR, LOW, "a")),
        [Pulse::new("a", HIGH, "b")]
    );
}

#[derive(Debug)]
struct Conjunction {
    common: ModuleCommon,
    previous_pulse: HashMap<String, bool>,
}

impl Conjunction {
    fn build(line: &str) -> Self {
        Self {
            common: ModuleCommon::build(&line[1..]),
            previous_pulse: HashMap::new(),
        }
    }
}

impl Module for Conjunction {
    fn get_type(&self) -> &str {
        "Conjunction"
    }

    fn get_common(&self) -> &ModuleCommon {
        &self.common
    }

    fn exec(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        // Update that memory to this input
        self.previous_pulse.insert(pulse.from.clone(), pulse.value);

        if self.previous_pulse.values().all(|mem| *mem) {
            // if all input are high, send low pulse
            self.common.pulses_to_send(LOW)
        } else {
            // else send high pulse
            self.common.pulses_to_send(HIGH)
        }
    }

    fn is_initial_state(&self) -> bool {
        self.previous_pulse.is_empty() || self.previous_pulse.values().all(|mem| !*mem)
    }

    fn update_previous_pulse(&mut self, previous_pulse: HashMap<String, bool>) {
        self.previous_pulse = previous_pulse;
    }

    fn reset(&mut self) {
        self.previous_pulse
            .values_mut()
            .for_each(|val| *val = false);
        assert!(self.is_initial_state());
    }
}

#[test]
fn test_conjunction() {
    let mut m = Conjunction::build("&inv -> b");
    assert_eq!(m.get_name(), "inv");
    assert_eq!(m.common.next_modules, ["b"]);
    assert!(m.previous_pulse.is_empty());
    assert!(m.is_initial_state());
    assert_eq!(
        m.exec(&Pulse::new("a", HIGH, "inv")),
        [Pulse::new("inv", LOW, "b")]
    );
    assert_eq!(
        m.exec(&Pulse::new("c", LOW, "inv")),
        [Pulse::new("inv", HIGH, "b")]
    );
    assert!(!m.is_initial_state());

    assert_eq!(
        m.exec(&Pulse::new("a", LOW, "inv")),
        [Pulse::new("inv", HIGH, "b")]
    );
    assert!(m.is_initial_state());
}

#[derive(Debug)]
struct Broadcast {
    common: ModuleCommon,
}

impl Broadcast {
    const NAME: &'static str = "broadcaster";

    fn build(line: &str) -> Self {
        Self {
            common: ModuleCommon::build(line),
        }
    }
}

impl Module for Broadcast {
    fn get_type(&self) -> &str {
        "Broadcast"
    }

    fn get_common(&self) -> &ModuleCommon {
        &self.common
    }

    fn exec(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.common.pulses_to_send(pulse.value)
    }

    fn is_initial_state(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        assert!(self.is_initial_state());
    }
}

#[test]
fn test_broadcast() {
    let mut m = Broadcast::build("broadcaster -> a, b, c");
    assert_eq!(m.get_name(), "broadcaster");
    assert_eq!(m.common.next_modules, ["a", "b", "c"]);
    assert!(m.is_initial_state());
    assert_eq!(
        m.exec(&Pulse::new("irrelevant", HIGH, "broadcaster")),
        [
            Pulse::new("broadcaster", HIGH, "a"),
            Pulse::new("broadcaster", HIGH, "b"),
            Pulse::new("broadcaster", HIGH, "c")
        ]
    );
    assert!(m.is_initial_state());
}

#[derive(Debug)]
struct Button {
    common: ModuleCommon,
}

impl Button {
    const NAME: &'static str = "button";

    fn new() -> Self {
        Self {
            common: ModuleCommon {
                name: Self::NAME.to_string(),
                next_modules: vec![Broadcast::NAME.to_string()],
            },
        }
    }
}

impl Module for Button {
    fn get_type(&self) -> &str {
        "Button"
    }

    fn get_common(&self) -> &ModuleCommon {
        &self.common
    }

    fn exec(&mut self, _pulse: &Pulse) -> Vec<Pulse> {
        self.common.pulses_to_send(LOW)
    }

    fn is_initial_state(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        assert!(self.is_initial_state());
    }
}

#[test]
fn test_button() {
    let mut m = Button::new();
    assert_eq!(m.get_name(), "button");
    assert_eq!(
        m.exec(&Pulse::new("irrelevant", HIGH, "button")),
        [Pulse::new("button", LOW, "broadcaster")]
    );
    assert!(m.is_initial_state());
}

type Configuration = HashMap<String, Box<dyn Module>>;

fn is_config_in_initial_state(configuration: &Configuration) -> bool {
    configuration.values().all(|m| m.is_initial_state())
}

const DEBUG: bool = false;

fn run_once_with_module_detection(
    configuration: &mut Configuration,
    module_to_detect: &Option<String>,
) -> (u64, u64) {
    let mut pulses_to_exec: VecDeque<Pulse> = VecDeque::new();

    // Press button
    pulses_to_exec.push_back(Pulse::new("", LOW, Button::NAME)); // LOW/HIGH irrelevant here
    if DEBUG {
        println!("-------------------------");
    }

    let mut count_low = 0;
    let mut count_high = 0;
    while !pulses_to_exec.is_empty() {
        // ("destination module", "pulse")
        let received_pulse: Pulse = pulses_to_exec.pop_front().unwrap();
        if let Some(module) = configuration.get_mut(&received_pulse.to) {
            let sent_pulses = module.exec(&received_pulse);
            for sent in sent_pulses {
                if sent.value {
                    count_high += 1;
                } else {
                    count_low += 1;
                }
                if let Some(mod_to_catch) = module_to_detect {
                    if *mod_to_catch == sent.to && sent.value == LOW {
                        // println!("{} got a LOW, stopping", mod_to_catch);
                        // Hack to allow caller to detect we interrupted because we found the module
                        return (u64::MIN, u64::MIN);
                    }
                }
                if DEBUG {
                    println!(
                        "{} -{}-> {}",
                        received_pulse.to,
                        if sent.value { "high" } else { "low" },
                        sent.to
                    );
                }
                pulses_to_exec.push_back(sent);
            }
        } else {
            // if dest module is not found, nothing to execute
            // if received_pulse.1 { count_high += 1 } else { count_low += 1 }
        }
    }

    if DEBUG {
        println!(
            "__ In initial state: {}",
            is_config_in_initial_state(configuration)
        );
    }
    (count_low, count_high)
}

fn run_once(configuration: &mut Configuration) -> (u64, u64) {
    run_once_with_module_detection(configuration, &None)
}

const PRESS_COUNT: usize = 1000;
// const PRESS_COUNT: usize = 4;

// Part 1
fn total_pulses_count_product(configuration: &mut Configuration) -> u64 {
    reset_configuration(configuration);

    let (sum_low, sum_high) = (0..PRESS_COUNT)
        .map(|_| run_once(configuration))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    // println!("Total low pulse: {}, high pulses {}", sum_low, sum_high);
    sum_low * sum_high
}

// Fewest number of button presses required to deliver a single low pulse to specified module.
fn pulse_count_for_low_to(configuration: &mut Configuration, module: &str) -> u64 {
    reset_configuration(configuration);

    let mut count = 0;
    let rx = Some(module.to_string());
    loop {
        let res = run_once_with_module_detection(configuration, &rx);
        count += 1;
        if is_config_in_initial_state(configuration) {
            println!("We are back in initial state after {count} runs");
        }
        if res == (u64::MIN, u64::MIN) {
            break;
        }
        if count % 100_000 == 0 {
            println!("{count} runs..");
        }
    }
    count
}

// Part 2
// Fewest number of button presses required to deliver a single low pulse to the module named rx.
// This requires to get the 4 nodes listed below in low state at the same time.
fn pulse_count_for_low_to_rx(configuration: &mut Configuration) -> u64 {
    [
        pulse_count_for_low_to(configuration, "dl"),
        pulse_count_for_low_to(configuration, "ks"),
        pulse_count_for_low_to(configuration, "pm"),
        pulse_count_for_low_to(configuration, "vk"),
    ]
    .iter()
    .fold(1, |n, i| num_integer::lcm(n, *i))
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
            panic!("Invalid line: {line}")
        };
        configuration.insert(m.get_name().to_string(), m);
    }
    // Button module is not listed in the configuration, adding manually
    let m = Box::new(Button::new());
    configuration.insert(m.get_name().to_string(), m);
    // Conjunction setup needs to be finished once we know all the modules
    finish_conjunction_setup(&mut configuration);

    configuration
}

fn finish_conjunction_setup(configuration: &mut Configuration) {
    // Find all inputs for each Conjunction.
    // We need a temporary map to avoid borrowed as mutable trouble.
    let conj_to_inputs: HashMap<String, HashMap<String, bool>> = configuration
        .values()
        .filter(|m| m.get_type() == "Conjunction")
        .map(|m| {
            let con = m.get_name().to_string();
            (
                con.clone(),
                configuration
                    .iter()
                    .filter(|(_, v)| v.get_next_modules().contains(&con))
                    .map(|(k, _)| (k.clone(), false))
                    .collect::<HashMap<String, bool>>(),
            )
        })
        .collect();

    for c_i in conj_to_inputs {
        let m = configuration.get_mut(&c_i.0).unwrap();
        m.update_previous_pulse(c_i.1);
    }
}

fn reset_configuration(configuration: &mut Configuration) {
    // Note that just cloning Configuration before changing it, as the trait Module
    // isn't object-safe.
    configuration.values_mut().for_each(|m| m.reset());
}

fn main() {
    let stdin = io::stdin();
    let mut configuration = build_configuration(&mut stdin.lock());
    // println!("{:#?}", configuration);

    println!("Part 1: {}", total_pulses_count_product(&mut configuration));

    println!("Part 2: {}", pulse_count_for_low_to_rx(&mut configuration));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader1 = BufReader::new(File::open("resources/input_test1").unwrap());
        let mut configuration1 = build_configuration(&mut reader1);

        assert_eq!(total_pulses_count_product(&mut configuration1), 32000000);

        let mut reader2 = BufReader::new(File::open("resources/input_test2").unwrap());
        let mut configuration2 = build_configuration(&mut reader2);

        assert_eq!(total_pulses_count_product(&mut configuration2), 11687500);
    }
}
