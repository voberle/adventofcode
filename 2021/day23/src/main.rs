use std::{
    collections::BinaryHeap,
    hash::{Hash, Hasher},
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet, FxHasher};

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

type Burrow = Vec<Option<char>>;

fn build(input: &str) -> Burrow {
    let mut burrow = vec![None, None, None, None, None, None, None];
    for c in input.chars().filter(char::is_ascii_alphabetic) {
        burrow.push(Some(c));
    }
    burrow
}

#[allow(dead_code)]
fn print_burrow(burrow: &Burrow) {
    println!("#############");
    println!(
        "#{}{}.{}.{}.{}.{}{}#",
        burrow[0].unwrap_or('.'),
        burrow[1].unwrap_or('.'),
        burrow[2].unwrap_or('.'),
        burrow[3].unwrap_or('.'),
        burrow[4].unwrap_or('.'),
        burrow[5].unwrap_or('.'),
        burrow[6].unwrap_or('.'),
    );
    println!(
        "###{}#{}#{}#{}###",
        burrow[7].unwrap_or('.'),
        burrow[8].unwrap_or('.'),
        burrow[9].unwrap_or('.'),
        burrow[10].unwrap_or('.'),
    );

    let end = if burrow.len() == 15 { 1 } else { 3 };
    for i in 0..end {
        println!(
            "  #{}#{}#{}#{}#",
            burrow[11 + 4 * i].unwrap_or('.'),
            burrow[11 + 4 * i + 1].unwrap_or('.'),
            burrow[11 + 4 * i + 2].unwrap_or('.'),
            burrow[11 + 4 * i + 3].unwrap_or('.'),
        );
    }

    println!("  #########");
}

fn get_next_states(burrow: &Burrow) -> Vec<(Burrow, u32)> {
    let mut next_states: Vec<(Burrow, u32)> = Vec::new();
    // For each position that is not empty,
    for (pos, amphipod) in burrow.iter().enumerate().filter(|(_, v)| v.is_some()) {
        // get all possible next positions and their cost
        for (next_pos, cost) in next_positions::get_next_possible_positions(burrow, pos) {
            // and create a burrow for it.
            let mut new_burrow = burrow.clone();
            new_burrow[next_pos] = new_burrow[pos];
            new_burrow[pos] = None;
            next_states.push((new_burrow, cost * get_base_cost(amphipod.unwrap())));
        }
    }
    next_states
}

type BurrowHash = u64;

fn burrow_hash(burrow: &[Option<char>]) -> BurrowHash {
    let mut hasher = FxHasher::default();
    burrow.hash(&mut hasher);
    hasher.finish()
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
    let mut visited: FxHashSet<BurrowHash> = FxHashSet::default();
    let mut distance: FxHashMap<BurrowHash, u32> = FxHashMap::default();
    let mut shortest_distance = u32::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        burrow: start.clone(),
        cost: 0,
    });

    while let Some(Node { burrow, cost }) = queue.pop() {
        visited.insert(burrow_hash(&burrow));

        if burrow == *end {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        queue.extend(
            get_next_states(&burrow)
                .iter()
                .filter_map(|(next_burrow, state_cost)| {
                    let next_burrow_hash = burrow_hash(next_burrow);
                    if visited.contains(&next_burrow_hash) {
                        return None;
                    }

                    let next_cost = cost + state_cost;
                    if let Some(prevcost) = distance.get(&next_burrow_hash) {
                        if *prevcost <= next_cost {
                            return None;
                        }
                    }

                    if next_cost >= shortest_distance {
                        return None;
                    }

                    distance.insert(next_burrow_hash, next_cost);
                    Some(Node {
                        burrow: next_burrow.clone(),
                        cost: next_cost,
                    })
                }),
        );
    }
    shortest_distance
}

const A: char = 'A';
const B: char = 'B';
const C: char = 'C';
const D: char = 'D';

#[rustfmt::skip]
fn least_energy_small_burrow(burrow: &Burrow) -> u32 {
    const END: [Option<char>; 15] = [
        None, None, None, None, None, None, None,
        Some(A), Some(B), Some(C), Some(D),
        Some(A), Some(B), Some(C), Some(D),
    ];

    find_shortest_path(burrow, &END.to_vec())
}

fn unfold_burrow(burrow: &Burrow) -> Burrow {
    // #D#C#B#A#
    // #D#B#A#C#
    const INSERT: [Option<char>; 8] = [
        Some(D),
        Some(C),
        Some(B),
        Some(A),
        Some(D),
        Some(B),
        Some(A),
        Some(C),
    ];

    let mut unfolded = burrow.clone();
    unfolded.resize(unfolded.len() + 8, None);
    unfolded.copy_within(11..11 + 4, 11 + 8);
    unfolded.splice(11..11 + 8, INSERT);

    unfolded
}

#[rustfmt::skip]
fn least_energy_big_burrow(burrow: &Burrow) -> u32 {
    const END: [Option<char>; 23] = [
        None, None, None, None, None, None, None,
        Some(A), Some(B), Some(C), Some(D),
        Some(A), Some(B), Some(C), Some(D),
        Some(A), Some(B), Some(C), Some(D),
        Some(A), Some(B), Some(C), Some(D),
    ];

    let unfolded = unfold_burrow(burrow);
    // print_burrow(&unfolded);

    find_shortest_path(&unfolded, &END.to_vec())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let burrow = build(&input);
    // print_burrow(&burrow);

    println!("Part 1: {}", least_energy_small_burrow(&burrow));
    println!("Part 2: {}", least_energy_big_burrow(&burrow));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(least_energy_small_burrow(&build(INPUT_TEST)), 12521);
    }

    #[test]
    fn test_part2() {
        assert_eq!(least_energy_big_burrow(&build(INPUT_TEST)), 44169);
    }
}
