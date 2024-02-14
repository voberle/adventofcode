use std::{
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
            "gtit" => regs[self.c] = u32::from(self.a > regs[self.b]),
            "gtri" => regs[self.c] = u32::from(regs[self.a] > self.b),
            "gtrr" => regs[self.c] = u32::from(regs[self.a] > regs[self.b]),
            "eqit" => regs[self.c] = u32::from(self.a == regs[self.b]),
            "eqri" => regs[self.c] = u32::from(regs[self.a] == self.b),
            "eqrr" => regs[self.c] = u32::from(regs[self.a] == regs[self.b]),
            _ => panic!("Invalid opcode"),
        }
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

fn exec(ip_binding: u32, instructions: &[Instruction], regs: &mut Registers) {
    let mut ip: u32 = 0;
    while ip < instructions.len() as u32 {
        regs[ip_binding] = ip;

        let ins = &instructions[ip as usize];
        ins.exec(regs);

        ip = regs[ip_binding];
        ip += 1;
    }
}

fn process1_reg0_val(ip_binding: u32, instructions: &[Instruction]) -> u32 {
    let mut regs = Registers::new();
    exec(ip_binding, instructions, &mut regs);
    regs[0]
}

fn part2(ip_binding: u32, instructions: &[Instruction]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (ip_binding, instructions) = build(&input);

    println!("Part 1: {}", process1_reg0_val(ip_binding, &instructions));
    println!("Part 2: {}", part2(ip_binding, &instructions));
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

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
