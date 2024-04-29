use std::io::{self, Read};

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}
use Instruction::{Down, Forward, Up};

fn build(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if let Some(val) = line.strip_prefix("forward ") {
                Forward(val.parse().unwrap())
            } else if let Some(val) = line.strip_prefix("down ") {
                Down(val.parse().unwrap())
            } else if let Some(val) = line.strip_prefix("up ") {
                Up(val.parse().unwrap())
            } else {
                panic!("Invalid instruction")
            }
        })
        .collect()
}

fn final_hor_x_depth(instructions: &[Instruction]) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    for ins in instructions {
        match ins {
            Forward(val) => position += val,
            Down(val) => depth += val,
            Up(val) => depth -= val,
        }
    }
    position * depth
}

fn final_hor_x_depth_new_meaning(instructions: &[Instruction]) -> u32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for ins in instructions {
        match ins {
            Forward(val) => {
                position += val;
                depth += val * aim;
            }
            Down(val) => aim += val,
            Up(val) => aim -= val,
        }
    }
    position * depth
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", final_hor_x_depth(&instructions));
    println!("Part 2: {}", final_hor_x_depth_new_meaning(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(final_hor_x_depth(&build(INPUT_TEST)), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(final_hor_x_depth_new_meaning(&build(INPUT_TEST)), 900);
    }
}
