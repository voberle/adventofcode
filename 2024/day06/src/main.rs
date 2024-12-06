use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
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

    fn get_initial_position(&self) -> usize {
        self.values.iter().position(|&c| c == '^').unwrap()
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

    let mut guard_pos = map.get_initial_position();
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

fn part2(map: &Grid) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", visited_positions_count(&map));
    println!("Part 2: {}", part2(&map));
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
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
