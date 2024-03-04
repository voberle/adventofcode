use itertools::Itertools;
use std::io::{self, Read};

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
    // Should input be a VecDeque to pop from front?
    input: Vec<i32>,
    output: Vec<i32>,
}

impl IntcodeComputer {
    fn build(input: &str) -> Self {
        Self {
            mem: input.split(',').map(|v| v.parse().unwrap()).collect(),
            input: Vec::new(),
            output: Vec::new(),
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
        let mut ip = 0;
        loop {
            let ins = Instruction::new(&self.mem[ip..]);
            match ins {
                Instruction::Add(a, b, c) => {
                    self.set(c, self.get(&a) + self.get(&b));
                    ip += ins.length();
                }
                Instruction::Mult(a, b, c) => {
                    self.set(c, self.get(&a) * self.get(&b));
                    ip += ins.length();
                }
                Instruction::Input(a) => {
                    let val = self.input.pop().unwrap();
                    self.set(a, val);
                    ip += ins.length();
                }
                Instruction::Output(a) => {
                    self.output.push(self.get(&a));
                    ip += ins.length();
                }
                Instruction::JumpIfTrue(a, b) => {
                    if self.get(&a) != 0 {
                        ip = self.get_address(&b);
                    } else {
                        ip += ins.length();
                    }
                }
                Instruction::JumpIfFalse(a, b) => {
                    if self.get(&a) == 0 {
                        ip = self.get_address(&b);
                    } else {
                        ip += ins.length();
                    }
                }
                Instruction::LessThan(a, b, c) => {
                    self.set(c, i32::from(self.get(&a) < self.get(&b)));
                    ip += ins.length();
                }
                Instruction::Equal(a, b, c) => {
                    self.set(c, i32::from(self.get(&a) == self.get(&b)));
                    ip += ins.length();
                }
                Instruction::Halt => break,
            }
        }
    }
}

fn run_amp(computer: &IntcodeComputer, input: i32, phase_setting: i32) -> i32 {
    let mut amp = computer.clone();
    amp.input.push(input);
    amp.input.push(phase_setting);
    amp.exec();
    amp.output.pop().unwrap()
}

fn get_thruster_signal(computer: &IntcodeComputer, phase_settings: &[i32]) -> i32 {
    let a_output = run_amp(computer, 0, phase_settings[0]);
    let b_output = run_amp(computer, a_output, phase_settings[1]);
    let c_output = run_amp(computer, b_output, phase_settings[2]);
    let d_output = run_amp(computer, c_output, phase_settings[3]);
    run_amp(computer, d_output, phase_settings[4])
}

fn max_thruster_signal(computer: &IntcodeComputer) -> i32 {
    (0..=4)
        .permutations(5)
        .map(|phase_settings| get_thruster_signal(computer, &phase_settings))
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", max_thruster_signal(&computer));
    // println!("Part 2: {}", run_diagnostic_test(&computer, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opcode_parameter_mode() {
        assert_eq!(Instruction::get_opcode_mode(3), (3, [0, 0, 0]));
        assert_eq!(Instruction::get_opcode_mode(101), (1, [1, 0, 0]));
        assert_eq!(Instruction::get_opcode_mode(1002), (2, [0, 1, 0]));
    }

    fn exec(code: &str) -> String {
        let mut computer = IntcodeComputer::build(code);
        computer.exec();
        computer.dump_memory()
    }

    #[test]
    fn test_exec() {
        assert_eq!(
            exec("1,9,10,3,2,3,11,0,99,30,40,50"),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
        assert_eq!(exec("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(exec("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(exec("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(exec("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }

    fn run_io(code: &str, input: i32) -> i32 {
        let mut computer = IntcodeComputer::build(code);
        computer.input.push(input);
        computer.exec();
        *computer.output.last().unwrap()
    }

    #[test]
    fn test_cmp() {
        let c = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(run_io(c, 8), 1);
        assert_eq!(run_io(c, 3), 0);
        let c = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(run_io(c, 3), 1);
        assert_eq!(run_io(c, 9), 0);
        let c = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(run_io(c, 8), 1);
        assert_eq!(run_io(c, 3), 0);
        let c = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(run_io(c, 3), 1);
        assert_eq!(run_io(c, 9), 0);
    }

    #[test]
    fn test_jump() {
        let c = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(run_io(c, 0), 0);
        assert_eq!(run_io(c, 4), 1);
        let c = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(run_io(c, 0), 0);
        assert_eq!(run_io(c, 4), 1);
    }

    #[test]
    fn test_larger_program() {
        let c = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run_io(c, 1), 999);
        assert_eq!(run_io(c, 8), 1000);
        assert_eq!(run_io(c, 45), 1001);
    }

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
}

#[cfg(test)]
mod previous_days {
    use std::fs;

    use super::*;

    fn run_noun_verb(computer: &IntcodeComputer, noun: i32, verb: i32) -> i32 {
        let mut computer = computer.clone();
        computer.set(Address(1), noun);
        computer.set(Address(2), verb);
        computer.exec();
        computer.mem[0]
    }

    #[test]
    #[cfg_attr(not(feature = "previous_days"), ignore)]
    fn day02_part1() {
        let input =
            fs::read_to_string("../day02/resources/input").expect("Unable to read input file");
        let result = fs::read_to_string("../day02/resources/part1.answer")
            .expect("Unable to read input file");

        let computer = IntcodeComputer::build(&input);
        let part1 = run_noun_verb(&computer, 12, 2);
        assert_eq!(part1.to_string(), result.trim());
    }

    fn run_diagnostic_test(computer: &IntcodeComputer, system_to_test_id: i32) -> i32 {
        let mut computer = computer.clone();
        computer.input.push(system_to_test_id);
        computer.exec();
        *computer.output.last().unwrap()
    }

    #[test]
    #[cfg_attr(not(feature = "previous_days"), ignore)]
    fn day05() {
        let input =
            fs::read_to_string("../day05/resources/input").expect("Unable to read input file");
        let result1 = fs::read_to_string("../day05/resources/part1.answer")
            .expect("Unable to read input file");
        let result2 = fs::read_to_string("../day05/resources/part2.answer")
            .expect("Unable to read input file");

        let computer = IntcodeComputer::build(&input);
        let part1 = run_diagnostic_test(&computer, 1);
        assert_eq!(part1.to_string(), result1.trim());

        let part2 = run_diagnostic_test(&computer, 5);
        assert_eq!(part2.to_string(), result2.trim());
    }
}
