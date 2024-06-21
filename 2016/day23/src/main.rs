use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
};

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

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

    fn get(&self, x: IntChar<i32>) -> i32 {
        match x {
            IntChar::Integer(val) => val,
            IntChar::Char(src) => self[src],
        }
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
    // Same instructions as Day 12
    Copy(IntChar<i32>, char),
    Increase(char),
    Decrease(char),
    JumpIfNotZero(IntChar<i32>, IntChar<i32>),
    // New instruction
    Toggle(char),
    Nop,
    // Optimization instruction for part 2
    Mult(IntChar<i32>, IntChar<i32>, char),
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "cpy" => Self::Copy(IntChar::new(parts[1]), char(parts[2])),
            "inc" => Self::Increase(char(parts[1])),
            "dec" => Self::Decrease(char(parts[1])),
            "jnz" => Self::JumpIfNotZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "tgl" => Self::Toggle(char(parts[1])),
            "mul" => Self::Mult(
                IntChar::new(parts[1]),
                IntChar::new(parts[2]),
                char(parts[3]),
            ),
            "nop" => Self::Nop,
            _ => panic!("Unknown instruction"),
        }
    }
}

// Executes the instruction specified by ins, modifying the registers if needed, and returns the next instruction ID.
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
fn execute(instructions: &mut [Instruction], ir: usize, regs: &mut Registers) -> usize {
    let ins = &instructions[ir];
    match ins {
        Instruction::Copy(x, r) => {
            regs[*r] = regs.get(*x);
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
        Instruction::JumpIfNotZero(v, offset) => {
            if regs.get(*v) != 0 {
                (ir as i32 + regs.get(*offset)) as usize
            } else {
                ir + 1
            }
        }
        Instruction::Toggle(offset) => {
            let ir_to_toggle = (ir as i32 + regs[*offset]) as usize;
            if ir_to_toggle >= instructions.len() {
                return ir + 1;
            }
            instructions[ir_to_toggle] = match &instructions[ir_to_toggle] {
                Instruction::Copy(x, r) => Instruction::JumpIfNotZero(*x, IntChar::Char(*r)),
                Instruction::Increase(r) => Instruction::Decrease(*r),
                Instruction::Decrease(r) => Instruction::Increase(*r),
                Instruction::JumpIfNotZero(v, o) => match o {
                    IntChar::Integer(_) => Instruction::Nop,
                    IntChar::Char(r) => Instruction::Copy(*v, *r),
                },
                Instruction::Toggle(offset) => Instruction::Increase(*offset),
                Instruction::Mult(a, b, r) => Instruction::Mult(*a, *b, *r),
                Instruction::Nop => Instruction::Nop,
            };
            ir + 1
        }
        Instruction::Mult(a, b, r) => {
            regs[*r] = regs.get(*a) * regs.get(*b);
            ir + 1
        }
        Instruction::Nop => ir + 1,
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

// This set of instructions multiplies b by d, storing the result into a.
const MULT_INSTRUCTIONS: [Instruction; 6] = [
    Instruction::Copy(IntChar::Char('b'), 'c'),
    Instruction::Increase('a'),
    Instruction::Decrease('c'),
    Instruction::JumpIfNotZero(IntChar::Char('c'), IntChar::Integer(-2)),
    Instruction::Decrease('d'),
    Instruction::JumpIfNotZero(IntChar::Char('d'), IntChar::Integer(-5)),
];

const MULT_INSTRUCTIONS_OPTIMIZED: [Instruction; 6] = [
    Instruction::Mult(IntChar::Char('b'), IntChar::Char('d'), 'a'),
    Instruction::Nop,
    Instruction::Nop,
    Instruction::Nop,
    Instruction::Nop,
    Instruction::Nop,
];

fn find_subset(instructions: &[Instruction], needle: &[Instruction]) -> Option<usize> {
    for (position, window) in instructions.windows(needle.len()).enumerate() {
        if window == needle {
            return Some(position);
        }
    }
    None
}

// Finds and optimizes
fn optimizer(instructions: &mut Vec<Instruction>) {
    assert_eq!(MULT_INSTRUCTIONS.len(), MULT_INSTRUCTIONS_OPTIMIZED.len());
    if let Some(subset_start) = find_subset(instructions, &MULT_INSTRUCTIONS) {
        instructions.splice(
            subset_start..subset_start + MULT_INSTRUCTIONS.len(),
            MULT_INSTRUCTIONS_OPTIMIZED,
        );
    }
}

fn execute_all(instructions: &[Instruction], mut regs: Registers) -> Registers {
    let mut modifiable_ins: Vec<Instruction> = instructions.to_vec();
    optimizer(&mut modifiable_ins);

    let mut ir = 0;
    while ir < modifiable_ins.len() {
        // println!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        ir = execute(&mut modifiable_ins, ir, &mut regs);
    }
    regs
}

fn value_sent_to_safe(instructions: &[Instruction]) -> i32 {
    execute_all(instructions, Registers::new(7, 0, 0, 0))['a']
}

fn actual_value_sent_to_safe(instructions: &[Instruction]) -> i32 {
    execute_all(instructions, Registers::new(12, 0, 0, 0))['a']
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", value_sent_to_safe(&instructions));
    println!("Part 2: {}", actual_value_sent_to_safe(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(value_sent_to_safe(&build(INPUT_TEST)), 3);
    }

    #[test]
    fn test_day12_part1() {
        let instructions = build(include_str!("../../day12/resources/input_test_1"));
        assert_eq!(
            execute_all(&instructions, Registers::new(0, 0, 0, 0))['a'],
            42
        );
    }
}
