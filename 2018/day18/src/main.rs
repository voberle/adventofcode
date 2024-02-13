use std::{
    fmt,
    io::{self, Read},
};

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
use Direction::*;

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

    fn print_with_pos(&self, positions: &[usize]) {
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

    fn print(&self) {
        self.print_with_pos(&[]);
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

    fn pos_as_str(&self, index: usize) -> String {
        format!("({},{})", self.row(index), self.col(index))
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

    // Get the up to 8 positions around
    fn neighbors(&self, pos: usize) -> Vec<usize> {
        ALL_DIRECTIONS
            .iter()
            .filter(|&&d| !self.direction_forbidden(pos, d))
            .map(|&d| self.position_in(pos, d))
            .collect()
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

fn total_resource_value(lumber_collection: &Grid, time: usize) -> usize {
    let mut grid = lumber_collection.clone();
    for _ in 0..time {
        grid.values = (0..grid.values.len())
            .map(|pos| transform(&grid, pos))
            .collect();
    }
    let wooden_area = grid.values.iter().filter(|&&a| a == Tree).count();
    let lumberyards = grid.values.iter().filter(|&&a| a == Lumberyard).count();
    wooden_area * lumberyards
}

fn part2(lumber_collection: &Grid) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lumber_collection = Grid::build(&input);

    println!("Part 1: {}", total_resource_value(&lumber_collection, 10));
    println!("Part 2: {}", part2(&lumber_collection));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(total_resource_value(&Grid::build(INPUT_TEST), 10), 1147);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
