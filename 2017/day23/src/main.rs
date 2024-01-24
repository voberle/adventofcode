use std::io::{self, Read};

use fxhash::FxHashMap;

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[derive(Debug)]
struct Registers<T> {
    regs: FxHashMap<char, T>,
}

impl<T> Registers<T>
where
    T: std::str::FromStr,
    T: Copy,
    T: Default,
{
    fn new() -> Self {
        Self {
            regs: FxHashMap::default(),
        }
    }

    fn get(&self, r: char) -> T {
        self.regs.get(&r).copied().unwrap_or_default()
    }

    fn set(&mut self, r: char, val: T) {
        self.regs.insert(r, val);
    }

    fn get_ic(&self, x: IntChar<T>) -> T {
        match x {
            IntChar::Integer(val) => val,
            IntChar::Char(src) => self.get(src),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum IntChar<T>
where
    T: std::str::FromStr,
{
    Integer(T),
    Char(char),
}

impl<T> IntChar<T>
where
    T: std::str::FromStr,
{
    fn new(s: &str) -> Self {
        if let Ok(val) = s.parse() {
            IntChar::Integer(val)
        } else if s.len() == 1 {
            IntChar::Char(s.chars().next().unwrap())
        } else {
            panic!("Invalid string for building IntChar")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Set(char, IntChar<i64>),
    Sub(char, IntChar<i64>),
    Mul(char, IntChar<i64>),
    JumpNotZero(IntChar<i64>, IntChar<i64>),
    Nop,
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "set" => Self::Set(char(parts[1]), IntChar::new(parts[2])),
            "sub" => Self::Sub(char(parts[1]), IntChar::new(parts[2])),
            "mul" => Self::Mul(char(parts[1]), IntChar::new(parts[2])),
            "jnz" => Self::JumpNotZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "nop" => Self::Nop,
            _ => panic!("Unknown instruction"),
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

fn execute_common(ins: &Instruction, ir: &mut usize, regs: &mut Registers<i64>) {
    match ins {
        Instruction::Set(x, y) => {
            regs.set(*x, regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::Sub(x, y) => {
            regs.set(*x, regs.get(*x) - regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::JumpNotZero(x, y) => {
            if regs.get_ic(*x) != 0 {
                *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
            } else {
                *ir += 1;
            }
        }
        Instruction::Nop => *ir += 1,
        _ => panic!("Wrong use of this function"),
    }
}

fn execute(instructions: &[Instruction], ir: &mut usize, regs: &mut Registers<i64>) -> bool {
    let ins = &instructions[*ir];
    match ins {
        Instruction::Mul(x, y) => {
            regs.set(*x, regs.get(*x) * regs.get_ic(*y));
            *ir += 1;
            return true;
        }
        _ => execute_common(ins, ir, regs),
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

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", mul_count(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
