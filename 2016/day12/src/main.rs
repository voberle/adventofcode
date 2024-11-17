use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Registers {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self { a, b, c, d }
    }
}

impl Index<char> for Registers {
    type Output = i32;

    fn index(&self, reg: char) -> &Self::Output {
        match reg {
            'a' => &self.a,
            'b' => &self.b,
            'c' => &self.c,
            'd' => &self.d,
            _ => panic!("Invalid index {reg}"),
        }
    }
}

impl IndexMut<char> for Registers {
    fn index_mut(&mut self, reg: char) -> &mut Self::Output {
        match reg {
            'a' => &mut self.a,
            'b' => &mut self.b,
            'c' => &mut self.c,
            'd' => &mut self.d,
            _ => panic!("Invalid index {reg}"),
        }
    }
}

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[derive(Debug)]
enum Instruction {
    CopyVal(i32, char),
    CopyReg(char, char),
    Increase(char),
    Decrease(char),
    JumpIfNotZeroVal(i32, i32),
    JumpIfNotZeroReg(char, i32),
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "cpy" => {
                if let Ok(val) = parts[1].parse() {
                    Self::CopyVal(val, char(parts[2]))
                } else {
                    Self::CopyReg(char(parts[1]), char(parts[2]))
                }
            }
            "inc" => Self::Increase(char(parts[1])),
            "dec" => Self::Decrease(char(parts[1])),
            "jnz" => {
                let offset = parts[2].parse().unwrap();
                if let Ok(val) = parts[1].parse() {
                    Self::JumpIfNotZeroVal(val, offset)
                } else {
                    Self::JumpIfNotZeroReg(char(parts[1]), offset)
                }
            }
            _ => panic!("Unknown instruction"),
        }
    }

    // Executes the instruction, modifying the registers if needed, and returns the next instruction ID.
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap
    )]
    fn execute(&self, ir: usize, regs: &mut Registers) -> usize {
        match self {
            Instruction::CopyVal(val, r) => {
                regs[*r] = *val;
                ir + 1
            }
            Instruction::CopyReg(src, r) => {
                regs[*r] = regs[*src];
                ir + 1
            }
            Instruction::Increase(r) => {
                regs[*r] += 1;
                ir + 1
            }
            Instruction::Decrease(r) => {
                regs[*r] -= 1;
                ir + 1
            }
            Instruction::JumpIfNotZeroVal(val, offset) => {
                if *val != 0 {
                    (ir as i32 + offset) as usize
                } else {
                    ir + 1
                }
            }
            Instruction::JumpIfNotZeroReg(r, offset) => {
                if regs[*r] != 0 {
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

fn execute_all(instructions: &[Instruction], mut regs: Registers) -> Registers {
    let mut ir = 0;
    while ir < instructions.len() {
        // print!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        ir = instructions[ir].execute(ir, &mut regs);
        // println!("; next {}", ir);
    }
    regs
}

fn value_in_reg_a(instructions: &[Instruction]) -> i32 {
    execute_all(instructions, Registers::new(0, 0, 0, 0))['a']
}

fn value_in_reg_a_with_c_at_1(instructions: &[Instruction]) -> i32 {
    execute_all(instructions, Registers::new(0, 0, 1, 0))['a']
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", value_in_reg_a(&instructions));
    println!("Part 2: {}", value_in_reg_a_with_c_at_1(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(value_in_reg_a(&build(INPUT_TEST)), 42);
    }
}
