use std::io::{self, Read};

enum Instruction {
    Noop,
    AddX(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value == "noop" {
            Self::Noop
        } else {
            Self::AddX(
                value
                    .strip_prefix("addx ")
                    .expect("Not a addx")
                    .parse()
                    .expect("Not a value"),
            )
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Into::into).collect()
}

fn signal_strengths_sum(instructions: &[Instruction]) -> i32 {
    let checkpoints = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    let mut x = 1;
    let mut cycle = 0;

    for ins in instructions {
        match ins {
            Instruction::Noop => {
                cycle += 1;
                if checkpoints.contains(&cycle) {
                    // println!("{} * {} = {}", cycle, x, cycle * x);
                    sum += cycle * x;
                }
            }
            Instruction::AddX(v) => {
                cycle += 1;
                if checkpoints.contains(&cycle) {
                    // println!("{} * {} = {}", cycle, x, cycle * x);
                    sum += cycle * x;
                }
                cycle += 1;
                if checkpoints.contains(&cycle) {
                    // println!("{} * {} = {}", cycle, x, cycle * x);
                    sum += cycle * x;
                }

                x += v;
            }
        }
    }
    sum
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", signal_strengths_sum(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(signal_strengths_sum(&build(INPUT_TEST)), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
