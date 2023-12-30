use fxhash::FxHashSet;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    pub fn new(c: char) -> Self {
        match c {
            '^' => North,
            '>' => East,
            'v' => South,
            '<' => West,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }

    fn towards(&self, dir: &Direction) -> Pos {
        match dir {
            North => Pos::new(self.row - 1, self.col),
            South => Pos::new(self.row + 1, self.col),
            East => Pos::new(self.row, self.col - 1),
            West => Pos::new(self.row, self.col + 1),
        }
    }
}

fn build_directions(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::new).collect()
}

fn at_least_one_present(input: &str) -> usize {
    let dirs = build_directions(input);
    let mut santa = Pos::new(0, 0);
    let mut visited: FxHashSet<Pos> = FxHashSet::default();
    visited.insert(santa);
    for d in dirs {
        santa = santa.towards(&d);
        visited.insert(santa);
    }
    visited.len()
}

fn part2(input: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", at_least_one_present(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(at_least_one_present(">"), 2);
        assert_eq!(at_least_one_present("^>v<"), 4);
        assert_eq!(at_least_one_present("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
