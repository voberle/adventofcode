use std::io::{self, Read};

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    #[allow(clippy::cast_sign_loss)]
    fn exec(&self, ip: &mut usize, accumulator: &mut i32) {
        match self {
            Instruction::Acc(arg) => {
                *accumulator += arg;
                *ip += 1;
            }
            Instruction::Jmp(arg) => {
                let signed_ip = i32::try_from(*ip).unwrap() + *arg;
                assert!(signed_ip >= 0);
                *ip = signed_ip as usize;
            }
            Instruction::Nop(_) => {
                *ip += 1;
            }
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

fn accumulator_after_one_run(instructions: &[Instruction]) -> i32 {
    let mut ip = 0;
    let mut accumulator = 0;

    let mut exec_count = vec![0; instructions.len()];
    loop {
        if exec_count[ip] > 0 {
            break;
        }
        let ins = &instructions[ip];
        exec_count[ip] += 1;

        ins.exec(&mut ip, &mut accumulator);
    }
    accumulator
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", accumulator_after_one_run(&instructions));
    println!("Part 2: {}", part2(&instructions));
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
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
