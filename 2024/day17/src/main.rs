#![allow(non_snake_case)]
use std::{
    fmt,
    io::{self, Read},
};

use itertools::Itertools;

fn pow(exp: u64) -> u64 {
    2u64.pow(u32::try_from(exp).unwrap())
}

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
    A: u64,
    B: u64,
    C: u64,
}

impl Registers {
    fn new(A: u64, B: u64, C: u64) -> Self {
        Self { A, B, C }
    }
}

#[derive(Debug, Clone, Copy)]
enum ComboOp {
    LiteralValue(u64), // literal values 0 through 3
    RegisterA,
    RegisterB,
    RegisterC,
    Reserved,
}

impl ComboOp {
    fn new(combo: u8) -> Self {
        match combo {
            0..=3 => ComboOp::LiteralValue(u64::from(combo)),
            4 => ComboOp::RegisterA,
            5 => ComboOp::RegisterB,
            6 => ComboOp::RegisterC,
            7 => ComboOp::Reserved,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn value(self, registers: &Registers) -> u64 {
        match self {
            ComboOp::LiteralValue(val) => val,
            ComboOp::RegisterA => registers.A,
            ComboOp::RegisterB => registers.B,
            ComboOp::RegisterC => registers.C,
            ComboOp::Reserved => panic!("Reserved operand"),
        }
    }
}

impl fmt::Display for ComboOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ComboOp::LiteralValue(v) => v,
                ComboOp::RegisterA => 4,
                ComboOp::RegisterB => 5,
                ComboOp::RegisterC => 6,
                ComboOp::Reserved => 7,
            }
        )
    }
}

#[derive(Debug)]
enum Instruction {
    Adv(ComboOp),
    Bxl(u64),
    Bst(ComboOp),
    Jnz(u64),
    Bxc(u64),
    Out(ComboOp),
    Bdv(ComboOp),
    Cdv(ComboOp),
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Instruction::Adv(ComboOp::new(operand)),
            1 => Instruction::Bxl(u64::from(operand)),
            2 => Instruction::Bst(ComboOp::new(operand)),
            3 => Instruction::Jnz(u64::from(operand)),
            4 => Instruction::Bxc(u64::from(operand)), // operand is ignored
            5 => Instruction::Out(ComboOp::new(operand)),
            6 => Instruction::Bdv(ComboOp::new(operand)),
            7 => Instruction::Cdv(ComboOp::new(operand)),
            _ => panic!("Invalid opcode"),
        }
    }

    fn exec(&self, regs: &mut Registers, ip: &mut usize, output: &mut Vec<u64>) {
        match self {
            Instruction::Adv(combo) => regs.A /= pow(combo.value(regs)),
            Instruction::Bxl(literal) => regs.B ^= literal,
            Instruction::Bst(combo) => regs.B = combo.value(regs) % 8,
            Instruction::Jnz(literal) => {
                if regs.A != 0 {
                    // We need to divided by two, because in our program vector each instruction + params
                    // takes one place, while in the source it takes two.
                    *ip = usize::try_from(*literal).unwrap() / 2;
                    return;
                }
            }
            Instruction::Bxc(_) => regs.B ^= regs.C,
            Instruction::Out(combo) => output.push(combo.value(regs) % 8),
            Instruction::Bdv(combo) => regs.B = regs.A / pow(combo.value(regs)),
            Instruction::Cdv(combo) => regs.C = regs.A / pow(combo.value(regs)),
        }
        *ip += 1;
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Instruction::Adv(combo_op) => write!(f, "0,{combo_op}"),
            Instruction::Bxl(val) => write!(f, "1,{val}"),
            Instruction::Bst(combo_op) => write!(f, "2,{combo_op}"),
            Instruction::Jnz(val) => write!(f, "3,{val}"),
            Instruction::Bxc(val) => write!(f, "4,{val}"),
            Instruction::Out(combo_op) => write!(f, "5,{combo_op}"),
            Instruction::Bdv(combo_op) => write!(f, "6,{combo_op}"),
            Instruction::Cdv(combo_op) => write!(f, "7,{combo_op}"),
        }
    }
}

fn program_to_string(program: &[Instruction]) -> String {
    program
        .iter()
        .map(std::string::ToString::to_string)
        .join(",")
}

