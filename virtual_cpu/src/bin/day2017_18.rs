use std::fs;

use virtual_cpu::instruction::Instruction;
use virtual_cpu::registers::Registers;
use virtual_cpu::test_utils;

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

// Executes the instruction specified by ins, modifying the registers if needed.
fn execute_sound_playing(
    instructions: &[Instruction],
    ir: &mut usize,
    regs: &mut Registers<i64>,
    last_sound_played: &mut i64,
) -> Option<i64> {
    let ins = &instructions[*ir];
    match ins {
        Instruction::Snd(x) => {
            // plays a sound with a frequency equal to the value of X
            *last_sound_played = regs.get_ic(*x);
            *ir += 1;
        }
        Instruction::Rcv(x) => {
            if regs.get(*x) != 0 {
                // recovers the frequency of the last sound played
                return Some(*last_sound_played);
            }
            *ir += 1;
        }
        _ => ins.execute(ir, regs),
    }
    None
}

fn recovered_frequency_value(instructions: &[Instruction]) -> i64 {
    let mut regs = Registers::new();
    let mut last_sound_played = 0;
    let mut ir = 0;
    while ir < instructions.len() {
        // println!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        if let Some(recv_snd) =
            execute_sound_playing(instructions, &mut ir, &mut regs, &mut last_sound_played)
        {
            return recv_snd;
        }
    }
    panic!("Didn't find a recovered sound")
}

pub fn part1(input: &str) -> String {
    let instructions = build(input);
    recovered_frequency_value(&instructions).to_string()
}

#[allow(dead_code)]
fn main() {
    let input_file = test_utils::get_input_file("day2017_18");
    let input = fs::read_to_string(input_file).expect("Unable to read input file");
    let res = part1(&input);
    println!("Part 1: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("test_input/day2017_18_input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(recovered_frequency_value(&build(INPUT_TEST_1)), 4);
    }
}
