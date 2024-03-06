use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self, Read},
};

mod previous_days;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Param {
    Position(usize),
    Immediate(i64),
    Relative(i64), // offset can be negative
}
use Param::{Immediate, Position, Relative};

impl Param {
    const POSITION: i64 = 0;
    const IMMEDIATE: i64 = 1;
    const RELATIVE: i64 = 2;

    fn new(program: &[i64], loc: usize, mode: i64) -> Self {
        match mode {
            Self::POSITION => Position(program[loc].try_into().unwrap()),
            Self::IMMEDIATE => Immediate(program[loc]),
            Self::RELATIVE => Relative(program[loc]),
            _ => panic!("Invalid parameter mode {}", mode),
        }
    }
}

impl From<usize> for Param {
    fn from(item: usize) -> Self {
        Param::Position(item)
    }
}

type WriteParam = Param;

#[derive(Debug)]
enum Instruction {
    Add(Param, Param, WriteParam),
    Mult(Param, Param, WriteParam),
    Input(WriteParam),
    Output(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, WriteParam),
    Equal(Param, Param, WriteParam),
    ChangeRelativeBase(Param),
    Halt,
}

impl Instruction {
    // Extract the opcode and parameter modes from an integer.
    fn get_opcode_mode(i: i64) -> (i64, [i64; 4]) {
        (
            i % 100,
            [
                (i / 100) % 10,
                (i / 1_000) % 10,
                (i / 10_000) % 10,
                (i / 100_000) % 10,
            ],
        )
    }

