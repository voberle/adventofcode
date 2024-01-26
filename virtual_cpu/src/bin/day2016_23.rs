use std::fs;

use virtual_cpu::instruction::Instruction;
use virtual_cpu::intchar::IntChar;
use virtual_cpu::parsing::char;
use virtual_cpu::registers::Registers;
use virtual_cpu::test_utils;

fn build_instruction(s: &str) -> Instruction {
    let parts: Vec<_> = s.split(' ').collect();
    match *parts.first().unwrap() {
        "tgl" => Instruction::Toggle(char(parts[1])),
        _ => Instruction::build(s),
    }
}

fn build_list(input: &str) -> Vec<Instruction> {
    input.lines().map(build_instruction).collect()
}

// Executes the instruction specified by ins, modifying the registers if needed.
fn execute(instructions: &mut Vec<Instruction>, ir: &mut usize, regs: &mut Registers<i64>) {
    let ins = &instructions[*ir];
    if let Instruction::Toggle(offset) = ins {
        let ir_to_toggle = (*ir as i64 + regs.get(*offset)) as usize;
        if ir_to_toggle < instructions.len() {
            instructions[ir_to_toggle] = match &instructions[ir_to_toggle] {
                Instruction::Copy(x, r) => Instruction::JumpNotZero(*x, IntChar::Char(*r)),
                Instruction::Increase(r) => Instruction::Decrease(*r),
                Instruction::Decrease(r) => Instruction::Increase(*r),
                Instruction::JumpNotZero(v, o) => match o {
                    IntChar::Integer(_) => Instruction::Nop,
                    IntChar::Char(r) => Instruction::Copy(*v, *r),
                },
                Instruction::Toggle(offset) => Instruction::Increase(*offset),
                // Instruction::Mult(a, b, r) => Instruction::Mult(*a, *b, *r),
                _ => ins.clone(),
            };
        }
        *ir += 1;
    } else {
        ins.execute(ir, regs);
    }
}

fn execute_all(instructions: &[Instruction], regs: &mut Registers<i64>) {
    let mut modifiable_ins: Vec<Instruction> = instructions.to_vec();
    // optimizer(&mut modifiable_ins);

    let mut ir = 0;
    while ir < modifiable_ins.len() {
        execute(&mut modifiable_ins, &mut ir, regs);
    }
}

fn value_sent_to_safe(instructions: &[Instruction]) -> i64 {
    let mut regs = Registers::new();
    regs.set('a', 7);
    execute_all(instructions, &mut regs);
    regs.get('a')
}

pub fn part1(input: &str) -> String {
    let instructions = build_list(input);
    value_sent_to_safe(&instructions).to_string()
}

#[allow(dead_code)]
fn main() {
    let input_file = test_utils::get_input_file("day2017_23");
    let input = fs::read_to_string(input_file).expect("Unable to read input file");
    let res = part1(&input);
    println!("Part 1: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("test_input/day2016_23_input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(value_sent_to_safe(&build_list(INPUT_TEST)), 3);
    }
}
