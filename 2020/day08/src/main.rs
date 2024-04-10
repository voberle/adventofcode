use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    #[allow(clippy::cast_sign_loss)]
    fn exec(self, ip: &mut usize, accumulator: &mut i32) {
        match self {
            Instruction::Acc(arg) => {
                *accumulator += arg;
                *ip += 1;
            }
            Instruction::Jmp(arg) => {
                let signed_ip = i32::try_from(*ip).unwrap() + arg;
                assert!(signed_ip >= 0);
                *ip = signed_ip as usize;
            }
            Instruction::Nop(_) => {
                *ip += 1;
            }
        }
    }

    fn invert(self) -> Option<Self> {
        match self {
            Self::Jmp(arg) => Some(Self::Nop(arg)),
            Self::Nop(arg) => Some(Self::Jmp(arg)),
            Self::Acc(_) => None,
        }
    }
}

#[allow(clippy::match_on_vec_items)]
fn build(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let p: Vec<_> = line.split_whitespace().collect();
            let arg = p[1].parse().unwrap();
            match p[0] {
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jmp(arg),
                "nop" => Instruction::Nop(arg),
                _ => panic!("Invalid instruction"),
            }
        })
        .collect()
}

// Return true if stopped by repeating an instruction, false if we stopped because the program ended.
fn exec_until_repeat(instructions: &[Instruction], accumulator: &mut i32) -> bool {
    let mut ip = 0;

    // Detecting when instructions repeat.
    let mut ins_already_executed = vec![false; instructions.len()];
    while ip < instructions.len() {
        if ins_already_executed[ip] {
            return true;
        }
        let ins = &instructions[ip];
        ins_already_executed[ip] = true;

        ins.exec(&mut ip, accumulator);
    }
    false
}

fn accumulator_after_one_run(instructions: &[Instruction]) -> i32 {
    let mut accumulator = 0;
    exec_until_repeat(instructions, &mut accumulator);
    accumulator
}

fn accumulator_after_fix(instructions: &[Instruction]) -> i32 {
    for i in 0..instructions.len() {
        let mut instructions = instructions.to_vec();

        if let Some(inv) = instructions[i].invert() {
            instructions[i] = inv;

            let mut accumulator = 0;
            // If we repeat an instruction, it means we are in a loop and can give up.
            if !exec_until_repeat(&instructions, &mut accumulator) {
                return accumulator;
            }
        }
    }
    panic!("No answer found")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", accumulator_after_one_run(&instructions));
    println!("Part 2: {}", accumulator_after_fix(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(accumulator_after_one_run(&build(INPUT_TEST)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(accumulator_after_fix(&build(INPUT_TEST)), 8);
    }
}
