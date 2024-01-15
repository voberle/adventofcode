use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    // Check we don't go outside grid.
    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `allowed`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn is_wall(&self, pos: usize) -> bool {
        self.values[pos] == '#'
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
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
fn find_shortest_path(grid: &Grid, start: usize, end: usize) -> usize {
    let mut visited: FxHashSet<usize> = FxHashSet::default();
    let mut distance: FxHashMap<usize, usize> = FxHashMap::default();
    // let mut previous: FxHashMap<usize, usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        // Mark node as visited
        visited.insert(pos);

        if pos == end {
            shortest_distance = usize::min(shortest_distance, cost);
            continue;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|d| {
            // Not allowed: Going outside grid
            if !grid.allowed(pos, *d) {
                return None;
            }
            // we could also optimize not going backwards at this level

            let next_pos = grid.next_pos(pos, *d);
            if grid.is_wall(next_pos) {
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
            // previous.insert(next_pos, pos);

            Some(Node {
                pos: next_pos,
                cost: next_cost,
            })
        }));
    }
    shortest_distance
}

// Returns the shortest path visiting all the nodes.
// Brute-force approach.
fn shortest_path_visiting_all_nodes(grid: &Grid, nodes: &[usize], start: usize) -> usize {
    // https://www.baeldung.com/cs/shortest-path-visiting-all-nodes
    let mut result = usize::MAX;
    let distances: Vec<Vec<usize>> = nodes
        .iter()
        .map(|a| {
            nodes
                .iter()
                .map(|b| find_shortest_path(grid, *a, *b))
                .collect()
        })
        .collect();

    let permutations = (0..nodes.len()).permutations(nodes.len());

    for permutation in permutations {
        // we filter out the permutations that don't start at the beginning
        if permutation[0] != start {
            continue;
        }
        let mut cost = 0;
        let mut previous = permutation[0];
        for node in &permutation {
            cost += distances[previous][*node];
            previous = *node;
        }
        if cost < result {
            // println!("{:?}: New low: {}", permutation, cost);
            result = cost;
        }
    }
    result
}

fn all_numbers_positions(map: &Grid) -> Vec<usize> {
    map.values
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
        .sorted_by_key(|(_, c)| *c) // sorting by numbers of the table, to ease debugging
        .map(|(p, _)| p)
        .collect()
}

fn shortest_visit_all_once(grid: &Grid) -> usize {
    let positions_to_visit = all_numbers_positions(grid);
    // since positions_to_visit is sorted by grid numbers, and we want to start at 0
    // 0 is also the position of the start
    let start_pos = 0;
    shortest_path_visiting_all_nodes(grid, &positions_to_visit, start_pos)
}

fn part2(grid: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", shortest_visit_all_once(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(shortest_visit_all_once(&Grid::build(INPUT_TEST)), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
