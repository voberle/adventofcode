use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

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

    fn get_position_of(&self, c: char) -> usize {
        self.values
            .iter()
            .position(|v| *v == c)
            .expect("Didn't find char")
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
fn fewest_steps_from(
    area: &Grid,
    from: usize,
    next_elevation_check_fn: fn(u8, u8) -> bool,
    end_check_fn: fn(&Grid, usize) -> bool,
) -> usize {
    // When position is just a usize, we can use a vector for visited and distance structures.
    let mut visited: Vec<bool> = vec![false; area.values.len()];
    let mut distance: Vec<usize> = vec![usize::MAX; area.values.len()];
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node { pos: from, cost: 0 });

    while let Some(Node { pos, cost }) = queue.pop() {
        visited[pos] = true;

        // if area.values[pos] == 'E' {
        // if area.get_elevation(pos) == 0 {
        if end_check_fn(area, pos) {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        let elevation = area.get_elevation(pos);

        queue.extend(area.next_positions_iter(pos).filter_map(|next_pos| {
            let next_elevation = area.get_elevation(next_pos);
            if next_elevation_check_fn(next_elevation, elevation) {
                return None;
            }

            if visited[next_pos] {
                return None;
            }

            let next_cost = cost + 1;
            if distance[next_pos] <= next_cost {
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

fn fewest_steps_up_from(area: &Grid, start: usize) -> usize {
    fewest_steps_from(
        area,
        start,
        |next_e: u8, e: u8| next_e > e + 1,
        |area, pos| area.values[pos] == 'E',
    )
}

fn fewest_steps_to_best_signal(area: &Grid) -> usize {
    let start = area.get_position_of('S');
    fewest_steps_up_from(area, start)
}

// Part 2 brute-forced by trying all starting points.
fn _fewest_steps_from_best_spot(area: &Grid) -> usize {
    area.values
        .iter()
        .enumerate()
        .filter_map(|(pos, v)| if *v == 'a' { Some(pos) } else { None })
        .map(|start| fewest_steps_up_from(area, start))
        .min()
        .unwrap()
}

// Part 2 but starting from the end.
fn fewest_steps_from_best_spot(area: &Grid) -> usize {
    let end = area.get_position_of('E');
    fewest_steps_from(
        area,
        end,
        |next_e: u8, e: u8| next_e < e - 1,
        |area, pos| area.get_elevation(pos) == 0,
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let area = Grid::build(&input);

    println!("Part 1: {}", fewest_steps_to_best_signal(&area));
    println!("Part 2: {}", fewest_steps_from_best_spot(&area));
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
        assert_eq!(fewest_steps_from_best_spot(&Grid::build(INPUT_TEST)), 29);
    }
}
