//! The Intcode computer.
//!
use itertools::Itertools;
use std::{collections::VecDeque, io::Read};

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

// To flag which params are the ones we write to.
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

#[allow(clippy::enum_glob_use)]
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
        use Instruction::*;
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
            1 => Add(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            2 => Mult(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            3 => Input(next_a(&mut i)),
            4 => Output(next_p(&mut i)),
            5 => JumpIfTrue(next_p(&mut i), next_p(&mut i)),
            6 => JumpIfFalse(next_p(&mut i), next_p(&mut i)),
            7 => LessThan(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            8 => Equal(next_p(&mut i), next_p(&mut i), next_a(&mut i)),
            9 => ChangeRelativeBase(next_p(&mut i)),
            99 => Halt,
            _ => panic!("Unknown opcode {}", opcode),
        }
    }

    fn param_count(self) -> usize {
        use Instruction::*;
        match self {
            Halt => 0,
            Input { .. } | Output { .. } | ChangeRelativeBase { .. } => 1,
            JumpIfTrue { .. } | JumpIfFalse { .. } => 2,
            Add { .. } | Mult { .. } | LessThan { .. } | Equal { .. } => 3,
        }
    }

    fn length(self) -> usize {
        self.param_count() + 1
    }
}

// Trait that abstracts Input/Output support in the base computer.
trait Bus {
    fn read(&mut self) -> Option<i64>;
    fn write(&mut self, v: i64);
}

#[derive(Debug, Clone)]
struct IntcodeBase {
    mem: Vec<i64>,
    ip: usize,
    relative_base: i64,
    halted: bool,
}

impl IntcodeBase {
    /// Builds a Intcode computer from a list of integers separated by commas.
    fn build(code: &str) -> Self {
        Self {
            mem: code.split(',').map(|v| v.parse().unwrap()).collect(),
            ip: 0,
            relative_base: 0,
            halted: false,
        }
    }

    fn ensure_mem_capacity(&mut self, capacity: usize) {
        if self.mem.len() <= capacity {
            self.mem.resize(capacity + 1, 0);
        }
    }

    fn get_mem(&mut self, addr: usize) -> i64 {
        self.ensure_mem_capacity(addr);
        self.mem[addr]
    }

    fn set_mem(&mut self, addr: usize, val: i64) {
        self.ensure_mem_capacity(addr);
        self.mem[addr] = val;
    }

    fn get(&mut self, p: &Param) -> i64 {
        match p {
            Position(addr) => self.get_mem(*addr),
            Immediate(val) => *val,
            Relative(addr) => self.get_mem((self.relative_base + *addr).try_into().unwrap()),
        }
    }

    fn get_address(&mut self, p: &Param) -> usize {
        self.get(p).try_into().unwrap()
    }

    fn set(&mut self, p: &Param, val: i64) {
        let addr: usize = match p {
            Position(addr) => *addr,
            Immediate(_) => panic!("Cannot write to immediate mode value"),
            Relative(addr) => (self.relative_base + *addr).try_into().unwrap(),
        };
        self.set_mem(addr, val);
    }

    /// Executes the instructions.
    ///
    /// This function returns when reaching the end of the program (a Halt instruction),
    /// or if trying to get some input, but there isn't any.
    /// The difference can be checked with the `is_halted()` function.
    fn exec<B: Bus>(&mut self, bus: &mut B) {
        assert!(!self.is_halted(), "Computer isn't running");
        loop {
            let ins = Instruction::new(&self.mem[self.ip..]);
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
                    if let Some(val) = bus.read() {
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
                    bus.write(a);
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
                    self.ip += ins.length();
                }
            }
        }
    }

    fn is_halted(&self) -> bool {
        self.halted
    }

    fn dump_memory(&self) -> String {
        self.mem.iter().join(",")
    }

    fn read_mem(&mut self, addr: usize) -> i64 {
        self.mem[addr]
    }

    fn write_mem(&mut self, addr: usize, val: i64) {
        self.mem[addr] = val;
    }
}

