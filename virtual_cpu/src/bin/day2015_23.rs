use std::fs;

use virtual_cpu::c_code::gen::get_c_code;
use virtual_cpu::instruction::{build_list, execute_all, Instruction};
use virtual_cpu::intchar::IntChar;
use virtual_cpu::parsing::char;
use virtual_cpu::registers::Registers;
use virtual_cpu::run_utils;

fn build_instruction(s: &str) -> Instruction {
    let t = s.replace(',', "");
    let parts: Vec<_> = t.split(' ').collect();
    match *parts.first().unwrap() {
        "tpl" => Instruction::Mul(char(parts[1]), IntChar::from(3)),
        "hlf" => Instruction::Div(char(parts[1]), IntChar::from(2)),
        _ => Instruction::build(&t),
    }
}

fn value_in(instructions: &[Instruction], reg: char) -> i64 {
    let mut regs = Registers::new();
    execute_all(instructions, &mut regs);
    regs.get(reg)
}

fn value_in_with_a_at_1(instructions: &[Instruction], reg: char) -> i64 {
    let mut regs = Registers::new();
    regs.set('a', 1);
    execute_all(instructions, &mut regs);
    regs.get(reg)
}

pub fn part1(input: &str) -> String {
    let instructions = build_list(input, build_instruction);
    value_in(&instructions, 'b').to_string()
}

pub fn part2(input: &str) -> String {
    let instructions = build_list(input, build_instruction);
    value_in_with_a_at_1(&instructions, 'b').to_string()
}

pub fn part1_c_code(input: &str) -> String {
    let instructions = build_list(input, build_instruction);
    get_c_code(&instructions, &Registers::new(), &['b'])
}

pub fn part2_c_code(input: &str) -> String {
    let instructions = build_list(input, build_instruction);
    let mut initial_registers = Registers::new();
    initial_registers.set('a', 1);
    get_c_code(&instructions, &initial_registers, &['b'])
}

#[allow(dead_code)]
fn main() {
    let input_file = run_utils::get_input_file("day2015_23");
    let input = fs::read_to_string(input_file).expect("Unable to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("test_input/day2015_23_input_test_1");

    #[test]
    fn test_part1_2() {
        assert_eq!(value_in(&build_list(INPUT_TEST, build_instruction), 'a'), 2);
        assert_eq!(
            value_in_with_a_at_1(&build_list(INPUT_TEST, build_instruction), 'a'),
            7
        );
    }
}
