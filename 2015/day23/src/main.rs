use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
struct Registers {
    a: u32,
    b: u32,
}

impl Registers {
    fn new() -> Self {
        Self { a: 0, b: 0 }
    }
}

impl Index<char> for Registers {
    type Output = u32;

    fn index(&self, reg: char) -> &Self::Output {
        match reg {
            'a' => &self.a,
            'b' => &self.b,
            _ => panic!("Invalid index {}", reg),
        }
    }
}

impl IndexMut<char> for Registers {
    fn index_mut(&mut self, reg: char) -> &mut Self::Output {
        match reg {
            'a' => &mut self.a,
            'b' => &mut self.b,
            _ => panic!("Invalid index {}", reg),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(i32),
    JumpIfEven(char, i32),
    JumpIfOne(char, i32), // jump if one", not odd
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match parts[0] {
            "hlf" => Self::Half(parts[1].chars().next().unwrap()),
            "tpl" => Self::Triple(parts[1].chars().next().unwrap()),
            "inc" => Self::Increment(parts[1].chars().next().unwrap()),
            "jmp" => Self::Jump(parts[1].parse().unwrap()),
            "jie" => Self::JumpIfEven(parts[1].chars().next().unwrap(), parts[2].parse().unwrap()),
            "jio" => Self::JumpIfOne(parts[1].chars().next().unwrap(), parts[2].parse().unwrap()),
            _ => panic!("Unknown instruction"),
        }
    }

    // Executes the instruction, modifying the registers if needed, and returns the next instruction ID.
    fn execute(&self, ir: usize, regs: &mut Registers) -> usize {
        match self {
            Instruction::Half(r) => {
                regs[*r] /= 2;
                ir + 1
            }
            Instruction::Triple(r) => {
                regs[*r] *= 3;
                ir + 1
            }
            Instruction::Increment(r) => {
                regs[*r] += 1;
                ir + 1
            }
            Instruction::Jump(offset) => (ir as i32 + offset) as usize,
            Instruction::JumpIfEven(r, offset) => {
                if regs[*r] % 2 == 0 {
                    (ir as i32 + offset) as usize
                } else {
                    ir + 1
                }
            }
            Instruction::JumpIfOne(r, offset) => {
                if regs[*r] == 1 {
                    (ir as i32 + offset) as usize
                } else {
                    ir + 1
                }
            }
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

fn execute_all(instructions: &[Instruction]) -> Registers {
    let mut regs = Registers::new();
    let mut ir = 0; // instruction register
    while ir < instructions.len() {
        // print!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        ir = instructions[ir].execute(ir, &mut regs);
        // println!("; next {}", ir);
    }
    regs
}

fn value_in_at_end(instructions: &[Instruction], reg: char) -> u32 {
    let regs = execute_all(instructions);
    regs[reg]
}

fn part2(instructions: &[Instruction]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", value_in_at_end(&instructions, 'b'));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(value_in_at_end(&build(INPUT_TEST), 'a'), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
