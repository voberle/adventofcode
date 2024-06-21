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
    Nop,
    // Instruction for day 23
    Out(IntChar<i32>),
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "cpy" => Self::Copy(IntChar::new(parts[1]), char(parts[2])),
            "inc" => Self::Increase(char(parts[1])),
            "dec" => Self::Decrease(char(parts[1])),
            "jnz" => Self::JumpIfNotZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "nop" => Self::Nop,
            "out" => Self::Out(IntChar::new(parts[1])),
            _ => panic!("Unknown instruction"),
        }
    }
}

// Executes the instruction specified by ins, modifying the registers if needed.
// Returns the output of "out" command if it was executed.
#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
fn execute(instructions: &[Instruction], ir: &mut usize, regs: &mut Registers) -> Option<i32> {
    let ins = &instructions[*ir];
    if let Instruction::Out(x) = ins {
        *ir += 1;
        return Some(regs.get(*x));
    }

    match ins {
        Instruction::Copy(x, r) => {
            regs[*r] = regs.get(*x);
            *ir += 1;
        }
        Instruction::Increase(r) => {
            regs[*r] += 1;
            *ir += 1;
        }
        Instruction::Decrease(r) => {
            regs[*r] -= 1;
            *ir += 1;
        }
        Instruction::JumpIfNotZero(v, offset) => {
            if regs.get(*v) != 0 {
                *ir = (*ir as i32 + regs.get(*offset)) as usize;
            } else {
                *ir += 1;
            }
        }
        Instruction::Nop => {
            *ir += 1;
        }
        Instruction::Out(_) => panic!("Can't happen, handled before"),
    }
    None
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

fn is_periodic_clock_signal(clock_signal: &[i32]) -> bool {
    let mut periodic = true;
    let mut iter = clock_signal.iter().peekable();
    while let Some(s) = iter.next() {
        if let Some(n) = iter.peek() {
            periodic &= *s != **n;
        }
        if !periodic {
            break;
        }
    }
    periodic
}

fn execute_all<const MAX_CLOCK_SIGNALS_TO_CHECK: usize>(
    instructions: &[Instruction],
    mut regs: Registers,
) -> bool {
    let mut ir = 0;
    let mut clock_signal: Vec<i32> = Vec::new();
    while ir < instructions.len()
        && is_periodic_clock_signal(&clock_signal)
        && clock_signal.len() < MAX_CLOCK_SIGNALS_TO_CHECK
    {
        let out = execute(instructions, &mut ir, &mut regs);
        if let Some(s) = out {
            assert!(s == 0 || s == 1);
            clock_signal.push(s);
        }
    }
    is_periodic_clock_signal(&clock_signal)
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut a = 0;
    while a < 10_000 {
        if execute_all::<40>(instructions, Registers::new(a, 0, 0, 0)) {
            break;
        }
        a += 1;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", part1(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_periodic_clock_signal() {
        assert!(is_periodic_clock_signal(&[0, 1, 0, 1, 0]));
        assert!(is_periodic_clock_signal(&[1, 0, 1, 0]));
        assert!(is_periodic_clock_signal(&[0, 1, 0, 1, 0, 1]));

        assert!(!is_periodic_clock_signal(&[0, 1, 1, 0, 0]));
        assert!(!is_periodic_clock_signal(&[0, 0]));
    }
}
