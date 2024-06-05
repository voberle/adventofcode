use std::collections::BinaryHeap;

use fxhash::{FxHashMap, FxHashSet};

use crate::direction::ALL_DIRECTIONS;

/// Base implementation of Dijkstra shortest path algorithm.

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    // Add extra dimensions to the search here.
    // Such dimensions must also be added to visited hash set and distance hash map.
    cost: usize,
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
fn find_shortest_path() -> usize {
    let start = 0;
    let end = 100;

    let mut visited: FxHashSet<usize> = FxHashSet::default();
    let mut distance: FxHashMap<usize, usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    // If positions is just a usize, we can replace the Set and Map with simple vectors:
    // let mut visited: Vec<bool> = vec![false; area.values.len()];
    // let mut distance: Vec<usize> = vec![usize::MAX; area.values.len()];

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        visited.insert(pos);
        // visited[pos] = true;

        if pos == end {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        // For simpler directions, we can skip the Direction enum and just do:
        // queue.extend(area.next_positions_iter(pos).filter_map(|next_pos| {

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|_d| {
            // Check if direction is valid, and any other check.

            // Calculate next position.
            let next_pos = pos + 1;

            // Check if next pos is valid.

            if visited.contains(&next_pos) {
                // if visited[next_pos] {
                return None;
            }

            // Adjust here if cost logic is more complicated.
            let next_cost = cost + 1;
            // if distance[next_pos] <= next_cost {
            if let Some(prevcost) = distance.get(&next_pos) {
                if *prevcost <= next_cost {
                    return None;
                }
            }

            // Possibly avoid going too far with checks like if next_cost >= shortest_distance { return None; }

            distance.insert(next_pos, next_cost);
            // distance[next_pos] = next_cost;
            Some(Node {
                pos: next_pos,
                cost: next_cost,
            })
        }));
    }
    shortest_distance
}
