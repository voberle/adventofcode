use std::{
    fmt,
    io::{self, Read},
};

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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Instruction::Acc(arg) => write!(f, "acc {}", arg),
            Instruction::Jmp(arg) => write!(f, "jmp {}", arg),
            Instruction::Nop(arg) => write!(f, "nop {}", arg),
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

#[allow(dead_code)]
fn print(instructions: &[Instruction]) {
    for ins in instructions {
        println!("{}", ins);
    }
}

fn accumulator_after_one_run(instructions: &[Instruction]) -> i32 {
    let mut ip = 0;
    let mut accumulator = 0;

    // Detecting when instructions repeat.
    let mut exec_count = vec![0; instructions.len()];
    while ip < instructions.len() {
        if exec_count[ip] > 0 {
            break;
        }
        let ins = &instructions[ip];
        exec_count[ip] += 1;

        ins.exec(&mut ip, &mut accumulator);
    }
    accumulator
}

fn exec(instructions: &[Instruction], max_ins_to_exec: usize) -> Option<i32> {
    let mut ip = 0;
    let mut accumulator = 0;

    let mut c = 0;
    while ip < instructions.len() {
        let ins = &instructions[ip];
        ins.exec(&mut ip, &mut accumulator);

        c += 1;
        if c > max_ins_to_exec {
            return None;
        }
    }
    // println!("Value found after {} instructions", c);
    Some(accumulator)
}

fn accumulator_after_fix(instructions: &[Instruction]) -> i32 {
    for i in 0..instructions.len() {
        let mut instructions = instructions.to_vec();

        if let Some(inv) = instructions[i].invert() {
            instructions[i] = inv;
            // print(&instructions);

            // Initially I set the value to 100_000, and it was still super fast, but 1000 is enough.
            if let Some(acc) = exec(&instructions, 1000) {
                // println!("Modified instruction: {}", i);
                return acc;
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
