use crate::intchar::IntChar;
use crate::parsing::char;
use crate::registers::Registers;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Set(char, IntChar<i64>),
    Add(char, IntChar<i64>),
    Sub(char, IntChar<i64>),
    Mul(char, IntChar<i64>),
    Mod(char, IntChar<i64>),
    Div(char, IntChar<i64>),
    JumpNotZero(IntChar<i64>, IntChar<i64>),
    JumpGreaterThanZero(IntChar<i64>, IntChar<i64>),
    Nop,

    Jump(IntChar<i64>),
    JumpIfEven(IntChar<i64>, IntChar<i64>),
    JumpIfOne(IntChar<i64>, IntChar<i64>), // jump if one", not odd

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
        Self::build_from_parts(&parts)
    }

    pub fn build_from_parts(parts: &[&str]) -> Self {
        match *parts.first().unwrap() {
            "set" => Self::Set(char(parts[1]), IntChar::new(parts[2])),
            "cpy" => Self::Set(char(parts[2]), IntChar::new(parts[1])), // params inversed vs set
            "add" => Self::Add(char(parts[1]), IntChar::new(parts[2])),
            "inc" => Self::Add(char(parts[1]), IntChar::from_int(1)),
            "sub" => Self::Sub(char(parts[1]), IntChar::new(parts[2])),
            "dec" => Self::Sub(char(parts[1]), IntChar::from_int(1)),
            "mul" => Self::Mul(char(parts[1]), IntChar::new(parts[2])),
            "tpl" => Self::Mul(char(parts[1]), IntChar::from_int(3)),
            "mod" => Self::Mod(char(parts[1]), IntChar::new(parts[2])),
            "div" => Self::Div(char(parts[1]), IntChar::new(parts[2])),
            "hlf" => Self::Div(char(parts[1]), IntChar::from_int(2)),
            "jnz" => Self::JumpNotZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "jgz" => Self::JumpGreaterThanZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "jmp" => Self::Jump(IntChar::new(parts[1])),
            "jie" => Self::JumpIfEven(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "jio" => Self::JumpIfOne(IntChar::new(parts[1]), IntChar::new(parts[2])),

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

            Instruction::JumpNotZero(x, y) => {
                if regs.get_ic(*x) != 0 {
                    *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
                } else {
                    *ir += 1;
                }
            }
            Instruction::JumpGreaterThanZero(x, y) => {
                if regs.get_ic(*x) > 0 {
                    *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
                } else {
                    *ir += 1;
                }
            }
            Instruction::Jump(x) => {
                *ir = (*ir as i64 + regs.get_ic(*x)) as usize;
            }
            Instruction::JumpIfEven(x, y) => {
                if regs.get_ic(*x) % 2 == 0 {
                    *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
                } else {
                    *ir += 1;
                }
            }
            Instruction::JumpIfOne(x, y) => {
                // jump if one", not odd
                if regs.get_ic(*x) == 1 {
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
}

pub fn execute_all(instructions: &[Instruction], regs: &mut Registers<i64>) {
    let mut ir = 0;
    while ir < instructions.len() {
        instructions[ir].execute(&mut ir, regs);
    }
}
