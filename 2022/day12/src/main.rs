use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

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
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn get_elevation(&self, pos: usize) -> u8 {
        let c = self.values[pos];
        match c {
            'S' => 0,
            'E' => b'z' - b'a',
            _ => c as u8 - b'a',
        }
    }
}

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

// Dijkstra shortest path.
fn fewest_steps_to_best_signal(area: &Grid) -> usize {
    let start = area
        .values
        .iter()
        .position(|v| *v == 'S')
        .expect("Didn't find start");

    let mut visited: FxHashSet<usize> = FxHashSet::default();
    let mut distance: FxHashMap<usize, usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        visited.insert(pos);

        if area.values[pos] == 'E' {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        let elevation = area.get_elevation(pos);

        queue.extend(
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(move |(d_row, d_col)| {
                    (
                        ((pos / area.cols) as isize + d_row) as usize,
                        ((pos % area.cols) as isize + d_col) as usize,
                    )
                })
                .filter(|&(row, col)| (row < area.rows && col < area.cols))
                .map(|(row, col)| row * area.cols + col)
                .filter_map(|next_pos| {
                    let next_elevation = area.get_elevation(next_pos);
                    if next_elevation > elevation + 1 {
                        // Too high.
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
                    Some(Node {
                        pos: next_pos,
                        cost: next_cost,
                    })
                }),
        );
    }
    shortest_distance
}

fn part2(area: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let area = Grid::build(&input);

    println!("Part 1: {}", fewest_steps_to_best_signal(&area));
    println!("Part 2: {}", part2(&area));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(fewest_steps_to_best_signal(&Grid::build(INPUT_TEST)), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
