use std::fs;

use virtual_cpu::instruction::Instruction;
use virtual_cpu::registers::Registers;
use virtual_cpu::test_utils;

fn execute(instructions: &[Instruction], ir: &mut usize, regs: &mut Registers<i64>) -> bool {
    let ins = &instructions[*ir];
    match ins {
        // Overwriting Mul from default
        Instruction::Mul(x, y) => {
            regs.set(*x, regs.get(*x) * regs.get_ic(*y));
            *ir += 1;
            return true;
        }
        _ => ins.execute(ir, regs),
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
    let instructions = Instruction::build_list(input);
    mul_count(&instructions).to_string()
}

#[allow(dead_code)]
fn main() {
    let input_file = test_utils::get_input_file("day2017_23");
    let input = fs::read_to_string(input_file).expect("Unable to read input file");
    let res = part1(&input);
    println!("Part 1: {}", res);
}
