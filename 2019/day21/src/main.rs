#![allow(non_snake_case)]
use std::{
    fmt,
    io::{self, Read},
};

use intcode::IntcodeComputer;

#[derive(Clone, Copy)]
enum Reg {
    T,
    J,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl Reg {
    fn is_writable(&self) -> bool {
        matches!(self, Reg::T | Reg::J)
    }

    fn index(self) -> usize {
        match self {
            Reg::T => 0,
            Reg::J => 1,
            Reg::A => 2,
            Reg::B => 3,
            Reg::C => 4,
            Reg::D => 5,
            Reg::E => 6,
            Reg::F => 7,
            Reg::G => 8,
            Reg::H => 9,
            Reg::I => 10,
        }
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
            Reg::E => 'E',
            Reg::F => 'F',
            Reg::G => 'G',
            Reg::H => 'H',
            Reg::I => 'I',
        };
        write!(f, "{}", c)
    }
}

enum Instruction {
    And(Reg, Reg),
    Or(Reg, Reg),
    Not(Reg, Reg),
}

impl Instruction {
    fn exec(&self, reg: &mut [bool]) {
        match self {
            Instruction::And(x, y) => reg[y.index()] = reg[x.index()] && reg[y.index()],
            Instruction::Or(x, y) => reg[y.index()] = reg[x.index()] || reg[y.index()],
            Instruction::Not(x, y) => reg[y.index()] = !reg[x.index()],
        }
    }
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

fn survey_hull(computer: &IntcodeComputer, instructions: &[Instruction], run: bool) -> i64 {
    let mut computer = computer.clone();
    write_instruction(&mut computer, instructions);

    write_string(&mut computer, if run { "RUN" } else { "WALK" });

    computer.exec();

    let output = ComputerOutput::read(&mut computer);
    match output {
        ComputerOutput::LastMoments(_) => output.print(),
        ComputerOutput::HullDamage(damage) => return damage,
    }
    panic!("Didn't make it across");
}

fn get_walk_instructions() -> Vec<Instruction> {
    use Reg::{A, C, D, J, T};
    // Supports jumping over following:
    // #####.###########
    // #####...#########
    // #####.#..########
    vec![
        NOT(A, J), // 1-away is empty
        NOT(C, T),
        OR(T, J),  // or 3-away is empty
        AND(D, J), // and 4-away is ground
    ]
}

fn survey_hull_part1(computer: &IntcodeComputer) -> i64 {
    let instructions = get_walk_instructions();
    survey_hull(computer, &instructions, false)
}

fn get_run_instructions() -> Vec<Instruction> {
    use Reg::{A, B, C, D, E, F, G, H, I, J, T};
    // Just jump above:
    // #####.###########
    // #####.#..########
    // #####...#########
    // #####...##.##.###
    // #####..##########
    vec![
        NOT(A, J), // 1-away is empty
        NOT(C, T),
        OR(T, J),  // or 3-away is empty
        AND(D, J), // and 4-away is ground
        // TO BE COMPLETED
    ]
}

fn survey_hull_part2(computer: &IntcodeComputer) -> i64 {
    let instructions = get_run_instructions();
    survey_hull(computer, &instructions, true)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", survey_hull_part1(&computer));
    println!("Part 2: {}", survey_hull_part2(&computer));
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn run_springscript(instructions: &[Instruction], regs: &[bool]) -> bool {
        let mut regs = regs.to_vec();
        for ins in instructions {
            ins.exec(&mut regs);
        }
        regs[Reg::J.index()]
    }

    fn run_converted(_instructions: &[Instruction], regs: &[bool]) -> bool {
        let (a, _b, c, d) = (regs[2], regs[3], regs[4], regs[5]);
        (!a || !c) && d
    }

    fn regs(pos_regs: &[bool]) -> Vec<bool> {
        let mut regs = vec![false, false]; // J and T
        regs.extend(pos_regs);
        regs
    }

    #[test]
    fn test_walk_instructions() {
        let instructions = get_walk_instructions();
        let exec = run_springscript;
        assert!(!exec(&instructions, &regs(&[false, false, false, false])));
        assert!(exec(&instructions, &regs(&[false, false, false, true])));
        assert!(!exec(&instructions, &regs(&[false, false, true, false])));
        assert!(exec(&instructions, &regs(&[false, false, true, true])));
        assert!(!exec(&instructions, &regs(&[false, true, false, false])));
        assert!(exec(&instructions, &regs(&[false, true, false, true])));
        assert!(!exec(&instructions, &regs(&[false, true, true, false])));
        assert!(exec(&instructions, &regs(&[false, true, true, true])));

        assert!(!exec(&instructions, &regs(&[true, false, false, false])));
        assert!(exec(&instructions, &regs(&[true, false, false, true])));
        assert!(!exec(&instructions, &regs(&[true, false, true, false])));
        assert!(!exec(&instructions, &regs(&[true, false, true, true])));
        assert!(!exec(&instructions, &regs(&[true, true, false, false])));
        assert!(exec(&instructions, &regs(&[true, true, false, true])));
        assert!(!exec(&instructions, &regs(&[true, true, true, false])));
        assert!(!exec(&instructions, &regs(&[true, true, true, true])));
    }

    #[test]
    fn test_walk_instructions_automatic() {
        let instructions = get_walk_instructions();

        let combi: Vec<_> = itertools::repeat_n([false, true].iter(), 4)
            .multi_cartesian_product()
            .collect();
        for p in combi {
            let r: Vec<bool> = p.iter().cloned().cloned().collect(); // ugly..
            let springscript_result = run_springscript(&instructions, &regs(&r));
            let converted_result = run_converted(&instructions, &regs(&r));
            assert_eq!(springscript_result, converted_result)
        }
    }
}
