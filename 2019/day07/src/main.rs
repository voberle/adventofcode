use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{self, Read},
};

mod previous_days;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Param {
    Position(usize),
    Immediate(i32),
}
use Param::{Immediate, Position};

impl Param {
    fn new(program: &[i32], loc: usize, mode: i32) -> Self {
        match mode {
            0 => Position(program[loc].try_into().unwrap()),
            1 => Immediate(program[loc]),
            _ => panic!("Invalid parameter mode {}", mode),
        }
    }

    fn get_val(&self, program: &[i32]) -> i32 {
        match self {
            Position(addr) => program[*addr],
            Immediate(val) => *val,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Address(usize);

impl Address {
    fn new(program: &[i32], loc: usize, mode: i32) -> Self {
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
    fn get_opcode_mode(i: i32) -> (i32, [i32; 3]) {
        assert_eq!(i / 10000, 0); // no more than 3 param modes
        (i % 100, [(i / 100) % 10, (i / 1000) % 10, (i / 10000) % 10])
    }

    // Builds the instruction that starts at index 0 of this program.
    fn new(program: &[i32]) -> Self {
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
    mem: Vec<i32>,
    ip: usize,
    input: VecDeque<i32>,
    output: Vec<i32>,
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

    fn get(&self, p: &Param) -> i32 {
        p.get_val(&self.mem)
    }

    fn get_address(&self, p: &Param) -> usize {
        let addr = self.get(p);
        addr.try_into().unwrap()
    }

    fn set(&mut self, p: Address, val: i32) {
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
                    self.set(c, i32::from(self.get(&a) < self.get(&b)));
                    self.ip += ins.length();
                }
                Instruction::Equal(a, b, c) => {
                    self.set(c, i32::from(self.get(&a) == self.get(&b)));
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

fn build_amp(computer: &IntcodeComputer, phase_setting: i32) -> IntcodeComputer {
    let mut amp = computer.clone();
    amp.input.push_back(phase_setting);
    amp
}

fn exec_amp(amp: &mut IntcodeComputer, input: i32) -> i32 {
    amp.input.push_back(input);
    amp.exec();
    amp.output.pop().unwrap()
}

fn build_and_exec(computer: &IntcodeComputer, input: i32, phase_setting: i32) -> i32 {
    let mut amp = build_amp(computer, phase_setting);
    exec_amp(&mut amp, input)
}

fn get_thruster_signal(computer: &IntcodeComputer, phase_settings: &[i32]) -> i32 {
    let a_output = build_and_exec(computer, 0, phase_settings[0]);
    let b_output = build_and_exec(computer, a_output, phase_settings[1]);
    let c_output = build_and_exec(computer, b_output, phase_settings[2]);
    let d_output = build_and_exec(computer, c_output, phase_settings[3]);
    build_and_exec(computer, d_output, phase_settings[4])
}

fn max_thruster_signal(computer: &IntcodeComputer) -> i32 {
    (0..=4)
        .permutations(5)
        .map(|phase_settings| get_thruster_signal(computer, &phase_settings))
        .max()
        .unwrap()
}

fn get_thruster_signal_with_feedback(computer: &IntcodeComputer, phase_settings: &[i32]) -> i32 {
    let mut amp_a = build_amp(computer, phase_settings[0]);
    let mut amp_b = build_amp(computer, phase_settings[1]);
    let mut amp_c = build_amp(computer, phase_settings[2]);
    let mut amp_d = build_amp(computer, phase_settings[3]);
    let mut amp_e = build_amp(computer, phase_settings[4]);

    let mut e_output = 0;
    while !amp_e.halted {
        let a_output = exec_amp(&mut amp_a, e_output);
        let b_output = exec_amp(&mut amp_b, a_output);
        let c_output = exec_amp(&mut amp_c, b_output);
        let d_output = exec_amp(&mut amp_d, c_output);
        e_output = exec_amp(&mut amp_e, d_output);
    }
    e_output
}

fn max_thruster_signal_with_feedback(computer: &IntcodeComputer) -> i32 {
    (5..=9)
        .permutations(5)
        .map(|phase_settings| get_thruster_signal_with_feedback(computer, &phase_settings))
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", max_thruster_signal(&computer));
    println!("Part 2: {}", max_thruster_signal_with_feedback(&computer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_thruster_signal() {
        let computer = IntcodeComputer::build("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(max_thruster_signal(&computer), 43210);

        let computer = IntcodeComputer::build(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(max_thruster_signal(&computer), 54321);

        let computer = IntcodeComputer::build("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        assert_eq!(max_thruster_signal(&computer), 65210);
    }

    #[test]
    fn test_max_thruster_signal_with_feedback() {
        let computer = IntcodeComputer::build(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(max_thruster_signal_with_feedback(&computer), 139629729);

        let computer = IntcodeComputer::build("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        assert_eq!(max_thruster_signal_with_feedback(&computer), 18216);
    }
}
