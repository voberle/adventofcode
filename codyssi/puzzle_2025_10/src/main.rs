use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

#[derive(Debug, Clone, PartialEq)]
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
                    .filter(|c| !c.is_ascii_whitespace())
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

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }
}

fn safest_danger_level(grid: &Grid) -> u32 {
    let min_horizontal_danger: u32 = (0..grid.rows)
        .map(|row| {
            (0..grid.cols)
                .map(|col| grid.values[grid.pos(row, col)])
                .sum()
        })
        .min()
        .unwrap();
    let min_vertical_danger: u32 = (0..grid.cols)
        .map(|col| {
            (0..grid.rows)
                .map(|row| grid.values[grid.pos(row, col)])
                .sum()
        })
        .min()
        .unwrap();
    min_horizontal_danger.min(min_vertical_danger)
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

// Dijkstra shortest path.
fn find_shortest_path(grid: &Grid, start: usize, end: usize) -> u32 {
    let mut visited: Vec<bool> = vec![false; grid.values.len()];
    let mut distance: Vec<u32> = vec![u32::MAX; grid.values.len()];
    let mut shortest_distance = u32::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: grid.values[start],
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        visited[pos] = true;

        if pos == end {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        // Going only down or right.
        queue.extend([(1, 0), (0, 1)].into_iter().filter_map(|(d_r, d_c)| {
            let next_row = grid.row(pos) + d_r;
            let next_col = grid.col(pos) + d_c;

            if next_row >= grid.rows || next_col >= grid.cols {
                return None;
            }

            let next_pos = grid.pos(next_row, next_col);

            if visited[next_pos] {
                return None;
            }

            let next_cost = cost + grid.values[next_pos];
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

fn safest_danger_level_to_15x15(grid: &Grid) -> u32 {
    let start = grid.pos(0, 0);
    let end = grid.pos(14, 14);

    find_shortest_path(grid, start, end)
}

fn safest_danger_level_to_end(grid: &Grid) -> u32 {
    let start = 0;
    let end = grid.values.len() - 1;

    find_shortest_path(grid, start, end)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", safest_danger_level(&grid));
    println!("Part 2: {}", safest_danger_level_to_15x15(&grid));
    println!("Part 3: {}", safest_danger_level_to_end(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let grid = Grid::build(&INPUT_TEST);
        assert_eq!(safest_danger_level(&grid), 73);
    }

    #[test]
    fn test_part2() {
        let grid = Grid::build(&INPUT_TEST);
        assert_eq!(safest_danger_level_to_15x15(&grid), 94);
    }

    #[test]
    fn test_part3() {
        let grid = Grid::build(&INPUT_TEST);
        assert_eq!(safest_danger_level_to_end(&grid), 120);
    }
}
