use crate::intchar::IntChar;
use crate::parsing::char;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Set(char, IntChar<i64>),
    Sub(char, IntChar<i64>),
    Mul(char, IntChar<i64>),
    JumpNotZero(IntChar<i64>, IntChar<i64>),

    Snd(IntChar<i64>),
    Add(char, IntChar<i64>),
    Mod(char, IntChar<i64>),
    Rcv(char), // In theory could be IntChar but input and part 2 limits to a register.
    JumpGreaterThanZero(IntChar<i64>, IntChar<i64>),

    Nop,
}

impl Instruction {
    pub fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "set" => Self::Set(char(parts[1]), IntChar::new(parts[2])),
            "sub" => Self::Sub(char(parts[1]), IntChar::new(parts[2])),
            "mul" => Self::Mul(char(parts[1]), IntChar::new(parts[2])),
            "jnz" => Self::JumpNotZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "nop" => Self::Nop,

            "snd" => Self::Snd(IntChar::new(parts[1])),
            "add" => Self::Add(char(parts[1]), IntChar::new(parts[2])),
            "mod" => Self::Mod(char(parts[1]), IntChar::new(parts[2])),
            "rcv" => Self::Rcv(char(parts[1])),
            "jgz" => Self::JumpGreaterThanZero(IntChar::new(parts[1]), IntChar::new(parts[2])),

            _ => panic!("Unknown instruction"),
        }
    }
}
