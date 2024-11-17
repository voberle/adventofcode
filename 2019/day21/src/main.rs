#![allow(non_snake_case)]
use std::{
    fmt,
    io::{self, Read},
};

use intcode::IntcodeComputer;

#[derive(Clone, Copy)]
#[allow(dead_code)]
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
    fn is_writable(self) -> bool {
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
        write!(f, "{c}")
    }
}

enum Instruction {
    And(Reg, Reg),
    Or(Reg, Reg),
    Not(Reg, Reg),
}

impl Instruction {
    #[allow(dead_code)]
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
            Instruction::And(x, y) => write!(f, "AND {x} {y}"),
            Instruction::Or(x, y) => write!(f, "OR {x} {y}"),
            Instruction::Not(x, y) => write!(f, "NOT {x} {y}"),
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
                    print!("{c}");
                }
            }
            ComputerOutput::HullDamage(d) => {
                println!("Hull damage {d}");
            }
        }
    }
}

fn survey_hull(computer: &IntcodeComputer, instructions: &[Instruction], run: bool) -> i64 {
    assert!(instructions.len() <= 15, "Too many instructions, max is 15");

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

    // (!a || !c) && d
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
    use Reg::{A, B, C, D, E, H, J, T};
    // Supports jumping over following:
    // #####.###########
    // #####.#..########
    // #####...#########
    // #####...##.##.###
    // #####..##########
    // #####.#.#..######
    // #####.##..#.#####
    // #####..###...####
    // #####.###..#.####

    // (!b && d && !e) || (!a && d) || (!b && c && d) || (!c && h && d)
    vec![
        // !b && d && !e
        NOT(B, T), // T = NOT B
        NOT(E, J), // J = NOT E
        AND(T, J), // J = NOT B AND NOT E
        AND(D, J), // J = NOT B AND NOT E AND D
        // !a && d
        NOT(A, T), // T = NOT A
        AND(D, T), // T = NOT A AND D
        OR(T, J),  // J = (NOT B AND NOT E AND D) OR (NOT A AND D)
        // !b && c && d
        NOT(B, T), // T = NOT B
        AND(C, T), // T = NOT B AND C
        AND(D, T), // T = NOT B AND C AND D
        OR(T, J),  // J = (NOT B AND NOT E AND D) OR (NOT A AND D) OR (NOT B AND C AND D)
        // !c && h && d
        NOT(C, T), // T = NOT C
        AND(D, T), // T = NOT C AND D
        AND(H, T), // T = NOT C AND D AND H
        OR(T, J), // J = (NOT B AND NOT E AND D) OR (NOT A AND D) OR (NOT B AND C AND D) OR (NOT C AND D AND H)
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
    use super::*;
    use itertools::Itertools;

    fn regs(pos_regs: &[bool]) -> Vec<bool> {
        let mut regs = vec![false, false]; // J and T
        regs.extend(pos_regs);
        regs
    }

    // Executes the given springscript for the set if position registers.
    fn exec_springscript(instructions: &[Instruction], regs: &[bool]) -> bool {
        let mut regs = regs.to_vec();
        // println!("Pos regs: {:?}", &regs[2..]);
        for ins in instructions {
            ins.exec(&mut regs);
            // println!(
            //     "{}: J={} T={}",
            //     ins,
            //     regs[Reg::J.index()],
            //     regs[Reg::T.index()]
            // );
        }
        regs[Reg::J.index()]
    }

    // Executes the logical expression equivalent to the WALK springscript.
    fn exec_walk(r: &[bool]) -> bool {
        let (a, _b, c, d) = (r[2], r[3], r[4], r[5]);
        (!a || !c) && d
    }

    // Executes the logical expression equivalent to the RUN springscript.
    fn exec_run(r: &[bool]) -> bool {
        let (a, b, c, d, e, _f, _g, h, _i) =
            (r[2], r[3], r[4], r[5], r[6], r[7], r[8], r[9], r[10]);
        (!b && d && !e) || (!a && d) || (!b && c && d) || (!c && d && h)
    }

    // Runs all the possible combinations of registers for both the springscript and the logical expression.
    fn verify_instructions(
        instructions: &[Instruction],
        sensor_view: usize,
        exec_fn: fn(&[bool]) -> bool,
    ) {
        let combi: Vec<_> = itertools::repeat_n([false, true].iter(), sensor_view)
            .multi_cartesian_product()
            .collect();
        for p in combi {
            let r: Vec<bool> = p.iter().copied().copied().collect(); // ugly..
            let springscript_result = exec_springscript(instructions, &regs(&r));
            let converted_result = exec_fn(&regs(&r));
            assert_eq!(springscript_result, converted_result, "Failed for {r:?}");
        }
    }

    #[test]
    fn test_walk() {
        let instructions = get_walk_instructions();
        verify_instructions(&instructions, 4, exec_walk);
    }

    #[test]
    fn test_run() {
        let instructions = get_run_instructions();
        verify_instructions(&instructions, 9, exec_run);
    }
}
