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
}
use Param::{Immediate, Position};

impl Param {
    fn new(program: &[i64], loc: usize, mode: i64) -> Self {
        match mode {
            0 => Position(program[loc].try_into().unwrap()),
            1 => Immediate(program[loc]),
            _ => panic!("Invalid parameter mode {}", mode),
        }
    }

    fn get_val(&self, program: &[i64]) -> i64 {
        match self {
            Position(addr) => program[*addr],
            Immediate(val) => *val,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Address(usize);

impl Address {
    fn new(program: &[i64], loc: usize, mode: i64) -> Self {
        match mode {
            0 => Self(program[loc].try_into().unwrap()),
            1 => panic!("Immediate mode not supported for writing to"),
            _ => panic!("Invalid parameter mode {}", mode),
        }
    }

    fn get_address(self) -> usize {
        self.0
    }
}

impl From<usize> for Address {
    fn from(item: usize) -> Self {
        Address(item)
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Param, Param, Address),
    Mult(Param, Param, Address),
    Input(Address),
    Output(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Address),
    Equal(Param, Param, Address),
    Halt,
}

impl Instruction {
    // Extract the opcode and parameter modes from an integer.
    fn get_opcode_mode(i: i64) -> (i64, [i64; 3]) {
        assert_eq!(i / 10000, 0); // no more than 3 param modes
        (i % 100, [(i / 100) % 10, (i / 1000) % 10, (i / 10000) % 10])
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
            let p = Address::new(program, *index + 1, modes[*index]);
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
            99 => Instruction::Halt,
            _ => panic!("Unknown opcode {}", opcode),
        }
    }

    fn param_count(self) -> usize {
        match self {
            Instruction::Halt => 0,
            Instruction::Input(_) | Instruction::Output(_) => 1,
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
    input: VecDeque<i64>,
    output: Vec<i64>,
    halted: bool,
}

impl IntcodeComputer {
    fn build(input: &str) -> Self {
        Self {
            mem: input.split(',').map(|v| v.parse().unwrap()).collect(),
            ip: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            halted: false,
        }
    }

    #[allow(dead_code)]
    fn dump_memory(&self) -> String {
        self.mem.iter().join(",")
    }

    fn get(&self, p: &Param) -> i64 {
        p.get_val(&self.mem)
    }

    fn get_address(&self, p: &Param) -> usize {
        let addr = self.get(p);
        addr.try_into().unwrap()
    }

    fn set(&mut self, p: Address, val: i64) {
        self.mem[p.get_address()] = val;
    }

    fn exec(&mut self) {
        loop {
            let ins = Instruction::new(&self.mem[self.ip..]);
            // println!("[{}] {:?}", self.ip, ins);
            match ins {
                Instruction::Add(a, b, c) => {
                    self.set(c, self.get(&a) + self.get(&b));
                    self.ip += ins.length();
                }
                Instruction::Mult(a, b, c) => {
                    self.set(c, self.get(&a) * self.get(&b));
                    self.ip += ins.length();
                }
                Instruction::Input(a) => {
                    if let Some(val) = self.input.pop_front() {
                        self.set(a, val);
                        self.ip += ins.length();
                    } else {
                        // Interrupt the execution loop. Program isn't halted, we are just waiting for more input.
                        // Since ip is a self variable, program will continue at right instruction.
                        break;
                    }
                }
                Instruction::Output(a) => {
                    self.output.push(self.get(&a));
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
                    self.set(c, i64::from(self.get(&a) < self.get(&b)));
                    self.ip += ins.length();
                }
                Instruction::Equal(a, b, c) => {
                    self.set(c, i64::from(self.get(&a) == self.get(&b)));
                    self.ip += ins.length();
                }
                Instruction::Halt => {
                    self.halted = true;
                    break;
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
    fn test_large_memory() {
        let mut computer = IntcodeComputer::build("1102,34915192,34915192,7,4,7,99,0");
        computer.exec();
        assert_eq!(computer.output[0], 1219070632396864);
    }

    #[test]
    fn test_large_numbers() {
        let mut computer = IntcodeComputer::build("104,1125899906842624,99");
        computer.exec();
        assert_eq!(computer.output[0], 1125899906842624);
    }
}
