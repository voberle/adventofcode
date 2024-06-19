use std::io::{self, Read};

use fxhash::FxHashMap;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    next_valves_names: Vec<String>,
    // Same as next_valves_names, but the indexes in the valves vector, for faster lookup.
    next_valves: Vec<usize>,
}

// Returns the valves sorted by flow rate (the 0 flow rate at the end)
fn build(input: &str) -> Vec<Valve> {
    // Parse the input.
    let re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    let mut valves: Vec<Valve> = input
        .lines()
        .map(|line| {
            let p = re.captures(line).unwrap();
            Valve {
                name: p[1].to_string(),
                flow_rate: p[2].parse().unwrap(),
                next_valves_names: p[3].split(", ").map(ToString::to_string).collect(),
                next_valves: Vec::new(),
            }
        })
        .collect();

    // Optimize the data structure by sorting it, and filling next_valves.

    // Sort the valves by flow rate (must be done before filling next_valves).
    valves.sort_unstable_by_key(|valve| valve.flow_rate);
    valves.reverse();

    for i in 0..valves.len() {
        let indexes: Vec<usize> = valves[i]
            .next_valves_names
            .iter()
            .map(|vs| valves.iter().position(|v| v.name == *vs).unwrap())
            .collect();

        valves[i].next_valves.extend(indexes);
    }

    valves
}

fn start_valve_index(valves: &[Valve]) -> usize {
    valves
        .iter()
        .position(|v| v.name == "AA")
        .expect("Didn't find start valve")
}

fn open(opened_valves: u32, i: usize) -> u32 {
    opened_valves | (1 << i)
}

fn is_open(opened_valves: u32, i: usize) -> bool {
    opened_valves & (1 << i) != 0
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    own_valve: usize,
    helper_valve: usize,
    // Is the valve is open or not. We don't have that many, so using a bitmask.
    opened_valves: u32,
}

impl State {
    fn initial(start_valve: usize) -> Self {
        Self {
            own_valve: start_valve,
            helper_valve: start_valve,
            opened_valves: 0,
        }
    }

    fn open_this(&self) -> Self {
        let opened_valves = open(self.opened_valves, self.own_valve);
        Self {
            own_valve: self.own_valve,
            helper_valve: self.helper_valve,
            opened_valves,
        }
    }

    fn move_to(&self, next_valve: usize) -> Self {
        Self {
            own_valve: next_valve,
            helper_valve: self.helper_valve,
            opened_valves: self.opened_valves,
        }
    }
}

type NextStatesFn = fn(
    next_minute_pressures: &mut FxHashMap<State, u32>,
    valves: &[Valve],
    state: &State,
    pressure: u32,
    minute: u32,
    total_time: u32,
);

fn max_pressure(valves: &[Valve], total_time: u32, next_states_fn: NextStatesFn) -> u32 {
    // We main a set of all possible states for a certain minute.
    // Then we go to next minute and create a new set with all new states.

    // All the best pressures for each state at this minute.
    let mut best_pressures_at: FxHashMap<State, u32> = FxHashMap::default();

    // At minute 0, we are at valve AA.
    best_pressures_at.insert(State::initial(start_valve_index(valves)), 0);

    for minute in 1..=total_time {
        let mut next_minute_pressures: FxHashMap<State, u32> = FxHashMap::default();
        for (state, pressure) in &best_pressures_at {
            next_states_fn(
                &mut next_minute_pressures,
                valves,
                state,
                *pressure,
                minute,
                total_time,
            );
        }

        std::mem::swap(&mut best_pressures_at, &mut next_minute_pressures);
        println!(
            "{}: {} options, max {}",
            minute,
            best_pressures_at.len(),
            best_pressures_at.values().max().unwrap()
        );
    }

    *best_pressures_at.values().max().unwrap()
}

fn alone_next_states(
    next_minute_pressures: &mut FxHashMap<State, u32>,
    valves: &[Valve],
    state: &State,
    pressure: u32,
    minute: u32,
    total_time: u32,
) {
    let valve = valves.get(state.own_valve).unwrap();

    // If this valve is closed and has some rate, we can choose to open it or not.
    if valve.flow_rate > 0 && !is_open(state.opened_valves, state.own_valve) {
        // Opening it.
        let new_pressure = pressure + valve.flow_rate * (total_time - minute);
        next_minute_pressures
            .entry(state.open_this())
            .and_modify(|p| {
                if new_pressure > *p {
                    *p = new_pressure;
                }
            })
            .or_insert(new_pressure);
    }

    // Moving without opening it.
    for next_valve in &valve.next_valves {
        next_minute_pressures
            .entry(state.move_to(*next_valve))
            .and_modify(|p| {
                if pressure > *p {
                    *p = pressure;
                }
            })
            .or_insert(pressure);
    }
}

fn max_pressure_alone(valves: &[Valve]) -> u32 {
    max_pressure(valves, 30, alone_next_states)
}

fn options_for(
    valves: &[Valve],
    current_valve: usize,
    // prev_valve: usize,
    opened_valves: u32,
) -> Vec<(usize, Option<usize>)> {
    let mut options: Vec<(usize, Option<usize>)> = Vec::new();

    let valve = valves.get(current_valve).unwrap();

    // If this valve is closed and has some rate, we can choose to open it or not.
    if valve.flow_rate > 0 && !is_open(opened_valves, current_valve) {
        // Opening it.
        options.push((current_valve, Some(current_valve)));
    }

    // Moving without opening it.
    for next_valve in &valve.next_valves {
        options.push((*next_valve, None));
    }

    options
}

fn with_helper_next_states(
    next_minute_pressures: &mut FxHashMap<State, u32>,
    valves: &[Valve],
    state: &State,
    pressure: u32,
    minute: u32,
    total_time: u32,
) {
    // Where we move next (can be the same if no move) and the valve that was opened, if any.
    let own_options = options_for(valves, state.own_valve, state.opened_valves);
    let helper_options = options_for(valves, state.helper_valve, state.opened_valves);

    for (own_next, valve_i_open) in own_options {
        for (helper_next, valve_helper_opens) in &helper_options {
            // if own_next == *helper_next {
            //     continue;
            // }

            let mut new_pressure = pressure;
            let mut opened_valves = state.opened_valves;
            if let Some(i) = valve_i_open {
                assert_eq!(i, own_next);
                new_pressure += valves.get(own_next).unwrap().flow_rate * (total_time - minute);
                opened_valves = open(opened_valves, i);
            }
            if let Some(h) = valve_helper_opens {
                assert_eq!(h, helper_next);
                if is_open(opened_valves, *h) {
                    // Don't have both open the same valve.
                    continue;
                }
                new_pressure += valves.get(*helper_next).unwrap().flow_rate * (total_time - minute);
                opened_valves = open(opened_valves, *h);
            }

            next_minute_pressures
                .entry(State {
                    own_valve: own_next,
                    helper_valve: *helper_next,
                    opened_valves,
                })
                .and_modify(|p| {
                    if new_pressure > *p {
                        *p = new_pressure;
                    }
                })
                .or_insert(new_pressure);
        }
    }
}

fn max_pressure_with_help(valves: &[Valve]) -> u32 {
    max_pressure(valves, 26, with_helper_next_states)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let valves = build(&input);

    println!("Part 1: {}", max_pressure_alone(&valves));
    println!("Part 2: {}", max_pressure_with_help(&valves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(max_pressure_alone(&build(INPUT_TEST)), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(max_pressure_with_help(&build(INPUT_TEST)), 1707);
    }
}
