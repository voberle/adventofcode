use std::fs;

use virtual_cpu::instruction::{execute_all, Instruction};
use virtual_cpu::registers::Registers;
use virtual_cpu::run_utils;

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
    let input_file = run_utils::get_input_file("day2016_12");
    let input = fs::read_to_string(input_file).expect("Unable to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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
