use std::io::{self, Read};

use fxhash::FxHashMap;
use lazy_static::lazy_static;
use regex::Regex;

const ORE_INDEX: usize = 0;
const CLAY_INDEX: usize = 1;
const OBSIDIAN_INDEX: usize = 2;
const GEODE_INDEX: usize = 3;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    // Each line indicates the cost to buy the corresponding robot.
    // First line how much to buy a Ore robot, next Clay, then Obsidian and finally Geode.
    // In a line, first element is ore, then clay and last obsidian. There is no geode, as geode
    // are not used to buy robots.
    robots_cost: [[u32; 3]; 4],
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        fn int(s: &str) -> u32 {
            s.parse().unwrap()
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
            .unwrap();
        }

        let p = RE.captures(value).unwrap();
        Self {
            id: int(&p[1]),
            robots_cost: [
                [int(&p[2]), 0, 0],
                [int(&p[3]), 0, 0],
                [int(&p[4]), int(&p[5]), 0],
                [int(&p[6]), 0, int(&p[7])],
            ],
        }
    }
}

fn build(input: &str) -> Vec<Blueprint> {
    input.lines().map(Into::into).collect()
}

impl Blueprint {
    // Returns the indexes of the robots that can be purchased with these resources.
    fn robots_available_for_purchase(&self, resources: &[u32]) -> Vec<usize> {
        (0..4)
            .filter(|&robot_idx| {
                self.robots_cost[robot_idx]
                    .iter()
                    .zip(resources.iter())
                    .all(|(cost, available)| available >= cost)
            })
            .collect()
    }

    fn pay_for_robot(&self, robot_we_are_buying: usize, resources: &mut [u32]) {
        for (resource_idx, cost) in self.robots_cost[robot_we_are_buying]
            .iter()
            .enumerate()
            .filter(|(_, cost)| **cost > 0)
        {
            resources[resource_idx] -= cost;
        }
    }
}

// Resource production.
// Add to resources list the resources produces by these robots on one minute.
fn produce_resouces(robots: &[u32], resources: &mut [u32]) {
    for (resource_idx, robot_count) in robots.iter().enumerate() {
        if *robot_count > 0 {
            resources[resource_idx] += robot_count;
        }
    }
}

// Make bought robot available for production.
fn enable_robot(robots: &mut [u32], robot_we_are_buying: usize) {
    robots[robot_we_are_buying] += 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    robots: [u32; 4],
    resources: [u32; 4],
}

impl State {
    fn new() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0; 4],
        }
    }

    fn geodes_count(&self) -> u32 {
        self.resources[3]
    }
}

// Insert the state to the states map, updating the max if needed.
fn insert_state(states: &mut FxHashMap<State, u32>, s: State, max_geodes: u32) {
    // Pruning of the list of states, to keep it from growing too big.
    if s.geodes_count() + 3 < max_geodes {
        return;
    }

    states
        .entry(s)
        .and_modify(|c| {
            if s.geodes_count() > *c {
                *c = s.geodes_count();
            }
        })
        .or_insert(s.geodes_count());
}

fn collect_geodes(blueprint: &Blueprint, time: usize) -> u32 {
    // Maximum number of geodes collected from this state.
    let mut max_geodes_for_states: FxHashMap<State, u32> = FxHashMap::default();

    // At minute 0, we have 1 ore.
    let initial_state = State::new();
    max_geodes_for_states.insert(initial_state, initial_state.geodes_count());

    let mut max_geodes = u32::MIN;

    for minute in 1..=time {
        let mut new_states: FxHashMap<State, u32> = FxHashMap::default();

        for state in max_geodes_for_states.keys() {
            // Find out if any robots can be purchased.
            let mut robots_purchasable = if minute < time {
                blueprint.robots_available_for_purchase(&state.resources)
            } else {
                // If we are in last minute, no point purchasing a robot.
                vec![]
            };

            if robots_purchasable.is_empty() {
                // Can't purchase anything, just produce resources and we are done for this minute.
                let mut state_copy = *state;
                produce_resouces(&state_copy.robots, &mut state_copy.resources);

                insert_state(&mut new_states, state_copy, max_geodes);
                continue;
            }

            // Robot production.
            // It seems we can assume that each minute, we can only build one robot maximum.

            // If we can buy a geode robot, buy this one immediately, no need to explore other options.
            if robots_purchasable.contains(&GEODE_INDEX) {
                robots_purchasable = vec![GEODE_INDEX];
            }

            // Don't buy robots that won't have time to help us anymore.
            if minute > time - 3 && robots_purchasable.contains(&ORE_INDEX) {
                robots_purchasable.retain(|i| *i != ORE_INDEX);
            }
            if minute > time - 3 && robots_purchasable.contains(&OBSIDIAN_INDEX) {
                robots_purchasable.retain(|i| *i != OBSIDIAN_INDEX);
            }
            if minute > time - 5 && robots_purchasable.contains(&CLAY_INDEX) {
                robots_purchasable.retain(|i| *i != CLAY_INDEX);
            }

            for robot_we_are_buying in robots_purchasable {
                let mut state_copy = *state;

                // We cannot make the robot available for building immediately.
                blueprint.pay_for_robot(robot_we_are_buying, &mut state_copy.resources);

                produce_resouces(&state_copy.robots, &mut state_copy.resources);

                // Make bought robot available for production.
                enable_robot(&mut state_copy.robots, robot_we_are_buying);

                insert_state(&mut new_states, state_copy, max_geodes);
            }

            // Not buying any is one of the options.
            let mut state_copy = *state;
            produce_resouces(&state_copy.robots, &mut state_copy.resources);

            insert_state(&mut new_states, state_copy, max_geodes);
        }

        std::mem::swap(&mut max_geodes_for_states, &mut new_states);

        // Not need to take max of previous rounds, we are not going lower on each round.
        max_geodes = *max_geodes_for_states.values().max().unwrap();

        // println!(
        //     "{}: {} states, max {}",
        //     minute,
        //     max_geodes_for_states.len(),
        //     max_geodes
        // );
    }

    // println!("Blueprint {} max is {}", blueprint.id, max_geodes);
    max_geodes
}

fn quality_levels(blueprints: &[Blueprint]) -> u32 {
    const TIME: usize = 24;
    blueprints
        .iter()
        .map(|blueprint| {
            let max_geodes = collect_geodes(blueprint, TIME);
            blueprint.id * max_geodes
        })
        .sum()
}

fn top_3_max_product(blueprints: &[Blueprint]) -> u32 {
    const TIME: usize = 32;
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| collect_geodes(blueprint, TIME))
        .product()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let blueprints = build(&input);

    println!("Part 1: {}", quality_levels(&blueprints));
    println!("Part 2: {}", top_3_max_product(&blueprints));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(quality_levels(&build(INPUT_TEST)), 33);
    }
}
