use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T>
where
    T: Default,
    T: Clone,
{
    pub values: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Grid<T>
where
    T: Default,
    T: Clone,
{
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            values: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    const fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
}

type ActionFn = fn(bool) -> bool; // For part 1
type Action2Fn = fn(u32) -> u32; // For part 2

#[derive(Debug)]
struct Instruction {
    action: ActionFn,
    action2: Action2Fn,
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
                    action2: |val| val + 1,
                    p1,
                    p2,
                },
                "toggle" => Instruction {
                    action: |val| val ^ true,
                    action2: |val| val + 2,
                    p1,
                    p2,
                },
                "turn off" => Instruction {
                    action: |_| false,
                    action2: |val| if val == 0 { 0 } else { val - 1 },
                    p1,
                    p2,
                },
                _ => panic!("Invalid instruction {}", &c[1]),
            }
        })
        .collect()
}

fn count_lights_on(grid: &Grid<bool>) -> usize {
    grid.values.iter().filter(|v| **v).count()
}

fn lit_lights_count(instructions: &[Instruction]) -> usize {
    let mut grid = Grid::new(1000, 1000);
    for ins in instructions {
        for r in ins.p1.row..=ins.p2.row {
            for c in ins.p1.col..=ins.p2.col {
                let idx = grid.pos(r, c);
                grid.values[idx] = (ins.action)(grid.values[idx]);
            }
        }
    }
    count_lights_on(&grid)
}

fn count_total_brightness(grid: &Grid<u32>) -> u32 {
    grid.values.iter().sum()
}

fn lit_lights_count_ind_brightness(instructions: &[Instruction]) -> u32 {
    let mut grid = Grid::new(1000, 1000);
    for ins in instructions {
        for r in ins.p1.row..=ins.p2.row {
            for c in ins.p1.col..=ins.p2.col {
                let idx = grid.pos(r, c);
                grid.values[idx] = (ins.action2)(grid.values[idx]);
            }
        }
    }
    count_total_brightness(&grid)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", lit_lights_count(&instructions));
    println!("Part 2: {}", lit_lights_count_ind_brightness(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(lit_lights_count(&build(INPUT_TEST_1)), 998996);
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part2() {
        assert_eq!(
            lit_lights_count_ind_brightness(&build(INPUT_TEST_2)),
            2000001
        );
    }
}
