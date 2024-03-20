#![allow(non_snake_case)]
use std::{
    fmt,
    io::{self, Read},
};

use intcode::IntcodeComputer;

enum Reg {
    T,
    J,
    A,
    B,
    C,
    D,
}

impl Reg {
    fn is_writable(&self) -> bool {
        matches!(self, Reg::T | Reg::J)
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Reg::T => 'T',
            Reg::J => 'J',
            Reg::A => 'A',
            Reg::B => 'B',
            Reg::C => 'C',
            Reg::D => 'D',
        };
        write!(f, "{}", c)
    }
}

enum Instruction {
    And(Reg, Reg),
    Or(Reg, Reg),
    Not(Reg, Reg),
}

fn AND(x: Reg, y: Reg) -> Instruction {
    assert!(y.is_writable());
    Instruction::And(x, y)
}

fn OR(x: Reg, y: Reg) -> Instruction {
    assert!(y.is_writable());
    Instruction::Or(x, y)
}

fn NOT(x: Reg, y: Reg) -> Instruction {
    assert!(y.is_writable());
    Instruction::Not(x, y)
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::And(x, y) => write!(f, "AND {} {}", x, y),
            Instruction::Or(x, y) => write!(f, "OR {} {}", x, y),
            Instruction::Not(x, y) => write!(f, "NOT {} {}", x, y),
        }
    }
}

const NEWLINE: i64 = 10;

fn write_string(computer: &mut IntcodeComputer, s: &str) {
    s.chars().map(|c| c as i64).for_each(|i| {
        computer.io.add_input(i);
    });
    computer.io.add_input(NEWLINE);
}

fn write_instruction(computer: &mut IntcodeComputer, instructions: &[Instruction]) {
    for ins in instructions {
        write_string(computer, &ins.to_string());
    }
}

enum ComputerOutput {
    LastMoments(Vec<char>),
    HullDamage(i64),
}

impl ComputerOutput {
    fn read(computer: &mut IntcodeComputer) -> Self {
        let mut output: Vec<char> = Vec::new();
        while let Some(i) = computer.io.get_output() {
            if i > 255 {
                // Output is outside ASCII range, so it's hull damage
                return Self::HullDamage(i);
            }
            output.push(char::from_u32(u32::try_from(i).unwrap()).unwrap());
        }
        Self::LastMoments(output)
    }

    #[allow(dead_code)]
    fn print(&self) {
        match self {
            ComputerOutput::LastMoments(output) => {
                for c in output {
                    print!("{}", c);
                }
            }
            ComputerOutput::HullDamage(d) => {
                println!("Hull damage {}", d);
            }
        }
    }
}

fn survey_hull(computer: &IntcodeComputer, instructions: &[Instruction]) -> i64 {
    let mut computer = computer.clone();
    write_instruction(&mut computer, instructions);

    write_string(&mut computer, "WALK");

    computer.exec();

    let output = ComputerOutput::read(&mut computer);
    match output {
        ComputerOutput::LastMoments(_) => output.print(),
        ComputerOutput::HullDamage(damage) => return damage,
    }
    0
}

fn survey_hull_part1(computer: &IntcodeComputer) -> i64 {
    use Reg::{A, B, C, D, J, T};
    let instructions = vec![
        NOT(A, J), // 1-away is empty
        NOT(C, T),
        OR(T, J), // or 3-away is empty
        AND(D, J), // and 4-away is ground
    ];
    let damage = survey_hull(computer, &instructions);
    assert!(damage > 0, "Didn't make it across");
    damage
}

fn survey_hull_part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", survey_hull_part1(&computer));
    println!("Part 2: {}", survey_hull_part2(&computer));
}
