use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub values: Vec<bool>,
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            values: vec![false; rows * cols],
            rows,
            cols,
        }
    }

    pub fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub const fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
}

type ActionFn = fn(bool) -> bool;

#[derive(Debug)]
struct Instruction {
    action: ActionFn,
    p1: Point,
    p2: Point,
}

fn build(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"(.+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let p1 = Point::new(c[2].parse().unwrap(), c[3].parse().unwrap());
            let p2 = Point::new(c[4].parse().unwrap(), c[5].parse().unwrap());
            assert!(p1.row <= p2.row);
            assert!(p1.col <= p2.col);
            match &c[1] {
                "turn on" => Instruction {
                    action: |_| true,
                    p1,
                    p2,
                },
                "toggle" => Instruction {
                    action: |val| val ^ true,
                    p1,
                    p2,
                },
                "turn off" => Instruction {
                    action: |_| false,
                    p1,
                    p2,
                },
                _ => panic!("Invalid instruction {}", &c[1]),
            }
        })
        .collect()
}

fn exec_action_on_rect(grid: &mut Grid, ins: &Instruction) {
    for r in ins.p1.row..=ins.p2.row {
        for c in ins.p1.col..=ins.p2.col {
            let idx = grid.pos(r, c);
            grid.values[idx] = (ins.action)(grid.values[idx]);
        }
    }
}

fn count_lights_on(grid: &Grid) -> usize {
    grid.values.iter().filter(|v| **v).count()
}

fn lit_lights_count(instructions: &[Instruction]) -> usize {
    let mut grid = Grid::new(1000, 1000);
    for ins in instructions {
        exec_action_on_rect(&mut grid, ins);
    }
    count_lights_on(&grid)
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", lit_lights_count(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(lit_lights_count(&build(INPUT_TEST)), 998996);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
