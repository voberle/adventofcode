use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    fn turn_right_90_degrees(self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

impl From<Direction> for usize {
    fn from(d: Direction) -> Self {
        match d {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }
}

struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
    guard_starting_position: usize,
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
        let guard_starting_position = values.iter().position(|&c| c == '^').unwrap();
        Self {
            values,
            rows,
            cols,
            guard_starting_position,
        }
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            Up => pos < self.cols,
            Right => pos % self.cols == self.cols - 1,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
        }
    }

    // Assumes validity of the move has been checked before with `allowed`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Right => pos + 1,
            Down => pos + self.cols,
            Left => pos - 1,
        }
    }
}

fn visited_positions_count(map: &Grid) -> usize {
    // A grid of the same size as the map to mark the visited positions.
    let mut visited = vec![false; map.values.len()];

    let mut guard_pos = map.guard_starting_position;
    visited[guard_pos] = true;

    let mut direction = Direction::Up;
    while map.allowed(guard_pos, direction) {
        let next_pos = map.next_pos(guard_pos, direction);
        match map.values.get(next_pos) {
            Some('.' | '^') => {
                guard_pos = next_pos;
                visited[guard_pos] = true;
            }
            Some('#') => {
                direction = direction.turn_right_90_degrees();
            }
            _ => panic!("Invalid map element"),
        }
    }

    visited.iter().filter(|&&v| v).count()
}

// Returns true if we reach a loop, false if we get out.
fn walk_until_loop(
    map: &Grid,
    extra_obstacle_pos: usize,
    mut guard_pos: usize,
    mut direction: Direction,
    mut visited: Vec<[bool; 4]>,
) -> bool {
    while map.allowed(guard_pos, direction) {
        let next_pos = map.next_pos(guard_pos, direction);
        if visited[next_pos][usize::from(direction)] {
            // Got a loop.
            // println!("");
            // print_with_visited(map, extra_obstacle_pos, &visited);

            return true;
        }

        if next_pos == extra_obstacle_pos {
            direction = direction.turn_right_90_degrees();
        } else {
            match map.values.get(next_pos) {
                Some('.' | '^') => {
                    guard_pos = next_pos;
                    visited[guard_pos][usize::from(direction)] = true;
                }
                Some('#') => {
                    direction = direction.turn_right_90_degrees();
                }
                _ => panic!("Invalid map element"),
            }
        }
    }
    false
}

#[allow(dead_code)]
fn print_with_visited(map: &Grid, extra_obstacle_pos: usize, visited: &[[bool; 4]]) {
    for row in 0..map.rows {
        for (p, visit) in visited
            .iter()
            .enumerate()
            .take((row + 1) * map.cols)
            .skip(row * map.cols)
        {
            if p == extra_obstacle_pos {
                print!("O");
                continue;
            }
            match map.values.get(p) {
                Some('#') => print!("#"),
                Some('^') => print!("^"),
                Some('.') => {
                    // This doesn't print + for all the corners, but it's good enough to debug.
                    if visit.iter().any(|v| *v) {
                        if !visit[usize::from(Up)] && !visit[usize::from(Down)] {
                            print!("-");
                        } else if !visit[usize::from(Left)] && !visit[usize::from(Right)] {
                            print!("|");
                        } else {
                            print!("+");
                        }
                    } else {
                        print!(".");
                    }
                }
                _ => panic!("Invalid map element"),
            }
        }
        println!();
    }
}

fn obstruction_positions_count(map: &Grid) -> usize {
    // A loop happens when we reach a previously visited place with the same direction.
    // So as we walk through the map, on each step we try to place an obstruction and check if we reach a loop.

    let mut obstructions_count = 0;

    // Visited positions with directions.
    let mut visited: Vec<[bool; 4]> = vec![[false; 4]; map.values.len()];

    let mut guard_pos = map.guard_starting_position;
    visited[guard_pos][usize::from(Up)] = true;

    let mut direction = Direction::Up;
    while map.allowed(guard_pos, direction) {
        let next_pos = map.next_pos(guard_pos, direction);
        match map.values.get(next_pos) {
            Some('.' | '^') => {
                // If next position is free, test if putting an obstacle would result in a loop.
                // The new obstruction can't be placed at the guard's starting position.
                if next_pos != map.guard_starting_position
                    && walk_until_loop(map, next_pos, guard_pos, direction, visited.clone())
                {
                    obstructions_count += 1;
                }

                guard_pos = next_pos;
                visited[guard_pos][usize::from(direction)] = true;
            }
            Some('#') => {
                direction = direction.turn_right_90_degrees();
            }
            _ => panic!("Invalid map element"),
        }
    }

    obstructions_count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", visited_positions_count(&map));
    println!("Part 2: {}", obstruction_positions_count(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(visited_positions_count(&Grid::build(INPUT_TEST)), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(obstruction_positions_count(&Grid::build(INPUT_TEST)), 6);
    }
}
