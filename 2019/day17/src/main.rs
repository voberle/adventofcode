use std::io::{self, Read};

use intcode::IntcodeComputer;

fn get_scaffolds_view(computer: &IntcodeComputer) -> Vec<char> {
    let mut scaffolds: Vec<char> = Vec::new();

    let mut computer = computer.clone();
    computer.exec();

    while let Some(i) = computer.io.get_output() {
        scaffolds.push(char::from_u32(u32::try_from(i).unwrap()).unwrap());
    }

    scaffolds
}

fn print_scaffolds_view(scaffolds: &[char]) {
    for c in scaffolds {
        print!("{}", c);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn convert(scaffolds: &[char]) -> Self {
        let cols = scaffolds.iter().position(|c| *c == '\n').unwrap();
        let mut values = scaffolds.to_vec();
        values.retain(|&c| c != '\n');
        let rows = values.len() / cols;
        Self { values, rows, cols }
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn is_scaffold_in(&self, pos: usize, direction: Direction) -> bool {
        if self.allowed(pos, direction) {
            let np = self.next_pos(pos, direction);
            self.values[np] == '#'
        } else {
            false
        }
    }

    // An intersection is a point with scaffolds in all 4 directions.
    fn is_intersection(&self, pos: usize) -> bool {
        self.values[pos] == '#'
            && self.is_scaffold_in(pos, North)
            && self.is_scaffold_in(pos, South)
            && self.is_scaffold_in(pos, West)
            && self.is_scaffold_in(pos, East)
    }

    fn get_intersections(&self) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(pos, _)| {
                if self.is_intersection(pos) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_alignment_parameter(&self, pos: usize) -> usize {
        self.col(pos) * self.row(pos)
    }
}

fn alignment_params_sum(computer: &IntcodeComputer) -> usize {
    let scaffolds_view = get_scaffolds_view(computer);
    print_scaffolds_view(&scaffolds_view);

    let scaffolds = Grid::convert(&scaffolds_view);
    let intersections = scaffolds.get_intersections();

    intersections
        .iter()
        .map(|p| scaffolds.get_alignment_parameter(*p))
        .sum()
}

fn part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", alignment_params_sum(&computer));
    println!("Part 2: {}", part2(&computer));
}
