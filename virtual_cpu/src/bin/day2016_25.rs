use std::fs;

use virtual_cpu::instruction::Instruction;
use virtual_cpu::intchar::IntChar;
use virtual_cpu::registers::Registers;
use virtual_cpu::test_utils;

fn build_instruction(s: &str) -> Instruction {
    let parts: Vec<_> = s.split(' ').collect();
    match *parts.first().unwrap() {
        "out" => Instruction::Out(IntChar::new(parts[1])),
        _ => Instruction::build(s),
    }
}

fn build_list(input: &str) -> Vec<Instruction> {
    input.lines().map(build_instruction).collect()
}

// Executes the instruction specified by ins, modifying the registers if needed.
// Returns the output of "out" command if it was executed.
fn execute(instructions: &[Instruction], ir: &mut usize, regs: &mut Registers<i64>) -> Option<i64> {
    let ins = &instructions[*ir];
    if let Instruction::Out(x) = ins {
        *ir += 1;
        return Some(regs.get_ic(*x));
    }
    ins.execute(ir, regs);
    None
}

fn is_periodic_clock_signal(clock_signal: &[i64]) -> bool {
    let mut periodic = true;
    let mut iter = clock_signal.iter().peekable();
    while let Some(s) = iter.next() {
        if let Some(n) = iter.peek() {
            periodic &= *s != **n;
        }
        if !periodic {
            break;
        }
    }
    periodic
}

fn execute_all<const MAX_CLOCK_SIGNALS_TO_CHECK: usize>(
    instructions: &[Instruction],
    mut regs: Registers<i64>,
) -> bool {
    let mut ir = 0;
    let mut clock_signal: Vec<i64> = Vec::new();
    while ir < instructions.len()
        && is_periodic_clock_signal(&clock_signal)
        && clock_signal.len() < MAX_CLOCK_SIGNALS_TO_CHECK
    {
        let out = execute(instructions, &mut ir, &mut regs);
        if let Some(s) = out {
            assert!(s == 0 || s == 1);
            clock_signal.push(s);
        }
    }
    is_periodic_clock_signal(&clock_signal)
}

fn lowest_possible_int(instructions: &[Instruction]) -> i64 {
    let mut a = 0;
    while a < 10_000 {
        let mut regs = Registers::new();
        regs.set('a', a);
        if execute_all::<40>(instructions, regs) {
            break;
        }
        a += 1;
    }
    a
}

pub fn part1(input: &str) -> String {
    let instructions = build_list(input);
    lowest_possible_int(&instructions).to_string()
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

    #[test]
    fn test_is_periodic_clock_signal() {
        assert!(is_periodic_clock_signal(&[0, 1, 0, 1, 0]));
        assert!(is_periodic_clock_signal(&[1, 0, 1, 0]));
        assert!(is_periodic_clock_signal(&[0, 1, 0, 1, 0, 1]));

        assert!(!is_periodic_clock_signal(&[0, 1, 1, 0, 0]));
        assert!(!is_periodic_clock_signal(&[0, 0]));
    }
}
