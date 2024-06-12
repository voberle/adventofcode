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
    id: usize,
    // Each line indicates the cost to buy the corresponding robot.
    // First line how much to buy a Ore robot, next Clay, then Obsidian and finally Geode.
    // In a line, first element is ore, then clay and last obsidian. There is no geode, as geode
    // are not used to buy robots.
    robots_cost: [[usize; 3]; 4],
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        fn int(s: &str) -> usize {
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
    fn robots_available_for_purchase(&self, resources: &[usize]) -> Vec<usize> {
        (0..4)
            .filter(|&robot_idx| {
                self.robots_cost[robot_idx]
                    .iter()
                    .zip(resources.iter())
                    .all(|(cost, available)| available >= cost)
            })
            .collect()
    }

    fn pay_for_robot(&self, robot_we_are_buying: usize, resources: &mut [usize]) {
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
fn produce_resouces(robots: &[usize], resources: &mut [usize]) {
    for (resource_idx, robot_count) in robots.iter().enumerate() {
        if *robot_count > 0 {
            resources[resource_idx] += robot_count;
        }
    }
}

// Make bought robot available for production.
fn enable_robot(robots: &mut [usize], robot_we_are_buying: usize) {
    robots[robot_we_are_buying] += 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    robots: [usize; 4],
    resources: [usize; 4],
}

impl State {
    fn new() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0; 4],
        }
    }

    fn geodes_count(&self) -> usize {
        self.resources[3]
    }
}

fn collect_geodes(blueprint: &Blueprint) -> usize {
    const TIME: usize = 24;

    // Maximum number of geodes collected from this state.
    let mut max_geodes_for_states: FxHashMap<State, usize> = FxHashMap::default();

    // At minute 0, we have 1 ore.
    let initial_state = State::new();
    max_geodes_for_states.insert(initial_state, initial_state.geodes_count());

    // println!("Checking blueprint {}", blueprint.id);
    for minute in 1..=TIME {
        let mut new_states: FxHashMap<State, usize> = FxHashMap::default();

        for (state, _best_geodes_count) in &max_geodes_for_states {
            // Find out if any robots can be purchased.
            let mut robots_purchasable = if minute < TIME {
                blueprint.robots_available_for_purchase(&state.resources)
            } else {
                // If we are in last minute, no point purchasing a robot.
                vec![]
            };

            if robots_purchasable.is_empty() {
                let mut state_copy = *state;

                // Can't purchase anything, just produce resources and we are done for this minute.
                produce_resouces(&state_copy.robots, &mut state_copy.resources);

                new_states
                    .entry(state_copy)
                    .and_modify(|c| {
                        if state_copy.geodes_count() > *c {
                            *c = state_copy.geodes_count();
                        }
                    })
                    .or_insert(state_copy.geodes_count());

                continue;
            }

            // Robot production.
            // It seems we can assume that each minute, we can only build one robot maximum.

            // If we can buy a geode robot, buy this one immediately, no need to explore other options.
            if robots_purchasable.contains(&GEODE_INDEX) {
                robots_purchasable = vec![GEODE_INDEX];
            }

            // Don't buy robots that won't have time to help us anymore.
            if minute > TIME - 3 && robots_purchasable.contains(&ORE_INDEX) {
                robots_purchasable.retain(|i| *i != ORE_INDEX);
            }
            if minute > TIME - 3 && robots_purchasable.contains(&OBSIDIAN_INDEX) {
                robots_purchasable.retain(|i| *i != OBSIDIAN_INDEX);
            }
            if minute > TIME - 5 && robots_purchasable.contains(&CLAY_INDEX) {
                robots_purchasable.retain(|i| *i != CLAY_INDEX);
            }

            for robot_we_are_buying in robots_purchasable {
                let mut state_copy = *state;

                // We cannot make the robot available for building immediately.
                blueprint.pay_for_robot(robot_we_are_buying, &mut state_copy.resources);

                produce_resouces(&state_copy.robots, &mut state_copy.resources);

                // Make bought robot available for production.
                enable_robot(&mut state_copy.robots, robot_we_are_buying);

                new_states
                    .entry(state_copy)
                    .and_modify(|c| {
                        if state_copy.geodes_count() > *c {
                            *c = state_copy.geodes_count();
                        }
                    })
                    .or_insert(state_copy.geodes_count());
            }

            // Not buying any is one of the options.
            let mut state_copy = *state;
            produce_resouces(&state_copy.robots, &mut state_copy.resources);

            new_states
                .entry(state_copy)
                .and_modify(|c| {
                    if state_copy.geodes_count() > *c {
                        *c = state_copy.geodes_count();
                    }
                })
                .or_insert(state_copy.geodes_count());
        }

        std::mem::swap(&mut max_geodes_for_states, &mut new_states);
        // println!("{}: {} states, max {}", minute, max_geodes_for_states.len(), max_geodes_for_states.values().max().unwrap());
    }
    let max = *max_geodes_for_states.values().max().unwrap();
    // println!("Blueprint {} max is {}", blueprint.id, max);
    max
}

fn quality_level(blueprint: &Blueprint) -> usize {
    let max_geodes = collect_geodes(blueprint);
    blueprint.id * max_geodes
}

fn quality_levels(blueprints: &[Blueprint]) -> usize {
    blueprints.iter().map(quality_level).sum()
}

fn part2(blueprints: &[Blueprint]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let blueprints = build(&input);

    println!("Part 1: {}", quality_levels(&blueprints));
    println!("Part 2: {}", part2(&blueprints));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(quality_levels(&build(INPUT_TEST)), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
