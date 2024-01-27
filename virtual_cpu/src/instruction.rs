use crate::intchar::IntChar;
use crate::parsing::char;
use crate::registers::Registers;

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    NotZero,
    GreaterThanZero,
    True,
    Even,
    EqualOne,
}
use Condition::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Set(char, IntChar<i64>),
    Add(char, IntChar<i64>),
    Sub(char, IntChar<i64>),
    Mul(char, IntChar<i64>),
    Mod(char, IntChar<i64>),
    Div(char, IntChar<i64>),
    JumpIf(Condition, IntChar<i64>, IntChar<i64>, fn(i64) -> bool),
    Nop,

    // Day 2017 18
    Snd(IntChar<i64>),
    Rcv(char), // In theory could be IntChar but input and part 2 limits to a register.
    // Day 2016 12, 23, 25
    Out(IntChar<i64>),
    Toggle(char),
}

impl Instruction {
    pub fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        Self::build_from_parts(parts.first().unwrap(), &parts[1..])
    }

    pub fn build_from_parts(ins: &str, p: &[&str]) -> Self {
        match ins {
            "set" => Self::Set(char(p[0]), IntChar::new(p[1])),
            "cpy" => Self::Set(char(p[1]), IntChar::new(p[0])), // params inversed vs set
            "add" => Self::Add(char(p[0]), IntChar::new(p[1])),
            "inc" => Self::Add(char(p[0]), IntChar::from(1)),
            "sub" => Self::Sub(char(p[0]), IntChar::new(p[1])),
            "dec" => Self::Sub(char(p[0]), IntChar::from(1)),
            "mul" => Self::Mul(char(p[0]), IntChar::new(p[1])),
            "mod" => Self::Mod(char(p[0]), IntChar::new(p[1])),
            "div" => Self::Div(char(p[0]), IntChar::new(p[1])),
            "jnz" => Self::JumpIf(NotZero, IntChar::new(p[0]), IntChar::new(p[1]), |v| v != 0),
            "jgz" => Self::JumpIf(
                GreaterThanZero,
                IntChar::new(p[0]),
                IntChar::new(p[1]),
                |v| v > 0,
            ),
            "jmp" => Self::JumpIf(True, IntChar::from(0), IntChar::new(p[0]), |_| true),
            "jie" => Self::JumpIf(Even, IntChar::new(p[0]), IntChar::new(p[1]), |v| v % 2 == 0),
            "jio" => Self::JumpIf(EqualOne, IntChar::new(p[0]), IntChar::new(p[1]), |v| v == 1),
            "nop" => Self::Nop,
            _ => panic!("Unknown instruction"),
        }
    }

    pub fn execute(&self, ir: &mut usize, regs: &mut Registers<i64>) {
        match self {
            Instruction::Set(x, y) => {
                regs.set(*x, regs.get_ic(*y));
                *ir += 1;
            }
            Instruction::Add(x, y) => {
                regs.set(*x, regs.get(*x) + regs.get_ic(*y));
                *ir += 1;
            }
            Instruction::Sub(x, y) => {
                regs.set(*x, regs.get(*x) - regs.get_ic(*y));
                *ir += 1;
            }
            Instruction::Mul(x, y) => {
                regs.set(*x, regs.get(*x) * regs.get_ic(*y));
                *ir += 1;
            }
            Instruction::Mod(x, y) => {
                regs.set(*x, regs.get(*x) % regs.get_ic(*y));
                *ir += 1;
            }
            Instruction::Div(x, y) => {
                regs.set(*x, regs.get(*x) / regs.get_ic(*y));
                *ir += 1;
            }
            Instruction::JumpIf(_, x, y, test_fn) => {
                if test_fn(regs.get_ic(*x)) {
                    *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
                } else {
                    *ir += 1;
                }
            }
            Instruction::Nop => *ir += 1,
            _ => panic!("Unsupported instruction in Instruction::execute()"),
        }
    }

    pub fn build_list(input: &str) -> Vec<Instruction> {
        input.lines().map(Instruction::build).collect()
    }

    /// Returns the list of register names used by this instruction, if any.
    pub fn get_register_names(&self) -> Vec<char> {
        let mut regs = Vec::new();
        match self {
            Instruction::Set(x, y)
            | Instruction::Add(x, y)
            | Instruction::Sub(x, y)
            | Instruction::Mul(x, y)
            | Instruction::Mod(x, y)
            | Instruction::Div(x, y) => {
                regs.push(*x);
                if let IntChar::Char(c) = y {
                    regs.push(*c);
                }
            }
            Instruction::JumpIf(_, x, y, _) => {
                if let IntChar::Char(c) = x {
                    regs.push(*c);
                }
                if let IntChar::Char(c) = y {
                    regs.push(*c);
                }
            }
            Instruction::Snd(x) | Instruction::Out(x) => {
                if let IntChar::Char(c) = x {
                    regs.push(*c);
                }
            }
            Instruction::Rcv(x) | Instruction::Toggle(x) => {
                regs.push(*x);
            }
            Instruction::Nop => {}
        }
        regs
    }
}

pub fn build_list(input: &str, build_instruction: fn(&str) -> Instruction) -> Vec<Instruction> {
    input.lines().map(build_instruction).collect()
}

pub fn execute_all(instructions: &[Instruction], regs: &mut Registers<i64>) {
    let mut ir = 0;
    while ir < instructions.len() {
        instructions[ir].execute(&mut ir, regs);
    }
}
