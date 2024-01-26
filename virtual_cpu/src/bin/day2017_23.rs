use std::fs;

use virtual_cpu::instruction::Instruction;
use virtual_cpu::registers::Registers;
use virtual_cpu::test_utils;

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

fn execute_common(ins: &Instruction, ir: &mut usize, regs: &mut Registers<i64>) {
    match ins {
        Instruction::Set(x, y) => {
            regs.set(*x, regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::Sub(x, y) => {
            regs.set(*x, regs.get(*x) - regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::JumpNotZero(x, y) => {
            if regs.get_ic(*x) != 0 {
                *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
            } else {
                *ir += 1;
            }
        }
        Instruction::Nop => *ir += 1,
        _ => panic!("Wrong use of this function"),
    }
}

fn execute(instructions: &[Instruction], ir: &mut usize, regs: &mut Registers<i64>) -> bool {
    let ins = &instructions[*ir];
    match ins {
        Instruction::Mul(x, y) => {
            regs.set(*x, regs.get(*x) * regs.get_ic(*y));
            *ir += 1;
            return true;
        }
        _ => execute_common(ins, ir, regs),
    }
    false
}

fn mul_count(instructions: &[Instruction]) -> usize {
    let mut mul_invocations = 0;
    let mut regs = Registers::new();
    let mut ir = 0;
    while ir < instructions.len() {
        if execute(instructions, &mut ir, &mut regs) {
            mul_invocations += 1;
        }
    }
    mul_invocations
}

pub fn part1(input: &str) -> String {
    let instructions = build(input);
    mul_count(&instructions).to_string()
}

fn main() {
    let input_file = test_utils::get_input_file("day2017_23");
    let input = fs::read_to_string(input_file).expect("Unable to read input file");
    let res = part1(&input);
    println!("Part 1: {}", res);
}
