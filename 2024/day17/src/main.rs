#![allow(non_snake_case)]
use std::io::{self, Read};

use itertools::Itertools;

fn build_program(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|v| v.parse().unwrap())
        .chunks(2)
        .into_iter()
        .map(|p| {
            let (opcode, combo) = p.collect_tuple().unwrap();
            Instruction::new(opcode, combo)
        })
        .collect()
}

fn build(input: &str) -> (Registers, Vec<Instruction>) {
    let mut it = input.lines();
    let registers = Registers::new(
        it.next()
            .unwrap()
            .trim_start_matches("Register A: ")
            .parse()
            .unwrap(),
        it.next()
            .unwrap()
            .trim_start_matches("Register B: ")
            .parse()
            .unwrap(),
        it.next()
            .unwrap()
            .trim_start_matches("Register C: ")
            .parse()
            .unwrap(),
    );
    it.next();
    let program = build_program(it.next().unwrap().trim_start_matches("Program: "));
    (registers, program)
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    A: u32,
    B: u32,
    C: u32,
}

impl Registers {
    fn new(A: u32, B: u32, C: u32) -> Self {
        Self { A, B, C }
    }
}

#[derive(Debug, Clone, Copy)]
enum ComboOp {
    LiteralValue(u32), // literal values 0 through 3
    RegisterA,
    RegisterB,
    RegisterC,
    Reserved,
}

impl ComboOp {
    fn new(combo: u8) -> Self {
        match combo {
            0..=3 => ComboOp::LiteralValue(u32::from(combo)),
            4 => ComboOp::RegisterA,
            5 => ComboOp::RegisterB,
            6 => ComboOp::RegisterC,
            7 => ComboOp::Reserved,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn value(self, registers: &Registers) -> u32 {
        match self {
            ComboOp::LiteralValue(val) => val,
            ComboOp::RegisterA => registers.A,
            ComboOp::RegisterB => registers.B,
            ComboOp::RegisterC => registers.C,
            ComboOp::Reserved => panic!("Reserved operand"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv(ComboOp),
    Bxl(u32),
    Bst(ComboOp),
    Jnz(u32),
    Bxc,
    Out(ComboOp),
    Bdv(ComboOp),
    Cdv(ComboOp),
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Instruction::Adv(ComboOp::new(operand)),
            1 => Instruction::Bxl(u32::from(operand)),
            2 => Instruction::Bst(ComboOp::new(operand)),
            3 => Instruction::Jnz(u32::from(operand)),
            4 => Instruction::Bxc, // operand is ignored
            5 => Instruction::Out(ComboOp::new(operand)),
            6 => Instruction::Bdv(ComboOp::new(operand)),
            7 => Instruction::Cdv(ComboOp::new(operand)),
            _ => panic!("Invalid opcode"),
        }
    }

    fn exec(&self, regs: &mut Registers, ip: &mut usize, output: &mut Vec<u32>) {
        match self {
            Instruction::Adv(combo) => regs.A /= 2u32.pow(combo.value(regs)),
            Instruction::Bxl(literal) => regs.B ^= literal,
            Instruction::Bst(combo) => regs.B = combo.value(regs) % 8,
            Instruction::Jnz(literal) => {
                if regs.A != 0 {
                    // We need to divided by two, because in our program vector each instruction + params
                    // takes one place, while in the source it takes two.
                    *ip = *literal as usize / 2;
                    return;
                }
            }
            Instruction::Bxc => regs.B ^= regs.C,
            Instruction::Out(combo) => output.push(combo.value(regs) % 8),
            Instruction::Bdv(combo) => regs.B = regs.A / 2u32.pow(combo.value(regs)),
            Instruction::Cdv(combo) => regs.C = regs.A / 2u32.pow(combo.value(regs)),
        }
        *ip += 1;
    }
}

fn run_program(registers: &mut Registers, program: &[Instruction]) -> Vec<u32> {
    let mut ip: usize = 0;
    let mut output = Vec::new();

    while let Some(ins) = program.get(ip) {
        ins.exec(registers, &mut ip, &mut output);
    }
    output
}

fn final_output(registers: &Registers, program: &[Instruction]) -> String {
    let mut regs = *registers;
    let output = run_program(&mut regs, program);
    output.into_iter().join(",")
}

fn part2(registers: &Registers, program: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (registers, program) = build(&input);

    println!("Part 1: {}", final_output(&registers, &program));
    println!("Part 2: {}", part2(&registers, &program));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_inst_1() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut regs = Registers::new(0, 0, 9);
        assert_eq!(run_program(&mut regs, &build_program("2,6")), vec![]);
        assert_eq!(regs.B, 1);
    }

    #[test]
    fn test_inst_2() {
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut regs = Registers::new(10, 0, 0);
        assert_eq!(
            run_program(&mut regs, &build_program("5,0,5,1,5,4")),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn test_inst_3() {
        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut regs = Registers::new(2024, 0, 0);
        assert_eq!(
            run_program(&mut regs, &build_program("0,1,5,4,3,0")),
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(regs.A, 0);
    }

    #[test]
    fn test_inst_4() {
        // If register B contains 29, the program 1,7 would set register B to 26.
        let mut regs = Registers::new(0, 29, 0);
        assert_eq!(run_program(&mut regs, &build_program("1,7")), vec![]);
        assert_eq!(regs.B, 26);
    }

    #[test]
    fn test_inst_5() {
        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        let mut regs = Registers::new(0, 2024, 43690);
        assert_eq!(run_program(&mut regs, &build_program("4,0")), vec![]);
        assert_eq!(regs.B, 44354);
    }

    #[test]
    fn test_part1() {
        let (registers, program) = build(INPUT_TEST);
        assert_eq!(final_output(&registers, &program), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let (registers, program) = build(INPUT_TEST);
        assert_eq!(part2(&registers, &program), 0);
    }
}
