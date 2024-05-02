use std::io::{self, Read};

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}
use Direction::{East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West};

const ALL_DIRECTIONS: [Direction; 8] = [
    North, East, South, West, NorthEast, NorthWest, SouthEast, SouthWest,
];

#[derive(Clone)]
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

    #[allow(dead_code)]
    fn print(&self) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if c == 0 {
                    print!("{RED}{}{RESET}", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
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

    pub fn direction_forbidden(&self, pos: usize, direction: Direction) -> bool {
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
    pub fn position_in(&self, pos: usize, direction: Direction) -> usize {
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
            .filter(|&&d| !self.direction_forbidden(pos, d))
            .map(|&d| self.position_in(pos, d))
            .collect()
    }
}

fn run_step(octopuses: &mut Grid) -> usize {
    // println!("---------");
    // octopuses.print();

    // Queue of octopuses that need flashing.
    let mut flashing: Vec<usize> = Vec::new();
    // Octopuses that have flashed.
    let mut flashed: Vec<usize> = Vec::new();

    // Increase energy level of all octopuses by 1.
    for (pos, energy_level) in &mut octopuses.values.iter_mut().enumerate() {
        *energy_level += 1;
        if *energy_level > 9 {
            flashing.push(pos);
        }
    }

    // Flash octopuses.
    while let Some(to_flash_pos) = flashing.pop() {
        if flashed.contains(&to_flash_pos) {
            // already flashed, skipping
            continue;
        }

        flashed.push(to_flash_pos);

        let adj_positions: Vec<usize> = octopuses.neighbors(to_flash_pos);
        for adj_pos in adj_positions {
            octopuses.values[adj_pos] += 1;
            if octopuses.values[adj_pos] > 9 {
                flashing.push(adj_pos);
            }
        }
    }

    // Set flashing octopuses level to 0.
    for pos in &flashed {
        octopuses.values[*pos] = 0;
    }

    flashed.len()
}

fn total_flashes(octopuses: &Grid, steps: usize) -> usize {
    let mut octopuses = octopuses.clone();
    (0..steps).map(|_| run_step(&mut octopuses)).sum()
}

fn step_when_all_flash(octopuses: &Grid) -> usize {
    let mut octopuses = octopuses.clone();
    for step in 1.. {
        let flash_count = run_step(&mut octopuses);
        if flash_count == octopuses.values.len() {
            return step;
        }
    }
    panic!("Didn't find when all octopuses flash")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let octopuses = Grid::build(&input);

    println!("Part 1: {}", total_flashes(&octopuses, 100));
    println!("Part 2: {}", step_when_all_flash(&octopuses));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(total_flashes(&Grid::build(INPUT_TEST), 100), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(step_when_all_flash(&Grid::build(INPUT_TEST)), 195);
    }
}