fn run_program(registers: &mut Registers, program: &[Instruction]) -> Vec<u64> {
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

// Brute force version.
#[allow(dead_code)]
fn find_reg_a_val_for_self_replicate(program: &[Instruction]) -> u64 {
    let program_as_string = program_to_string(program);

    for reg_a in 0.. {
        let registers = Registers::new(reg_a, 0, 0);
        let output = final_output(&registers, program);
        if output == program_as_string {
            return reg_a;
        }
    }
    panic!("Not found")
}

#[allow(unused_assignments)]
fn converted_prog(reg_a: u64) -> String {
    let mut a = reg_a;
    let mut b = 0;
    let mut c = 0;
    let mut output = Vec::new();

    while a != 0 {
        // println!("a = {a}");
        // Bst(A)  B = A % 8
        b = a % 8;
        // Bxl(3)  B = B ^ 3
        b ^= 3;
        // Cdv(B)  C = A / 2.pow(B)
        c = a / pow(b);
        // Bxc(1)  B = B ^ C
        b ^= c;
        // Bxl(3)  B = B ^ 3
        b ^= 3;
        // Adv(3)  A = A / 2.pow(3)
        a /= 8;
        // Out(B)  Outputs B % 8
        // println!("b = {},  a = {a}", b % 8);
        output.push(b % 8);
    } // Jnz(0)  If A != 0, jumps to beginning

    output.into_iter().join(",")
}

// Still brute force, if a bit faster.
#[allow(dead_code)]
fn find_reg_a_val_with_converted(program: &[Instruction]) -> u64 {
    let program_as_string = program_to_string(program);

    for reg_a in 0.. {
        let output = converted_prog(reg_a);
        if output == program_as_string {
            return reg_a;
        }
    }
    panic!("Not found")
}

// Finds the first value of A that gives the target B.
#[allow(unused_assignments)]
fn find_a(target_b: u64, from: u64) -> u64 {
    for reg_a in from.. {
        let mut a = reg_a;
        let mut b = 0;
        let mut c = 0;

        b = a % 8;
        b ^= 3;
        c = a / pow(b);
        b ^= c;
        b ^= 3;
        a /= 8;

        if b % 8 == target_b {
            return reg_a;
        }
    }
    panic!("bug");
}

fn find_reg_a_val_smart(program: &[Instruction]) -> u64 {
    let program_as_string = program_to_string(program);

    let mut reg_a = 0;
    let mut from = 1;
    for b in program_as_string
        .split(',')
        .map(|o| o.parse::<u64>().unwrap())
        .rev()
    {
        reg_a = find_a(b, from);
        from = reg_a << 3;
    }

    // For some reason the value of A we get here is not yet right.
    // The output has its second value off by 1.
    // We also know that the value of A that we got is too low.
    // However we must be close: So now we can manually try all remaining options.
    for corrected_reg_a in reg_a + 1.. {
        let output = converted_prog(corrected_reg_a);
        if output == program_as_string {
            reg_a = corrected_reg_a;
            break;
        }
    }

    // Verification with converted prog
    let output_with_a = converted_prog(reg_a);
    assert_eq!(output_with_a, program_as_string);
    // Verification with real prog
    let regs = Registers::new(reg_a, 0, 0);
    let output_with_a_bis = final_output(&regs, program);
    assert_eq!(output_with_a_bis, program_as_string);

    reg_a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (registers, program) = build(&input);

    println!("Part 1: {}", final_output(&registers, &program));

    // To check that the converted program is correct:
    // assert_eq!(final_output(&registers, &program), converted_prog(registers.A));

    // Brute force implementation by interpreting the program:
    // println!("Part 2: {}", find_reg_a_val_for_self_replicate(&program));
    // and by running the program in Rust:
    // println!("Part 2: {}", find_reg_a_val_with_converted(&program));

    println!("Part 2: {}", find_reg_a_val_smart(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

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
        let (registers, program) = build(INPUT_TEST_1);
        assert_eq!(final_output(&registers, &program), "4,6,3,5,6,3,5,2,1,0");
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_copies_itself() {
        let (mut registers, program) = build(INPUT_TEST_2);
        registers.A = 117440;
        let output = final_output(&registers, &program);
        assert_eq!(output, program_to_string(&program));
    }

    #[test]
    fn test_part2() {
        let (_, program) = build(INPUT_TEST_2);
        assert_eq!(find_reg_a_val_for_self_replicate(&program), 117440);
    }
}
