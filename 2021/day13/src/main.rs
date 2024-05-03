use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

enum FoldInstruction {
    X(u32),
    Y(u32),
}

#[derive(Clone)]
struct Paper(FxHashSet<Coord>);

impl Paper {
    // Returns top left and bottom right corners
    fn borders(&self) -> (Coord, Coord) {
        // Not using iterator min / max to keep only one loop.
        let mut min_x = u32::MAX;
        let mut max_x = u32::MIN;
        let mut min_y = u32::MAX;
        let mut max_y = u32::MIN;
        for c in &self.0 {
            min_x = min_x.min(c.x);
            max_x = max_x.max(c.x);
            min_y = min_y.min(c.y);
            max_y = max_y.max(c.y);
        }
        (Coord::new(min_x, min_y), Coord::new(max_x, max_y))
    }

    fn count_dots(&self) -> usize {
        self.0.len()
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (top_left, bottom_right) = self.borders();
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                print!(
                    "{}",
                    if self.0.contains(&Coord::new(x, y)) {
                        '\u{2B1B}'
                    } else {
                        '\u{2B1C}'
                    }
                );
            }
            println!();
        }
    }

    fn fold(&self, instruction: &FoldInstruction) -> Self {
        let coords = self
            .0
            .iter()
            .map(|c| match instruction {
                FoldInstruction::X(x) => {
                    if c.x <= *x {
                        *c
                    } else {
                        Coord::new(x - x.abs_diff(c.x), c.y)
                    }
                }
                FoldInstruction::Y(y) => {
                    if c.y <= *y {
                        *c
                    } else {
                        Coord::new(c.x, y - y.abs_diff(c.y))
                    }
                }
            })
            .collect();
        Self(coords)
    }
}

fn build(input: &str) -> (Paper, Vec<FoldInstruction>) {
    let mut it = input.lines();
    let mut coords: FxHashSet<Coord> = FxHashSet::default();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        let (x, y) = line
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        coords.insert(Coord::new(x, y));
    }

    let mut instructions = Vec::new();
    for line in it {
        let ins = if let Some(x) = line.strip_prefix("fold along x=") {
            FoldInstruction::X(x.parse().unwrap())
        } else if let Some(y) = line.strip_prefix("fold along y=") {
            FoldInstruction::Y(y.parse().unwrap())
        } else {
            panic!("Invalid instruction")
        };
        instructions.push(ins);
    }

    (Paper(coords), instructions)
}

fn dots_after_folding_first_instruction(paper: &Paper, instructions: &[FoldInstruction]) -> usize {
    let folded_paper = paper.fold(instructions.first().unwrap());
    folded_paper.count_dots()
}

fn code_after_folding(paper: &Paper, instructions: &[FoldInstruction]) {
    let mut paper = paper.clone();
    for ins in instructions {
        paper = paper.fold(ins);
    }
    paper.print();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (paper, instructions) = build(&input);

    println!(
        "Part 1: {}",
        dots_after_folding_first_instruction(&paper, &instructions)
    );
    println!("Part 2:");
    code_after_folding(&paper, &instructions);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (paper, instructions) = build(INPUT_TEST);
        assert_eq!(
            dots_after_folding_first_instruction(&paper, &instructions),
            17
        );
    }
}