    // Builds the instruction that starts at index 0 of this program.
    fn new(program: &[i64]) -> Self {
        let (opcode, modes) = Self::get_opcode_mode(program[0]);

        let mut i = 0;
        let next_p = |index: &mut usize| {
            let p = Param::new(program, *index + 1, modes[*index]);
            *index += 1;
            p
        };
        let next_a = |index: &mut usize| {
            let mode = modes[*index];
            assert_ne!(mode, Param::IMMEDIATE);
            let p = Param::new(program, *index + 1, mode);
            *index += 1;
            p
        };

        match opcode {
            1 => Instruction::Add(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            2 => Instruction::Mult(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            3 => Instruction::Input(next_a(&mut i)),
            4 => Instruction::Output(next_p(&mut i)),
            5 => Instruction::JumpIfTrue(next_p(&mut i), next_p(&mut i)),
            6 => Instruction::JumpIfFalse(next_p(&mut i), next_p(&mut i)),
            7 => Instruction::LessThan(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            8 => Instruction::Equal(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            9 => Instruction::ChangeRelativeBase(next_p(&mut i)),
            99 => Instruction::Halt,
            _ => panic!("Unknown opcode {}", opcode),
        }
    }

    fn param_count(self) -> usize {
        match self {
            Instruction::Halt => 0,
            Instruction::Input(_) | Instruction::Output(_) | Instruction::ChangeRelativeBase(_) => {
                1
            }
            Instruction::JumpIfTrue(_, _) | Instruction::JumpIfFalse(_, _) => 2,
            Instruction::Add(_, _, _)
            | Instruction::Mult(_, _, _)
            | Instruction::LessThan(_, _, _)
            | Instruction::Equal(_, _, _) => 3,
        }
    }

    fn length(self) -> usize {
        self.param_count() + 1
    }
}

#[derive(Debug, Clone)]
struct IntcodeComputer {
    mem: Vec<i64>,
    ip: usize,
    relative_base: i64,
    input: VecDeque<i64>,
    output: Vec<i64>,
    halted: bool,
}

impl IntcodeComputer {
    fn build(input: &str) -> Self {
        Self {
            mem: input.split(',').map(|v| v.parse().unwrap()).collect(),
            ip: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            halted: false,
        }
    }

    fn ensure_mem_capacity(&mut self, capacity: usize) {
        if self.mem.len() <= capacity {
            self.mem.resize(capacity + 1, 0);
        }
    }

    #[allow(dead_code)]
    fn dump_memory(&self) -> String {
        self.mem.iter().join(",")
    }

    fn get(&mut self, p: &Param) -> i64 {
        match p {
            Position(addr) => {
                let a = *addr;
                self.ensure_mem_capacity(a);
                self.mem[a]
            }
            Immediate(val) => *val,
            Relative(addr) => {
                let a: usize = (self.relative_base + *addr).try_into().unwrap();
                self.ensure_mem_capacity(a);
                self.mem[a]
            }
        }
    }

    fn get_address(&mut self, p: &Param) -> usize {
        self.get(p).try_into().unwrap()
    }

    fn set(&mut self, p: &Param, val: i64) {
        let addr: usize = match p {
            Position(addr) => *addr,
            Immediate(_) => panic!("get_address not supported for immediate mode"),
            Relative(addr) => (self.relative_base + *addr).try_into().unwrap(),
        };
        self.ensure_mem_capacity(addr);
        self.mem[addr] = val;
    }

    fn exec(&mut self) {
        loop {
            let ins = Instruction::new(&self.mem[self.ip..]);
            // println!("[{}] {:?}", self.ip, ins);
            match ins {
                Instruction::Add(a, b, c) => {
                    let a = self.get(&a);
                    let b = self.get(&b);
                    self.set(&c, a + b);
                    self.ip += ins.length();
                }
                Instruction::Mult(a, b, c) => {
                    let a = self.get(&a);
                    let b = self.get(&b);
                    self.set(&c, a * b);
                    self.ip += ins.length();
                }
                Instruction::Input(a) => {
                    if let Some(val) = self.input.pop_front() {
                        self.set(&a, val);
                        self.ip += ins.length();
                    } else {
                        // Interrupt the execution loop. Program isn't halted, we are just waiting for more input.
                        // Since ip is a self variable, program will continue at right instruction.
                        break;
                    }
                }
                Instruction::Output(a) => {
                    let a = self.get(&a);
                    self.output.push(a);
                    self.ip += ins.length();
                }
                Instruction::JumpIfTrue(a, b) => {
                    if self.get(&a) != 0 {
                        self.ip = self.get_address(&b);
                    } else {
                        self.ip += ins.length();
                    }
                }
                Instruction::JumpIfFalse(a, b) => {
                    if self.get(&a) == 0 {
                        self.ip = self.get_address(&b);
                    } else {
                        self.ip += ins.length();
                    }
                }
                Instruction::LessThan(a, b, c) => {
                    let a = self.get(&a);
                    let b = self.get(&b);
                    self.set(&c, i64::from(a < b));
                    self.ip += ins.length();
                }
                Instruction::Equal(a, b, c) => {
                    let a = self.get(&a);
                    let b = self.get(&b);
                    self.set(&c, i64::from(a == b));
                    self.ip += ins.length();
                }
                Instruction::Halt => {
                    self.halted = true;
                    break;
                }
                Instruction::ChangeRelativeBase(a) => {
                    let val = self.get(&a);
                    self.relative_base += val;
                    // println!("Relative base changed by {}, now {}", val, self.relative_base);
                    self.ip += ins.length();
                }
            }
        }
    }
}

fn get_boost_keycode(computer: &IntcodeComputer) -> i64 {
    let mut computer = computer.clone();
    computer.input.push_back(1);
    computer.exec();
    assert_eq!(
        computer.output.len(),
        1,
        "Failing opcodes {:?}",
        computer.output
    );
    *computer.output.last().unwrap()
}

fn part2(computer: &IntcodeComputer) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", get_boost_keycode(&computer));
    println!("Part 2: {}", part2(&computer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_numbers() {
        let mut computer = IntcodeComputer::build("1102,34915192,34915192,7,4,7,99,0");
        computer.exec();
        assert_eq!(computer.output[0], 1219070632396864);

        let mut computer = IntcodeComputer::build("104,1125899906842624,99");
        computer.exec();
        assert_eq!(computer.output[0], 1125899906842624);
    }

    #[test]
    fn test_relative_mode() {
        let mut computer =
            IntcodeComputer::build("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        computer.exec();
        assert_eq!(
            computer.output.iter().join(","),
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
        );
    }
}
