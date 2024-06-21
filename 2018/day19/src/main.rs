use std::{
    fmt,
    io::{self, Read},
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Clone)]
struct Registers(Vec<u32>);

impl Registers {
    const REGISTERS_COUNT: u32 = 6;

    fn new() -> Self {
        Self(vec![0; Self::REGISTERS_COUNT as usize])
    }
}

impl Index<u32> for Registers {
    type Output = u32;
    fn index(&self, reg: u32) -> &Self::Output {
        assert!((0..Self::REGISTERS_COUNT).contains(&reg));
        &self.0[reg as usize]
    }
}

impl IndexMut<u32> for Registers {
    fn index_mut(&mut self, reg: u32) -> &mut Self::Output {
        assert!((0..Self::REGISTERS_COUNT).contains(&reg));
        &mut self.0[reg as usize]
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: String,
    a: u32,
    b: u32,
    c: u32,
}

impl Instruction {
    fn build(s: &str) -> Self {
        let p: Vec<_> = s.split(' ').collect();
        Self {
            opcode: p[0].to_string(),
            a: p[1].parse().unwrap(),
            b: p[2].parse().unwrap(),
            c: p[3].parse().unwrap(),
        }
    }

    fn exec(&self, regs: &mut Registers) {
        match self.opcode.as_str() {
            "addr" => regs[self.c] = regs[self.a] + regs[self.b],
            "addi" => regs[self.c] = regs[self.a] + self.b,
            "mulr" => regs[self.c] = regs[self.a] * regs[self.b],
            "muli" => regs[self.c] = regs[self.a] * self.b,
            "banr" => regs[self.c] = regs[self.a] & regs[self.b],
            "bani" => regs[self.c] = regs[self.a] & self.b,
            "borr" => regs[self.c] = regs[self.a] | regs[self.b],
            "bori" => regs[self.c] = regs[self.a] | self.b,
            "setr" => regs[self.c] = regs[self.a],
            "seti" => regs[self.c] = self.a,
            "gtir" => regs[self.c] = u32::from(self.a > regs[self.b]),
            "gtri" => regs[self.c] = u32::from(regs[self.a] > self.b),
            "gtrr" => regs[self.c] = u32::from(regs[self.a] > regs[self.b]),
            "eqit" => regs[self.c] = u32::from(self.a == regs[self.b]),
            "eqri" => regs[self.c] = u32::from(regs[self.a] == self.b),
            "eqrr" => regs[self.c] = u32::from(regs[self.a] == regs[self.b]),
            _ => panic!("Invalid opcode {}", self.opcode),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

fn build(input: &str) -> (u32, Vec<Instruction>) {
    let mut it = input.lines();
    let ip_binding = it
        .next()
        .unwrap()
        .strip_prefix("#ip ")
        .unwrap()
        .parse()
        .unwrap();
    let mut instructions = Vec::new();
    for line in it {
        instructions.push(Instruction::build(line));
    }
    (ip_binding, instructions)
}

fn exec(ip_binding: u32, instructions: &[Instruction], regs: &mut Registers, ip: &mut u32) {
    const DEBUG: bool = false;

    let ins = &instructions[*ip as usize];

    regs[ip_binding] = *ip;
    if DEBUG {
        print!("ip={} {} {:?}", ip, ins, regs);
    }

    ins.exec(regs);

    *ip = regs[ip_binding];
    *ip += 1;
    if DEBUG {
        println!(" {:?}", regs);
    }
}

fn exec_all(ip_binding: u32, instructions: &[Instruction], regs: &mut Registers) {
    let mut ip: u32 = 0;
    while ip < u32::try_from(instructions.len()).unwrap() {
        exec(ip_binding, instructions, regs, &mut ip);
    }
}

// Find the input to the main loop by running a sub-set of the instructions
fn calculate_r1(ip_binding: u32, instructions: &[Instruction], r0: u32) -> u32 {
    let mut regs = Registers::new();
    regs[0] = r0;

    // By not executing the very last instructions,
    // we only get the input calculation part of the instructions executed.
    exec_all(
        ip_binding,
        &instructions[0..instructions.len() - 1],
        &mut regs,
    );
    regs[1]
}

// Rust implementation of the instructions 1 to 16.
fn translated_loop(r1: u32) -> u32 {
    // This code is looking for all numbers between 1 and r1
    // which have another number <= r1 whose product is equal to r1.
    // This means all factors of r1.
    //
    // It returns the sum of all those numbers.
    let mut r0 = 0;

    let mut r3 = 1;
    while r3 <= r1 {
        let mut r2 = 1;
        while r2 <= r1 {
            if r3 * r2 == r1 {
                r0 += r3;
            }
            r2 += 1;
        }
        r3 += 1;
    }
    r0
}

// Optimized version of above function.
fn sum_of_factors(n: u32) -> u32 {
    (1..=n).filter(|i| n % i == 0).sum()
}

fn process1_reg0_val(ip_binding: u32, instructions: &[Instruction]) -> u32 {
    let mut regs = Registers::new();
    exec_all(ip_binding, instructions, &mut regs);
    let result = regs[0];

    // Checks optimized version is correct for part 1.
    let r1 = calculate_r1(ip_binding, instructions, 0);
    assert_eq!(translated_loop(r1), result);
    assert_eq!(sum_of_factors(r1), result);

    result
}

fn process2_reg0_val(ip_binding: u32, instructions: &[Instruction]) -> u32 {
    let r1 = calculate_r1(ip_binding, instructions, 1);
    sum_of_factors(r1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (ip_binding, instructions) = build(&input);

    println!("Part 1: {}", process1_reg0_val(ip_binding, &instructions));
    println!("Part 2: {}", process2_reg0_val(ip_binding, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (ip_binding, instructions) = build(INPUT_TEST);
        assert_eq!(process1_reg0_val(ip_binding, &instructions), 6);
    }
}
