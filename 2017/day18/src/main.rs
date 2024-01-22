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
    Snd(IntChar<i64>),
    Set(char, IntChar<i64>),
    Add(char, IntChar<i64>),
    Mul(char, IntChar<i64>),
    Mod(char, IntChar<i64>),
    Recover(IntChar<i64>),
    JumpGreaterThanZero(IntChar<i64>, IntChar<i64>),
    Nop,
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "snd" => Self::Snd(IntChar::new(parts[1])),
            "set" => Self::Set(char(parts[1]), IntChar::new(parts[2])),
            "add" => Self::Add(char(parts[1]), IntChar::new(parts[2])),
            "mul" => Self::Mul(char(parts[1]), IntChar::new(parts[2])),
            "mod" => Self::Mod(char(parts[1]), IntChar::new(parts[2])),
            "rcv" => Self::Recover(IntChar::new(parts[1])),
            "jgz" => Self::JumpGreaterThanZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "nop" => Self::Nop,
            _ => panic!("Unknown instruction"),
        }
    }
}

// Executes the instruction specified by ins, modifying the registers if needed.
fn execute(
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
        Instruction::Set(x, y) => {
            regs.set(*x, regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::Add(x, y) => {
            regs.set(*x, regs.get(*x) + regs.get_ic(*y));
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
        Instruction::Recover(x) => {
            if regs.get_ic(*x) != 0 {
                // recovers the frequency of the last sound played
                return Some(*last_sound_played);
            }
            *ir += 1;
        }
        Instruction::JumpGreaterThanZero(x, y) => {
            if regs.get_ic(*x) > 0 {
                *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
            } else {
                *ir += 1;
            }
        }
        Instruction::Nop => *ir += 1,
    }
    None
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

fn recovered_frequency_value(instructions: &[Instruction]) -> i64 {
    let mut regs = Registers::new();
    let mut last_sound_played = 0;
    let mut ir = 0;
    while ir < instructions.len() {
        // println!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        if let Some(recv_snd) = execute(instructions, &mut ir, &mut regs, &mut last_sound_played) {
            return recv_snd;
        }
    }
    panic!("Didn't find a recovered sound")
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", recovered_frequency_value(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(recovered_frequency_value(&build(INPUT_TEST)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
