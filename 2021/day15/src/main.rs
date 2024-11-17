use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

// Abstract cave.
trait Cave {
    fn get_rows(&self) -> usize;
    fn get_cols(&self) -> usize;
    fn get_risk_level(&self, pos: usize) -> u32;
    fn get_start(&self) -> usize;
    fn get_destination(&self) -> usize;

    fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.get_rows() {
            for p in row * self.get_cols()..(row + 1) * self.get_cols() {
                let c = self.get_risk_level(p);
                if positions.contains(&p) {
                    print!("{RED}{c}{RESET}");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.print_with_pos(&[]);
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.get_cols(),
            East => pos % self.get_cols() == self.get_cols() - 1,
            South => pos / self.get_cols() == self.get_rows() - 1,
            West => pos % self.get_cols() == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.get_cols(),
            East => pos + 1,
            South => pos + self.get_cols(),
            West => pos - 1,
        }
    }
}

// Base grid / Small cave.
struct Grid {
    values: Vec<u32>,
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
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

impl Cave for Grid {
    fn get_rows(&self) -> usize {
        self.rows
    }

    fn get_cols(&self) -> usize {
        self.cols
    }

    fn get_risk_level(&self, pos: usize) -> u32 {
        self.values[pos]
    }

    fn get_start(&self) -> usize {
        0
    }

    fn get_destination(&self) -> usize {
        self.values.len() - 1
    }
}

// Big cave.
struct BigCave<'a> {
    base: &'a Grid,
}

impl<'a> BigCave<'a> {
    const RATIO: usize = 5;

    fn new(base: &'a Grid) -> Self {
        Self { base }
    }

    fn col(&self, index: usize) -> usize {
        index % self.get_cols()
    }

    fn row(&self, index: usize) -> usize {
        index / self.get_cols()
    }
}

impl<'a> Cave for BigCave<'a> {
    fn get_rows(&self) -> usize {
        self.base.get_rows() * Self::RATIO
    }

    fn get_cols(&self) -> usize {
        self.base.get_cols() * Self::RATIO
    }

    fn get_risk_level(&self, pos: usize) -> u32 {
        // Find which quadrant we are in.
        let row = self.row(pos);
        let col = self.col(pos);

        let row_quadrant = row / self.base.get_rows();
        let col_quadrant = col / self.base.get_cols();
        let risk_extra = u32::try_from(row_quadrant + col_quadrant).unwrap();

        // Corresponding position in the small grid.
        let base_pos = self
            .base
            .pos(row % self.base.get_rows(), col % self.base.get_cols());
        let base_risk = self.base.values[base_pos];

        let risk = base_risk + risk_extra;

        // Risk levels above 9 wrap back around to 1.
        if risk > 9 {
            risk % 10 + 1
        } else {
            risk
        }
    }

    fn get_start(&self) -> usize {
        0
    }

    fn get_destination(&self) -> usize {
        self.get_cols() * self.get_rows() - 1
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
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

fn find_shortest_path<C: Cave>(cave: &C, start: usize, end: usize) -> u32 {
    let mut visited: FxHashSet<usize> = FxHashSet::default();
    let mut distance: FxHashMap<usize, u32> = FxHashMap::default();
    let mut shortest_distance = u32::MAX;

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

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&dir| {
            if !cave.allowed(pos, dir) {
                return None;
            }
            let next_pos = cave.next_pos(pos, dir);

            if visited.contains(&next_pos) {
                return None;
            }

            let next_cost = cost + cave.get_risk_level(next_pos);
            if let Some(prevcost) = distance.get(&next_pos) {
                if *prevcost <= next_cost {
                    return None;
                }
            }

            distance.insert(next_pos, next_cost);
            Some(Node {
                pos: next_pos,
                cost: next_cost,
            })
        }));
    }
    shortest_distance
}

fn lowest_total_risk_small_map(cave: &Grid) -> u32 {
    find_shortest_path(cave, cave.get_start(), cave.get_destination())
}

fn lowest_total_risk_large_map(cave: &Grid) -> u32 {
    let big_cave = BigCave::new(cave);
    // big_cave.print();

    find_shortest_path(&big_cave, big_cave.get_start(), big_cave.get_destination())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cave = Grid::build(&input);

    println!("Part 1: {}", lowest_total_risk_small_map(&cave));
    println!("Part 2: {}", lowest_total_risk_large_map(&cave));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(lowest_total_risk_small_map(&Grid::build(INPUT_TEST)), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(lowest_total_risk_large_map(&Grid::build(INPUT_TEST)), 315);
    }
}
