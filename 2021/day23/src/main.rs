use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

mod next_positions;

fn get_base_cost(amphipod: char) -> u32 {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Invalid amphipod"),
    }
}

// The idea is to encode all possible positions that we can have,
// and then run Dijkstra on it.
// The challenge then is to find the right encoding and the transition between the states.
// Each position where an amphipod can be is marked by a number, which can then be used as an index in a vector.
// #################
// #01. 2. 3. 4. 56#
// ###7 #8 #9 #10###
//   #11#12#13#14#
//   #############

type Burrow = [Option<char>; 15];

fn build(input: &str) -> Burrow {
    let mut burrow = [None; 15];
    for (i, c) in input.chars().filter(char::is_ascii_alphabetic).enumerate() {
        burrow[i + 7] = Some(c);
    }
    burrow
}

#[allow(dead_code)]
fn print_burrow(burrow: &Burrow) {
    println!(
        r"#############
#{}{}.{}.{}.{}.{}{}#
###{}#{}#{}#{}###
#{}#{}#{}#{}#
#########",
        burrow[0].unwrap_or('.'),
        burrow[1].unwrap_or('.'),
        burrow[2].unwrap_or('.'),
        burrow[3].unwrap_or('.'),
        burrow[4].unwrap_or('.'),
        burrow[5].unwrap_or('.'),
        burrow[6].unwrap_or('.'),
        burrow[7].unwrap_or('.'),
        burrow[8].unwrap_or('.'),
        burrow[9].unwrap_or('.'),
        burrow[10].unwrap_or('.'),
        burrow[11].unwrap_or('.'),
        burrow[12].unwrap_or('.'),
        burrow[13].unwrap_or('.'),
        burrow[14].unwrap_or('.'),
    );
}

fn get_next_states(burrow: &Burrow) -> Vec<(Burrow, u32)> {
    let mut next_states: Vec<(Burrow, u32)> = Vec::new();
    // For each position that is not empty,
    for (pos, amphipod) in burrow.iter().enumerate().filter(|(_, v)| v.is_some()) {
        // get all possible next positions and their cost
        for (next_pos, cost) in next_positions::get_next_possible_positions(burrow, pos) {
            // and create a burrow for it.
            let mut new_burrow = *burrow;
            new_burrow[next_pos] = new_burrow[pos];
            new_burrow[pos] = None;
            next_states.push((new_burrow, cost * get_base_cost(amphipod.unwrap())));
        }
    }
    next_states
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    burrow: Burrow,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra shortest path.
fn find_shortest_path(start: &Burrow, end: &Burrow) -> u32 {
    let mut visited: FxHashSet<Burrow> = FxHashSet::default();
    let mut distance: FxHashMap<Burrow, u32> = FxHashMap::default();
    let mut shortest_distance = u32::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        burrow: *start,
        cost: 0,
    });

    while let Some(Node { burrow, cost }) = queue.pop() {
        visited.insert(burrow);

        if burrow == *end {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        queue.extend(
            get_next_states(&burrow)
                .iter()
                .filter_map(|(next_burrow, state_cost)| {
                    if visited.contains(next_burrow) {
                        return None;
                    }

                    let next_cost = cost + state_cost;
                    if let Some(prevcost) = distance.get(next_burrow) {
                        if *prevcost <= next_cost {
                            return None;
                        }
                    }

                    if next_cost >= shortest_distance {
                        return None;
                    }

                    distance.insert(*next_burrow, next_cost);
                    Some(Node {
                        burrow: *next_burrow,
                        cost: next_cost,
                    })
                }),
        );
    }
    shortest_distance
}

#[rustfmt::skip]
fn least_energy_to_organize(burrow: &Burrow) -> u32 {
    const A: char = 'A';
    const B: char = 'B';
    const C: char = 'C';
    const D: char = 'D';
    let end = [
        None, None, None, None, None, None, None,
        Some(A), Some(B), Some(C), Some(D),
        Some(A), Some(B), Some(C), Some(D),
    ];

    find_shortest_path(burrow, &end)
}

fn part2(burrow: &Burrow) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let burrow = build(&input);
    // print_burrow(&burrow);

    println!("Part 1: {}", least_energy_to_organize(&burrow));
    println!("Part 2: {}", part2(&burrow));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(least_energy_to_organize(&build(INPUT_TEST)), 12521);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
