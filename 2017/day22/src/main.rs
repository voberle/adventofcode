use std::io::{self, Read};

use fxhash::FxHashMap;

type Pos = (i32, i32);
type ComputeGrid = FxHashMap<Pos, bool>;

// Returns the grid and the starting position
fn build(input: &str) -> (ComputeGrid, Pos) {
    let mut rows = 0;
    let mut cols = 0;
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            rows += 1;
            cols = line.len();
            line.chars()
                .enumerate()
                .map(|(col, c)| ((row as i32, col as i32), c == '#'))
                .collect::<Vec<_>>()
        })
        .collect();

    (grid, (rows / 2, cols as i32 / 2))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn turn(self, left: bool) -> Direction {
        let turn = match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        };
        if left {
            turn
        } else {
            turn.opposite()
        }
    }

    fn next_pos(self, pos: Pos) -> Pos {
        match self {
            Up => (pos.0 - 1, pos.1),
            Down => (pos.0 + 1, pos.1),
            Left => (pos.0, pos.1 - 1),
            Right => (pos.0, pos.1 + 1),
        }
    }
}

fn activity_burst(grid: &mut ComputeGrid, pos: &mut Pos, dir: &mut Direction) -> bool {
    let node_infection_status = grid.get(pos).copied().unwrap_or_default();
    // Turn based on infection status
    *dir = dir.turn(!node_infection_status);
    // Flip current node status
    grid.insert(*pos, !node_infection_status);
    // Move to next node
    *pos = dir.next_pos(*pos);
    // Return if we infected a node
    !node_infection_status
}

fn infection_number(grid: &ComputeGrid, start: Pos, activity_bursts: usize) -> usize {
    let mut infection_bursts = 0;

    let mut grid = grid.clone();
    let mut pos = start;
    let mut dir = Up;
    for _ in 0..activity_bursts {
        if activity_burst(&mut grid, &mut pos, &mut dir) {
            infection_bursts += 1;
        }
    }
    infection_bursts
}

fn part2(grid: &ComputeGrid) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (grid, start) = build(&input);

    println!("Part 1: {}", infection_number(&grid, start, 10000));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (grid, start) = build(INPUT_TEST);
        assert_eq!(infection_number(&grid, start, 10000), 5587);
    }

    #[test]
    fn test_part2() {
        let (grid, start) = build(INPUT_TEST);
        assert_eq!(part2(&grid), 0);
    }
}
