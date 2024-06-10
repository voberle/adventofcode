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

// Number of valves that have a positive flow rate.
fn positive_flow_rate_count(valves: &[Valve]) -> usize {
    valves.iter().filter(|valve| valve.flow_rate > 0).count()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    current_valve: usize,
    // Array indicating if the valve is open or not.
    opened_valves: Vec<bool>,
}

impl State {
    fn initial(start_valve: usize, positive_flow_rate_count: usize) -> Self {
        Self {
            current_valve: start_valve,
            opened_valves: vec![false; positive_flow_rate_count],
        }
    }

    fn open_this(&self) -> Self {
        let mut opened_valves = self.opened_valves.clone();
        opened_valves[self.current_valve] = true;
        Self {
            current_valve: self.current_valve,
            opened_valves,
        }
    }

    fn move_to(&self, next_valve: usize) -> Self {
        Self {
            current_valve: next_valve,
            opened_valves: self.opened_valves.clone(),
        }
    }
}

fn max_pressure(valves: &[Valve]) -> u32 {
    // We main a set of all possible states for a certain minute.
    // Then we go to next minute and create a new set with all new states.

    const TIME: u32 = 30;

    // All the best pressures for each state at this minute.
    let mut best_pressures_at: FxHashMap<State, u32> = FxHashMap::default();

    // At minute 0, we are at valve AA.
    best_pressures_at.insert(
        State::initial(start_valve_index(valves), positive_flow_rate_count(valves)),
        0,
    );

    for minute in 1..=TIME {
        let mut next_minute_pressures: FxHashMap<State, u32> = FxHashMap::default();
        for (state, pressure) in &best_pressures_at {
            let valve = valves.get(state.current_valve).unwrap();

            // If this valve is closed and has some rate, we can choose to open it or not.
            if valve.flow_rate > 0 && !state.opened_valves[state.current_valve] {
                // Opening it.
                let new_pressure = pressure + valve.flow_rate * (TIME - minute);

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
                        if pressure > p {
                            *p = *pressure;
                        }
                    })
                    .or_insert(*pressure);
            }
        }

        std::mem::swap(&mut best_pressures_at, &mut next_minute_pressures);
    }

    *best_pressures_at.values().max().unwrap()
}

fn part2(valves: &[Valve]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let valves = build(&input);

    println!("Part 1: {}", max_pressure(&valves));
    println!("Part 2: {}", part2(&valves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(max_pressure(&build(INPUT_TEST)), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
