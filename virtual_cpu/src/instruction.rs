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
    JumpNotZero(IntChar<i64>, IntChar<i64>),
    JumpGreaterThanZero(IntChar<i64>, IntChar<i64>),
    Nop,

    Snd(IntChar<i64>),
    Rcv(char), // In theory could be IntChar but input and part 2 limits to a register.
}

impl Instruction {
    pub fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "set" => Self::Set(char(parts[1]), IntChar::new(parts[2])),
            "add" => Self::Add(char(parts[1]), IntChar::new(parts[2])),
            "sub" => Self::Sub(char(parts[1]), IntChar::new(parts[2])),
            "mul" => Self::Mul(char(parts[1]), IntChar::new(parts[2])),
            "mod" => Self::Mod(char(parts[1]), IntChar::new(parts[2])),
            "jnz" => Self::JumpNotZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "jgz" => Self::JumpGreaterThanZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "nop" => Self::Nop,

            "snd" => Self::Snd(IntChar::new(parts[1])),
            "rcv" => Self::Rcv(char(parts[1])),

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
            Instruction::Nop => *ir += 1,
            _ => panic!("Unsupported instruction in Instruction::execute()"),
        }
    }
}
