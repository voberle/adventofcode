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
            _ => panic!("Invalid index {}", reg),
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
            _ => panic!("Invalid index {}", reg),
        }
    }
}

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[derive(Debug, Clone)]
enum Instruction {
    // Same instructions as Day 12
    CopyVal(i32, char),
    CopyReg(char, char),
    Increase(char),
    Decrease(char),
    JumpIfNotZeroValVal(i32, i32),
    JumpIfNotZeroValReg(i32, char),
    JumpIfNotZeroRegVal(char, i32),
    JumpIfNotZeroRegReg(char, char),
    // New instruction
    Toggle(char),
    Invalid,
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
                if let Ok(val) = parts[1].parse() {
                    if let Ok(offset) = parts[2].parse() {
                        Self::JumpIfNotZeroValVal(val, offset)
                    } else {
                        Self::JumpIfNotZeroValReg(val, char(parts[2]))
                    }
                } else if let Ok(offset) = parts[2].parse() {
                    Self::JumpIfNotZeroRegVal(char(parts[1]), offset)
                } else {
                    Self::JumpIfNotZeroRegReg(char(parts[1]), char(parts[2]))
                }
            }
            "tgl" => Self::Toggle(char(parts[1])),
            _ => panic!("Unknown instruction"),
        }
    }
}

// Executes the instruction specified by ins, modifying the registers if needed, and returns the next instruction ID.
fn execute(instructions: &mut Vec<Instruction>, ir: usize, regs: &mut Registers) -> usize {
    let ins = &instructions[ir];
    match ins {
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
        Instruction::JumpIfNotZeroValVal(val, offset) => {
            if *val != 0 {
                (ir as i32 + offset) as usize
            } else {
                ir + 1
            }
        }
        Instruction::JumpIfNotZeroValReg(val, offset) => {
            if *val != 0 {
                (ir as i32 + regs[*offset]) as usize
            } else {
                ir + 1
            }
        }
        Instruction::JumpIfNotZeroRegVal(r, offset) => {
            if regs[*r] != 0 {
                (ir as i32 + offset) as usize
            } else {
                ir + 1
            }
        }
        Instruction::JumpIfNotZeroRegReg(r, offset) => {
            if regs[*r] != 0 {
                (ir as i32 + regs[*offset]) as usize
            } else {
                ir + 1
            }
        }
        Instruction::Toggle(offset) => {
            let ir_to_toggle = (ir as i32 + regs[*offset]) as usize;
            if ir_to_toggle >= instructions.len() {
                return ir + 1;
            }
            let ins_to_toggle = &instructions[ir_to_toggle];
            instructions[ir_to_toggle] = match ins_to_toggle {
                Instruction::CopyVal(val, r) => Instruction::JumpIfNotZeroValReg(*val, *r),
                Instruction::CopyReg(src, r) => Instruction::JumpIfNotZeroRegReg(*src, *r),
                Instruction::Increase(r) => Instruction::Decrease(*r),
                Instruction::Decrease(r) => Instruction::Increase(*r),
                Instruction::JumpIfNotZeroValReg(v, o) => Instruction::CopyVal(*v, *o),
                Instruction::JumpIfNotZeroRegReg(r, offset) => Instruction::CopyReg(*r, *offset),
                Instruction::Toggle(offset) => Instruction::Increase(*offset),
                Instruction::JumpIfNotZeroValVal(_, _)
                | Instruction::JumpIfNotZeroRegVal(_, _)
                | Instruction::Invalid => Instruction::Invalid,
            };
            ir + 1
        }
        Instruction::Invalid => ir + 1,
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

fn execute_all(instructions: &[Instruction], mut regs: Registers) -> Registers {
    let mut modifiable_ins: Vec<Instruction> = instructions.to_vec();
    let mut ir = 0;
    while ir < modifiable_ins.len() {
        // print!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        ir = execute(&mut modifiable_ins, ir, &mut regs);
        // println!("; next {}", ir);
    }
    regs
}

fn value_sent_to_safe(instructions: &[Instruction]) -> i32 {
    execute_all(instructions, Registers::new(7, 0, 0, 0))['a']
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", value_sent_to_safe(&instructions));
    //println!("Part 2: {}", value_in_reg_a_with_c_at_1(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(value_sent_to_safe(&build(INPUT_TEST)), 3);
    }
}
