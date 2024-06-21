use std::io::{self, Read};

use fxhash::FxHashMap;

type Pos = (i32, i32);
type ComputeGrid = FxHashMap<Pos, NodeState>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}
use NodeState::{Clean, Flagged, Infected, Weakened};

impl NodeState {
    fn flip(self) -> Self {
        match self {
            Clean => Infected,
            Infected => Clean,
            _ => panic!("Extended states not supported"),
        }
    }

    fn next(self) -> Self {
        match self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,
        }
    }

    fn is_infected(self) -> bool {
        self == Infected
    }
}

// Returns the grid and the starting position
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
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
                .map(|(col, c)| {
                    (
                        (row as i32, col as i32),
                        if c == '#' { Infected } else { Clean },
                    )
                })
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

    fn turn_base(self, current_node_state: NodeState) -> Direction {
        let turn_left = !current_node_state.is_infected();
        let turn = match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        };
        if turn_left {
            turn
        } else {
            turn.opposite()
        }
    }

    fn turn_evolved(self, current_node_state: NodeState) -> Direction {
        match current_node_state {
            Clean => self.turn_base(NodeState::Clean),
            Weakened => self,
            Infected => self.turn_base(NodeState::Infected),
            Flagged => self.opposite(),
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

fn activity_burst(
    grid: &mut ComputeGrid,
    pos: &mut Pos,
    dir: &mut Direction,
    turn_fn: fn(Direction, NodeState) -> Direction,
    change_state_fn: fn(NodeState) -> NodeState,
) -> bool {
    let node_infection_state = grid.get(pos).copied().unwrap_or(Clean);
    // Turn based on infection status
    *dir = turn_fn(*dir, node_infection_state);

    // Flip current node status
    let next_state = change_state_fn(node_infection_state);
    grid.insert(*pos, next_state);

    // Move to next node
    *pos = dir.next_pos(*pos);

    // Return if we infected a node
    next_state.is_infected()
}

fn infection_number(
    grid: &ComputeGrid,
    start: Pos,
    activity_bursts: usize,
    turn_fn: fn(Direction, NodeState) -> Direction,
    change_state_fn: fn(NodeState) -> NodeState,
) -> usize {
    let mut infection_bursts = 0;

    let mut grid = grid.clone();
    let mut pos = start;
    let mut dir = Up;
    for _ in 0..activity_bursts {
        if activity_burst(&mut grid, &mut pos, &mut dir, turn_fn, change_state_fn) {
            infection_bursts += 1;
        }
    }
    infection_bursts
}

fn infection_base(grid: &ComputeGrid, start: Pos, activity_bursts: usize) -> usize {
    infection_number(
        grid,
        start,
        activity_bursts,
        Direction::turn_base,
        NodeState::flip,
    )
}

fn infection_evolved(grid: &ComputeGrid, start: Pos, activity_bursts: usize) -> usize {
    infection_number(
        grid,
        start,
        activity_bursts,
        Direction::turn_evolved,
        NodeState::next,
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (grid, start) = build(&input);

    println!("Part 1: {}", infection_base(&grid, start, 10000));
    println!("Part 2: {}", infection_evolved(&grid, start, 10_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (grid, start) = build(INPUT_TEST);
        assert_eq!(infection_base(&grid, start, 10000), 5587);
    }

    #[test]
    fn test_part2() {
        let (grid, start) = build(INPUT_TEST);
        assert_eq!(infection_evolved(&grid, start, 100), 26);
        assert_eq!(infection_evolved(&grid, start, 10_000_000), 2511944);
    }
}
