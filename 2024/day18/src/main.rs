use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn move_to(&self, change: (isize, isize)) -> Self {
        Self::new(
            (self.x as isize + change.0) as usize,
            (self.y as isize + change.1) as usize,
        )
    }

    fn is_valid(&self, map_size: usize) -> bool {
        self.x < map_size && self.y < map_size
    }
}

impl From<(usize, usize)> for Coords {
    fn from(pair: (usize, usize)) -> Self {
        Self {
            x: pair.0,
            y: pair.1,
        }
    }
}

fn build(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| {
            (line
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap())
            .into()
        })
        .collect()
}

#[allow(dead_code)]
fn print_map(map: &FxHashSet<Coords>, map_size: usize, start: Coords, end: Coords) {
    for y in 0..map_size {
        for x in 0..map_size {
            let pos = Coords::new(x, y);
            if pos == start {
                print!("S");
            } else if pos == end {
                print!("E");
            } else if map.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Coords,
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
fn find_shortest_path(
    bytes_coords: &FxHashSet<Coords>,
    map_size: usize,
    start: Coords,
    end: Coords,
) -> usize {
    let mut visited: FxHashSet<Coords> = FxHashSet::default();
    let mut distance: FxHashMap<Coords, usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        visited.insert(pos);

        if pos == end {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        queue.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|mv| {
                    let next_pos = pos.move_to(mv);
                    if !next_pos.is_valid(map_size) {
                        return None;
                    }

                    if visited.contains(&next_pos) {
                        return None;
                    }

                    if bytes_coords.contains(&next_pos) {
                        return None;
                    }

                    let next_cost = cost + 1;
                    if let Some(prevcost) = distance.get(&next_pos) {
                        if *prevcost <= next_cost {
                            return None;
                        }
                    }

                    if next_cost >= shortest_distance {
                        return None;
                    }

                    distance.insert(next_pos, next_cost);
                    Some(Node {
                        pos: next_pos,
                        cost: next_cost,
                    })
                }),
        );
    }
    shortest_distance
}

fn shortest_path_after(bytes_coords: &[Coords], bytes_to_use: usize, map_size: usize) -> usize {
    let map: FxHashSet<Coords> = bytes_coords[0..bytes_to_use].iter().copied().collect();
    let start = Coords::new(0, 0);
    let end = Coords::new(map_size - 1, map_size - 1);
    // print_map(&map, map_size, start, end);

    find_shortest_path(&map, map_size, start, end)
}

fn part2(bytes_coords: &[Coords]) -> usize {
    0
}

const MAP_SIZE: usize = 70 + 1;
const BYTES_TO_USE: usize = 1024;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes_coords = build(&input);

    println!(
        "Part 1: {}",
        shortest_path_after(&bytes_coords, BYTES_TO_USE, MAP_SIZE)
    );
    println!("Part 2: {}", part2(&bytes_coords));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");
    const MAP_SIZE_TEST: usize = 6 + 1;
    const BYTES_TO_USE_TEST: usize = 12;

    #[test]
    fn test_part1() {
        assert_eq!(
            shortest_path_after(&build(INPUT_TEST), BYTES_TO_USE_TEST, MAP_SIZE_TEST),
            22
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
