use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

type Position = (usize, usize);

// Brian Kernighan’s Algorithm
fn count_set_bits(mut n: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        n &= n - 1;
        count += 1;
    }
    count
}

fn is_wall(pos: Position, fav_nb: usize) -> bool {
    let (x, y) = (pos.0, pos.1);
    let n = x * x + 3 * x + 2 * x * y + y + y * y + fav_nb;
    let set_bits = count_set_bits(n);
    set_bits % 2 == 1
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

// Check we don't go outside grid.
fn allowed(pos: Position, direction: Direction) -> bool {
    match direction {
        North => pos.1 > 0,
        West => pos.0 > 0,
        South | East => true,
    }
}

// Returns the position of the next move in that direction.
// Assumes validity of the move has been checked before with `allowed`.
fn next_pos(pos: Position, direction: Direction) -> Position {
    match direction {
        North => (pos.0, pos.1 - 1),
        South => (pos.0, pos.1 + 1),
        West => (pos.0 - 1, pos.1),
        East => (pos.0 + 1, pos.1),
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Position,
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

// Dijkstra shortest path
fn find_shortest_path(
    fav_nb: usize,
    start: Position,
    target: Option<Position>,
) -> (FxHashMap<Position, usize>, usize) {
    let mut visited: FxHashSet<Position> = FxHashSet::default();
    let mut distance: FxHashMap<Position, usize> = FxHashMap::default();
    let mut previous: FxHashMap<Position, Position> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        // Mark node as visited
        visited.insert(pos);

        if let Some(end) = target {
            if pos == end {
                shortest_distance = usize::min(shortest_distance, cost);
                continue;
            }
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|d| {
            // Not allowed: Going outside grid
            if !allowed(pos, *d) {
                return None;
            }
            // we could also optimize not going backwards at this level

            let next_pos = next_pos(pos, *d);
            if is_wall(next_pos, fav_nb) {
                return None;
            }

            if visited.contains(&next_pos) {
                return None;
            }

            let next_cost = cost + 1;
            if let Some(prevcost) = distance.get(&next_pos) {
                if *prevcost <= next_cost {
                    return None;
                }
            }

            distance.insert(next_pos, next_cost);
            previous.insert(next_pos, pos);

            Some(Node {
                pos: next_pos,
                cost: next_cost,
            })
        }));
    }

    (distance, shortest_distance)
}

fn min_number_steps(fav_nb: usize, target: (usize, usize)) -> usize {
    let (distance, shortest_distance) = find_shortest_path(fav_nb, (1, 1), Some(target));

    let end_key = distance
        .iter()
        .filter(|(&k, _)| k == target)
        .min_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap();
    assert_eq!(shortest_distance, *distance.get(end_key).unwrap());

    shortest_distance
}

fn locations_count_reachable_in_max(fav_nb: usize, max_steps: usize) -> usize {
    let (distance, _) = find_shortest_path(fav_nb, (1, 1), None);
    distance.iter().filter(|(_, &d)| d <= max_steps).count() + 1
    // +1 as the start is not included in the distances
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let fav_nb: usize = input.trim().parse().unwrap();

    let target = (31, 39);
    println!("Part 1: {}", min_number_steps(fav_nb, target));
    println!("Part 2: {}", locations_count_reachable_in_max(fav_nb, 50));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traditional version
    fn count_set_bits_trad(mut n: usize) -> usize {
        let mut count = 0;
        while n > 0 {
            count += n & 1;
            n >>= 1;
        }
        count
    }

    #[test]
    fn test_count_set_bits() {
        assert_eq!(count_set_bits(353252), count_set_bits_trad(353252));
    }

    #[test]
    fn test_is_wall() {
        assert!(is_wall((6, 2), 10));
        assert!(!is_wall((5, 3), 10));
    }

    #[test]
    fn test_part1() {
        assert_eq!(min_number_steps(10, (7, 4)), 11);
    }
}
