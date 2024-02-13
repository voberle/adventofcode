use std::{
    fmt,
    io::{self, Read},
};

mod visualization;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Area {
    OpenGround,
    Tree,
    Lumberyard,
}
use Area::{Lumberyard, OpenGround, Tree};

impl Area {
    fn build(c: char) -> Self {
        match c {
            '.' => OpenGround,
            '|' => Tree,
            '#' => Lumberyard,
            _ => panic!("Invalid area char"),
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                OpenGround => '.',
                Tree => '|',
                Lumberyard => '#',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    values: Vec<Area>,
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
                l.chars().map(Area::build).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    #[allow(dead_code)]
    fn print(&self) {
        const YELLOW: &str = "\x1b[33m";
        const GREEN: &str = "\x1b[32m";
        const BLUE: &str = "\x1b[94m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                match c {
                    OpenGround => print!("{YELLOW}{}{RESET}", c),
                    Tree => print!("{GREEN}{}{RESET}", c),
                    Lumberyard => print!("{BLUE}{}{RESET}", c),
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

    fn direction_forbidden(&self, pos: usize, direction: Direction) -> bool {
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
    fn position_in(&self, pos: usize, direction: Direction) -> usize {
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

    // Get the up to 8 area around
    fn neighbor_areas(&self, pos: usize) -> Vec<Area> {
        ALL_DIRECTIONS
            .iter()
            .filter(|&&d| !self.direction_forbidden(pos, d))
            .map(|&d| self.values[self.position_in(pos, d)])
            .collect()
    }
}

fn transform(grid: &Grid, pos: usize) -> Area {
    let mut a = grid.values[pos];
    let neighbors_area = grid.neighbor_areas(pos);
    match a {
        OpenGround => {
            if neighbors_area.iter().filter(|&&a| a == Tree).count() >= 3 {
                a = Tree;
            }
        }
        Tree => {
            if neighbors_area.iter().filter(|&&a| a == Lumberyard).count() >= 3 {
                a = Lumberyard;
            }
        }
        Lumberyard => {
            if neighbors_area.iter().any(|&a| a == Lumberyard)
                && neighbors_area.iter().any(|&a| a == Tree)
            {
                a = Lumberyard;
            } else {
                a = OpenGround;
            }
        }
    }
    a
}

fn advance_one_minute(grid: &mut Grid) {
    grid.values = (0..grid.values.len())
        .map(|pos| transform(grid, pos))
        .collect();
}

fn total_resource_value(grid: &Grid) -> usize {
    let wooden_area = grid.values.iter().filter(|&&a| a == Tree).count();
    let lumberyards = grid.values.iter().filter(|&&a| a == Lumberyard).count();
    wooden_area * lumberyards
}

fn resource_after_10_min(lumber_collection: &Grid) -> usize {
    let mut grid = lumber_collection.clone();
    for _ in 0..10 {
        advance_one_minute(&mut grid);
    }
    total_resource_value(&grid)
}

fn resource_after_1000_years(lumber_collection: &Grid) -> usize {
    // The pattern becomes periodic.
    const TIME: usize = 1_000_000_000;
    const START_OFFSET: usize = 1010;

    let mut grid = lumber_collection.clone();

    // Period doesn't start immediately, moving ahead.
    for _ in 0..START_OFFSET {
        advance_one_minute(&mut grid);
    }

    let period_start_grid = grid.clone();
    let mut period = 0;
    loop {
        period += 1;
        advance_one_minute(&mut grid);
        if grid.values == period_start_grid.values {
            break;
        }
    }

    let jump_ahead_start = START_OFFSET + ((TIME - START_OFFSET) / period) * period;
    for _ in jump_ahead_start..TIME {
        advance_one_minute(&mut grid);
    }

    total_resource_value(&grid)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lumber_collection = Grid::build(&input);
    // lumber_collection.print();

    let param = std::env::args().nth(1).unwrap_or_default();
    if param == "visu" {
        visualization::fancy(&lumber_collection).unwrap();
        return;
    }

    println!("Part 1: {}", resource_after_10_min(&lumber_collection));
    println!("Part 2: {}", resource_after_1000_years(&lumber_collection));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(resource_after_10_min(&Grid::build(INPUT_TEST)), 1147);
    }
}
