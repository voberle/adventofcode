use std::{
    fmt,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl SeatState {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Invalid state"),
        }
    }
}

impl fmt::Display for SeatState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Floor => '.',
                Self::Empty => 'L',
                Self::Occupied => '#',
            }
        )
    }
}

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

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<SeatState>,
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
                l.chars().map(SeatState::new).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                print!("{}", self.values[p]);
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

    fn direction_forbidden(&self, pos: usize, direction: &Direction) -> bool {
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

    fn position_in(&self, pos: usize, direction: &Direction) -> usize {
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

    fn count_adjacents_occupied(&self, pos: usize) -> usize {
        ALL_DIRECTIONS
            .iter()
            .filter(|d| !self.direction_forbidden(pos, d))
            .map(|d| self.position_in(pos, d))
            .filter(|&p| self.values[p] == SeatState::Occupied)
            .count()
    }

    fn count_adjacents_occupied_all_direction(&self, pos: usize) -> usize {
        ALL_DIRECTIONS
            .iter()
            .filter(|d| {
                let mut p = pos;
                loop {
                    if self.direction_forbidden(p, d) {
                        return false;
                    }
                    p = self.position_in(p, d);
                    if self.values[p] == SeatState::Occupied {
                        return true;
                    }
                    if self.values[p] == SeatState::Empty {
                        return false;
                    }
                }
            })
            .count()
    }

    fn apply_rule(&self, pos: usize, occ_count: usize, occupied_limit: usize) -> SeatState {
        match self.values.get(pos) {
            Some(SeatState::Empty) => {
                if occ_count == 0 {
                    return SeatState::Occupied;
                }
            }
            Some(SeatState::Occupied) => {
                if occ_count >= occupied_limit {
                    return SeatState::Empty;
                }
            }
            _ => {}
        }
        self.values[pos]
    }

    fn count_occupied_seats(&self) -> usize {
        self.values
            .iter()
            .filter(|&&v| v == SeatState::Occupied)
            .count()
    }
}

fn occupied_count_end(
    grid: &Grid,
    count_adjacents_occupied_fn: fn(&Grid, usize) -> usize,
    occupied_limit: usize,
) -> usize {
    let mut grid = grid.clone();
    let mut next_values: Vec<SeatState> = Vec::new();
    loop {
        for pos in 0..grid.values.len() {
            let occ_count = count_adjacents_occupied_fn(&grid, pos);
            next_values.push(grid.apply_rule(pos, occ_count, occupied_limit));
        }

        if grid.values == next_values {
            break;
        }

        std::mem::swap(&mut grid.values, &mut next_values);
        next_values.clear();
    }

    grid.count_occupied_seats()
}

fn occupied_count_end_rule1(grid: &Grid) -> usize {
    occupied_count_end(grid, Grid::count_adjacents_occupied, 4)
}

fn occupied_count_end_rule2(grid: &Grid) -> usize {
    occupied_count_end(grid, Grid::count_adjacents_occupied_all_direction, 5)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", occupied_count_end_rule1(&grid));
    println!("Part 2: {}", occupied_count_end_rule2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(occupied_count_end_rule1(&Grid::build(INPUT_TEST)), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(occupied_count_end_rule2(&Grid::build(INPUT_TEST)), 26);
    }
}
