use fxhash::{FxHashMap, FxHashSet};
use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Free,
    Start,
    End,
}
use Tile::{End, Free, Start, Wall};

impl Tile {
    fn build(c: char) -> Self {
        match c {
            '#' => Wall,
            '.' => Free,
            'S' => Start,
            'E' => End,
            _ => panic!("Invalid tile char"),
        }
    }

    fn is_wall(self) -> bool {
        matches!(self, Wall)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

impl Direction {
    fn is_rotated(self, other: Direction) -> bool {
        match self {
            North | South => [East, West].contains(&other),
            East | West => [North, South].contains(&other),
        }
    }

    // Cost in case of direction change
    fn cost_change(self, other: Direction) -> u32 {
        if self == other {
            0
        } else if self.is_rotated(other) {
            1000
        } else {
            // opposite
            2000
        }
    }
}

struct Grid {
    values: Vec<Tile>,
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
                l.chars().map(Tile::build).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn find_start(&self) -> usize {
        self.values.iter().position(|t| matches!(t, Start)).unwrap()
    }

    fn find_end(&self) -> usize {
        self.values.iter().position(|t| matches!(t, End)).unwrap()
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    dir: Direction,
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
fn find_smallest_cost(map: &Grid, start: usize, start_direction: Direction, end: usize) -> u32 {
    let mut visited: FxHashSet<(usize, Direction)> = FxHashSet::default();
    let mut distance: FxHashMap<(usize, Direction), u32> = FxHashMap::default();
    let mut smallest_cost = u32::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        dir: start_direction,
        cost: 0,
    });

    while let Some(Node { pos, dir, cost }) = queue.pop() {
        visited.insert((pos, dir));

        if pos == end {
            smallest_cost = smallest_cost.min(cost);
            continue;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&d| {
            if !map.allowed(pos, d) {
                // Cannot go outside the map.
                return None;
            }
            // We could exclude going backwards, but on the start position it might make sense.

            let next_pos = map.next_pos(pos, d);
            if visited.contains(&(next_pos, d)) {
                return None;
            }

            if map.values[next_pos].is_wall() {
                return None;
            }

            let next_cost = cost + 1 + dir.cost_change(d);

            if let Some(prevcost) = distance.get(&(next_pos, d)) {
                if *prevcost <= next_cost {
                    // We have visited this place at cheaper
                    return None;
                }
            }

            if next_cost >= smallest_cost {
                return None;
            }

            distance.insert((next_pos, d), next_cost);
            Some(Node {
                pos: next_pos,
                dir: d,
                cost: next_cost,
            })
        }));
    }

    smallest_cost
}

fn lowest_score(map: &Grid) -> u32 {
    let start = map.find_start();
    let end = map.find_end();
    let dir = East;

    find_smallest_cost(map, start, dir, end)
}

fn part2(map: &Grid) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", lowest_score(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(lowest_score(&Grid::build(INPUT_TEST_1)), 7036);
        assert_eq!(lowest_score(&Grid::build(INPUT_TEST_2)), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST_1)), 0);
    }
}
