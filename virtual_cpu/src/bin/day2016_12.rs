use std::fs;

use virtual_cpu::instruction::Instruction;
use virtual_cpu::registers::Registers;
use virtual_cpu::test_utils;

fn execute_all(instructions: &[Instruction], regs: &mut Registers<i64>) {
    let mut ir = 0;
    while ir < instructions.len() {
        instructions[ir].execute(&mut ir, regs);
    }
}

fn value_in_reg_a(instructions: &[Instruction]) -> i64 {
    let mut regs = Registers::new();
    execute_all(instructions, &mut regs);
    regs.get('a')
}

fn value_in_reg_a_with_c_at_1(instructions: &[Instruction]) -> i64 {
    let mut regs = Registers::new();
    regs.set('c', 1);
    execute_all(instructions, &mut regs);
    regs.get('a')
}

pub fn part1(input: &str) -> String {
    let instructions = Instruction::build_list(input);
    value_in_reg_a(&instructions).to_string()
}

pub fn part2(input: &str) -> String {
    let instructions = Instruction::build_list(input);
    value_in_reg_a_with_c_at_1(&instructions).to_string()
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

    const INPUT_TEST: &str = include_str!("test_input/day2016_12_input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(value_in_reg_a(&Instruction::build_list(INPUT_TEST)), 42);
    }
}
