// https://adventofcode.com/2023/day/18

use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: char,
    meters: i32,
    color: String,
}

impl Instruction {
    fn new(direction: char, meters: i32, color: String) -> Self {
        Self {
            direction,
            meters,
            color,
        }
    }

    fn build(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        Self {
            direction: parts[0].chars().next().unwrap(),
            meters: parts[1].parse().unwrap(),
            color: parts[2].to_string(),
        }
    }

    // Digs these instructions. The new position is the last item of the result.
    fn dig(&self, start: &Pos) -> Vec<Pos> {
        let range = 1..self.meters + 1;
        match self.direction {
            'U' => range.map(|i| Pos::new(start.row - i, start.col)).collect(),
            'D' => range.map(|i| Pos::new(start.row + i, start.col)).collect(),
            'L' => range.map(|i| Pos::new(start.row, start.col - i)).collect(),
            'R' => range.map(|i| Pos::new(start.row, start.col + i)).collect(),
            _ => panic!("Invalid direction char {}", self.direction),
        }
    }
}

#[test]
fn test_instruction_dig() {
    let start = Pos::new(0, 0);
    let ins = Instruction::build("R 2 (#70c710)");
    assert_eq!(ins.dig(&start), vec![Pos::new(0, 1), Pos::new(0, 2)])
}

fn dig(dig_plan: &Vec<Instruction>) -> Vec<Pos> {
    let mut trench: Vec<Pos> = Vec::new();
    let mut current = &Pos::new(0, 0);
    for ins in dig_plan {
        trench.extend(ins.dig(current));
        current = trench.last().unwrap();
    }
    trench
}

fn print_trench(trench: &[Pos]) {
    let min_row = trench.iter().map(|p| p.row).min().unwrap();
    let min_col = trench.iter().map(|p| p.col).min().unwrap();
    let max_row = trench.iter().map(|p| p.row).max().unwrap();
    let max_col = trench.iter().map(|p| p.col).max().unwrap();
    for row in min_row..max_row + 1 {
        for col in min_col..max_col + 1 {
            let p = Pos::new(row, col);
            print!("{}", if trench.contains(&p) { "#" } else { "." });
        }
        println!();
    }
}

fn capacity(dig_plan: &Vec<Instruction>) -> u32 {
    0
}

fn build_dig_plan<R>(reader: &mut R) -> Vec<Instruction>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            Instruction::build(&line)
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let dig_plan = build_dig_plan(&mut stdin.lock());
    // println!("{:?}", dig_plan);

    let trench = dig(&dig_plan);
    print_trench(&trench);

    println!("Part 1: {}", capacity(&dig_plan));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let dig_plan = build_dig_plan(&mut reader);
        assert_eq!(capacity(&dig_plan), 62);
    }
}
