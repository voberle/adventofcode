use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

const TRACK: char = '.';
const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

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

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn next_positions_iter(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(d_row, d_col)| {
                (
                    ((pos / self.cols) as isize + d_row) as usize,
                    ((pos % self.cols) as isize + d_col) as usize,
                )
            })
            .filter(|&(row, col)| (row < self.rows && col < self.cols))
            .map(|(row, col)| row * self.cols + col)
    }

    fn find_position_of(&self, element: char) -> usize {
        self.values.iter().position(|c| *c == element).unwrap()
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
// Making it as efficient as possible, since it's called a lot.
fn find_shortest_path(map: &Grid, start: usize, end: usize) -> usize {
    let mut visited: Vec<bool> = vec![false; map.values.len()];
    let mut distance: Vec<usize> = vec![usize::MAX; map.values.len()];
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        visited[pos] = true;

        if pos == end {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        queue.extend(map.next_positions_iter(pos).filter_map(|next_pos| {
            if visited[next_pos] {
                return None;
            }
            if map.values[next_pos] == WALL {
                return None;
            }

            let next_cost = cost + 1;
            if distance[next_pos] <= next_cost {
                return None;
            }

            if next_cost >= shortest_distance {
                return None;
            }

            distance[next_pos] = next_cost;
            Some(Node {
                pos: next_pos,
                cost: next_cost,
            })
        }));
    }
    shortest_distance
}

fn cheats_count(map: &Grid, saving_at_least: usize) -> usize {
    let start = map.find_position_of(START);
    let end = map.find_position_of(END);

    let base_time = find_shortest_path(map, start, end);

    // Shamelessly brute-forced.
    map.values
        .iter()
        .enumerate()
        .filter(|(_, elt)| **elt == WALL)
        .filter(|&(pos, _)| {
            let mut modified_map = map.clone();
            modified_map.values[pos] = TRACK;
            let time = find_shortest_path(&modified_map, start, end);
            time < base_time && base_time - time >= saving_at_least
        })
        .count()
}

fn part2(map: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", cheats_count(&map, 100));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_find_shortest_path() {
        let map = Grid::build(INPUT_TEST);
        let start = map.find_position_of(START);
        let end = map.find_position_of(END);

        assert_eq!(find_shortest_path(&map, start, end), 84);
    }

    #[test]
    fn test_part1() {
        assert_eq!(cheats_count(&Grid::build(INPUT_TEST), 20), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
