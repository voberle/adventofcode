// https://adventofcode.com/2023/day/18

use std::io::{self, BufRead};

#[derive(Debug)]
struct Instruction {
    direction: char,
    meters: u32,
    color: String,
}

impl Instruction {
    fn new(direction: char, meters: u32, color: String) -> Self {
        Self { direction, meters, color }
    }

    fn build(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        Self {
            direction: parts[0].chars().next().unwrap(),
            meters: parts[1].parse().unwrap(),
            color: parts[2].to_string(),
        }
    }
}

fn capacity(dig_plan: &Vec<Instruction>) -> u32 {

    0
}

fn build_dig_plan<R>(reader: &mut R) -> Vec<Instruction> where R: BufRead
{
    reader.lines().map(|l| {
        let line = l.unwrap();
        Instruction::build(&line)
    })
    .collect()
}

fn main() {
    let stdin = io::stdin();
    let dig_plan = build_dig_plan(&mut stdin.lock());
    // println!("{:?}", dig_plan);

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
        println!("{}", dig_plan);
        assert_eq!(capacity(&dig_plan), 62);
    }
}