/// Simple vector based implementation of Bus trait.
#[derive(Debug, Clone)]
pub struct InputOutput {
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl InputOutput {
    fn new() -> Self {
        Self {
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    // Multiple inputs are added in the order the computer reads them (first add the first one the computer reads).
    pub fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn extend_input(&mut self, input: &[i64]) {
        self.input.extend(input);
    }

    // Returns first the oldest output of the computer.
    pub fn get_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    #[must_use]
    pub fn dump_input(&self) -> String {
        self.input.iter().join(",")
    }

    #[must_use]
    pub fn dump_output(&self) -> String {
        self.output.iter().join(",")
    }
}

impl Bus for InputOutput {
    fn read(&mut self) -> Option<i64> {
        self.input.pop_front()
    }

    fn write(&mut self, v: i64) {
        self.output.push_back(v);
    }
}

/// Intcode computer that uses two vectors for I/O.
#[derive(Debug, Clone)]
pub struct IntcodeComputer {
    base: IntcodeBase,
    pub io: InputOutput,
}

impl IntcodeComputer {
    /// Builds a Intcode computer from a list of integers separated by commas.
    ///
    /// # Panics
    ///
    /// Will panic if input is invalid.
    #[must_use]
    pub fn build(code: &str) -> Self {
        Self {
            base: IntcodeBase::build(code),
            io: InputOutput::new(),
        }
    }

    /// Executes the instructions.
    /// This function returns when reaching the end of the program (a Halt instruction),
    /// or if trying to get some input, but the `input` vector is empty.
    /// The difference can be checked with the `is_halted()` function.
    pub fn exec(&mut self) {
        self.base.exec(&mut self.io);
    }

    // Execute the program with given integer as input, returning last integer from output.
    ///
    /// # Panics
    ///
    /// Will panic if there is no output.
    #[must_use]
    pub fn run(&mut self, input: i64) -> i64 {
        self.io.add_input(input);
        self.exec();
        self.io.get_output().unwrap()
    }

    #[must_use]
    pub fn is_halted(&self) -> bool {
        self.base.is_halted()
    }

    #[must_use]
    pub fn dump_memory(&self) -> String {
        self.base.dump_memory()
    }

    #[must_use]
    pub fn read_mem(&mut self, addr: usize) -> i64 {
        self.base.read_mem(addr)
    }

    pub fn write_mem(&mut self, addr: usize, val: i64) {
        self.base.write_mem(addr, val);
    }
}

/// Stdin/stdout based implementation of Bus trait.
pub struct ASCIIInputOutput {}

impl ASCIIInputOutput {
    fn new() -> Self {
        Self {}
    }
}

impl Bus for ASCIIInputOutput {
    fn read(&mut self) -> Option<i64> {
        std::io::stdin()
            .bytes()
            .next()
            .and_then(std::result::Result::ok)
            .map(i64::from)
    }

    fn write(&mut self, v: i64) {
        // Convert the value to ASCII.
        let c = char::from_u32(u32::try_from(v).unwrap()).unwrap();
        print!("{}", c);
    }
}

/// ASCII Intcode computer.
pub struct ASCIIIntcodeComputer {
    base: IntcodeBase,
    io: ASCIIInputOutput,
}

impl ASCIIIntcodeComputer {
    /// Builds a Intcode computer from a list of integers separated by commas.
    ///
    /// # Panics
    ///
    /// Will panic if input is invalid.
    #[must_use]
    pub fn build(code: &str) -> Self {
        Self {
            // Trim to avoid trouble with extra new line at the end.
            base: IntcodeBase::build(code.trim()),
            io: ASCIIInputOutput::new(),
        }
    }

    /// Executes the instructions.
    ///
    /// This function returns when reaching the end of the program (a Halt instruction).
    /// It quits if if trying to get some input, but the `input` vector is empty.
    pub fn exec(&mut self) {
        self.base.exec(&mut self.io);
    }
}
