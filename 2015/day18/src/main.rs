use std::io::{self, Read};

pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}
use Direction::*;

const ALL_DIRECTIONS: [Direction; 8] = [
    North, East, South, West, NorthEast, NorthWest, SouthEast, SouthWest,
];

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub values: Vec<char>,
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn build(input: &str) -> Self {
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

    pub fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("{RED}{}{RESET}", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }

    pub fn print(&self) {
        self.print_with_pos(&[]);
    }

    fn north_forbidden(&self, pos: usize) -> bool {
        pos < self.cols
    }

    fn east_forbidden(&self, pos: usize) -> bool {
        pos % self.cols == self.cols - 1
    }

    fn south_forbidden(&self, pos: usize) -> bool {
        pos / self.cols == self.rows - 1
    }

    fn west_forbidden(&self, pos: usize) -> bool {
        pos % self.cols == 0
    }

    pub fn direction_forbidden(&self, pos: usize, direction: &Direction) -> bool {
        match direction {
            North => self.north_forbidden(pos),
            East => self.east_forbidden(pos),
            South => self.south_forbidden(pos),
            West => self.west_forbidden(pos),
            NorthEast => self.north_forbidden(pos) || self.east_forbidden(pos),
            NorthWest => self.north_forbidden(pos) || self.west_forbidden(pos),
            SouthEast => self.south_forbidden(pos) || self.east_forbidden(pos),
            SouthWest => self.south_forbidden(pos) || self.west_forbidden(pos),
        }
    }

    // Assumes validity of the move has been checked before with `can_go`.
    pub fn position_in(&self, pos: usize, direction: &Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
            NorthEast => pos - self.cols + 1,
            NorthWest => pos - self.cols - 1,
            SouthEast => pos + self.cols + 1,
            SouthWest => pos + self.cols - 1,
        }
    }

    // Get the up to 8 positions around
    pub fn neighbors(&self, pos: usize) -> Vec<usize> {
        ALL_DIRECTIONS
            .iter()
            .filter(|d| !self.direction_forbidden(pos, d))
            .map(|d| self.position_in(pos, d))
            .collect()
    }
}

fn lights_count<const STEPS: usize>(grid: &mut Grid, block_corner_lights: bool) -> usize {
    for _ in 0..STEPS {
        grid.values = grid
            .values
            .iter()
            .enumerate()
            .map(|(pos, val)| {
                let neighbors_on = grid
                    .neighbors(pos)
                    .iter()
                    .filter(|n| grid.values[**n] == '#')
                    .count();
                if *val == '#' {
                    // ON
                    if neighbors_on == 2 || neighbors_on == 3 {
                        '#' // stays on when 2 or 3 neighbors are on
                    } else {
                        '.'
                    }
                } else {
                    // OFF
                    if neighbors_on == 3 {
                        '#' // turns on if exactly 3 neighbors are on
                    } else {
                        '.'
                    }
                }
            })
            .collect();
        if block_corner_lights {
            turn_corner_lights_on(grid);
        }
    }
    // grid.print();
    grid.values.iter().filter(|e| **e == '#').count()
}

fn lights_count_part1<const STEPS: usize>(grid: &Grid) -> usize {
    let mut g = grid.clone();
    lights_count::<STEPS>(&mut g, false)
}

fn pos(cols: usize, row: usize, col: usize) -> usize {
    row * cols + col
}

fn turn_corner_lights_on(grid: &mut Grid) {
    grid.values[0] = '#';
    grid.values[pos(grid.cols, grid.rows - 1, 0)] = '#';
    grid.values[pos(grid.cols, 0, grid.cols - 1)] = '#';
    grid.values[pos(grid.cols, grid.rows - 1, grid.cols - 1)] = '#';
}

fn lights_count_part2<const STEPS: usize>(grid: &Grid) -> usize {
    let mut g = grid.clone();
    turn_corner_lights_on(&mut g);
    lights_count::<STEPS>(&mut g, true)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);
    // grid.print();

    println!("Part 1: {}", lights_count_part1::<100>(&grid));
    println!("Part 2: {}", lights_count_part2::<100>(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(lights_count_part1::<4>(&Grid::build(INPUT_TEST)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(lights_count_part2::<5>(&Grid::build(INPUT_TEST)), 17);
    }
}
