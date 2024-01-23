use std::{
    io::{self, Read},
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

use fxhash::FxHashMap;

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[derive(Debug)]
struct Registers<T> {
    regs: FxHashMap<char, T>,
}

impl<T> Registers<T>
where
    T: std::str::FromStr,
    T: Copy,
    T: Default,
{
    fn new() -> Self {
        Self {
            regs: FxHashMap::default(),
        }
    }

    fn get(&self, r: char) -> T {
        self.regs.get(&r).copied().unwrap_or_default()
    }

    fn set(&mut self, r: char, val: T) {
        self.regs.insert(r, val);
    }

    fn get_ic(&self, x: IntChar<T>) -> T {
        match x {
            IntChar::Integer(val) => val,
            IntChar::Char(src) => self.get(src),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum IntChar<T>
where
    T: std::str::FromStr,
{
    Integer(T),
    Char(char),
}

impl<T> IntChar<T>
where
    T: std::str::FromStr,
{
    fn new(s: &str) -> Self {
        if let Ok(val) = s.parse() {
            IntChar::Integer(val)
        } else if s.len() == 1 {
            IntChar::Char(s.chars().next().unwrap())
        } else {
            panic!("Invalid string for building IntChar")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Snd(IntChar<i64>),
    Set(char, IntChar<i64>),
    Add(char, IntChar<i64>),
    Mul(char, IntChar<i64>),
    Mod(char, IntChar<i64>),
    Rcv(char), // In theory could be IntChar but input and part 2 limits to a register.
    JumpGreaterThanZero(IntChar<i64>, IntChar<i64>),
    Nop,
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "snd" => Self::Snd(IntChar::new(parts[1])),
            "set" => Self::Set(char(parts[1]), IntChar::new(parts[2])),
            "add" => Self::Add(char(parts[1]), IntChar::new(parts[2])),
            "mul" => Self::Mul(char(parts[1]), IntChar::new(parts[2])),
            "mod" => Self::Mod(char(parts[1]), IntChar::new(parts[2])),
            "rcv" => Self::Rcv(char(parts[1])),
            "jgz" => Self::JumpGreaterThanZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "nop" => Self::Nop,
            _ => panic!("Unknown instruction"),
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

// Executes the instruction: Common parts for part 1 and 2.
fn execute_common(ins: &Instruction, ir: &mut usize, regs: &mut Registers<i64>) {
    match ins {
        Instruction::Set(x, y) => {
            regs.set(*x, regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::Add(x, y) => {
            regs.set(*x, regs.get(*x) + regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::Mul(x, y) => {
            regs.set(*x, regs.get(*x) * regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::Mod(x, y) => {
            regs.set(*x, regs.get(*x) % regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::JumpGreaterThanZero(x, y) => {
            if regs.get_ic(*x) > 0 {
                *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
            } else {
                *ir += 1;
            }
        }
        Instruction::Nop => *ir += 1,
        _ => panic!("Wrong use of this function"),
    }
}

// Executes the instruction specified by ins, modifying the registers if needed.
fn execute_sound_playing(
    instructions: &[Instruction],
    ir: &mut usize,
    regs: &mut Registers<i64>,
    last_sound_played: &mut i64,
) -> Option<i64> {
    let ins = &instructions[*ir];
    match ins {
        Instruction::Snd(x) => {
            // plays a sound with a frequency equal to the value of X
            *last_sound_played = regs.get_ic(*x);
            *ir += 1;
        }
        Instruction::Rcv(x) => {
            if regs.get(*x) != 0 {
                // recovers the frequency of the last sound played
                return Some(*last_sound_played);
            }
            *ir += 1;
        }
        _ => execute_common(ins, ir, regs),
    }
    None
}

fn recovered_frequency_value(instructions: &[Instruction]) -> i64 {
    let mut regs = Registers::new();
    let mut last_sound_played = 0;
    let mut ir = 0;
    while ir < instructions.len() {
        // println!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        if let Some(recv_snd) =
            execute_sound_playing(instructions, &mut ir, &mut regs, &mut last_sound_played)
        {
            return recv_snd;
        }
    }
    panic!("Didn't find a recovered sound")
}

// Returns true if a value was sent, Err if we timed out on receive.
fn execute_send_receive(
    instructions: &Arc<Vec<Instruction>>,
    ir: &mut usize,
    regs: &mut Registers<i64>,
    sender: &mpsc::Sender<i64>,
    receiver: &mpsc::Receiver<i64>,
) -> Result<bool, &'static str> {
    let ins = &instructions[*ir];
    match ins {
        Instruction::Snd(x) => {
            let val = regs.get_ic(*x);
            if sender.send(val).is_err() {
                panic!("Failed to send {}", val);
            }
            *ir += 1;
            return Ok(true);
        }
        Instruction::Rcv(x) => {
            // Receives next val and stores it in register X
            if let Ok(val) = receiver.recv_timeout(Duration::from_secs(1)) {
                regs.set(*x, val);
            } else {
                return Err("Timeout on receiving a value");
            }
            *ir += 1;
        }
        _ => execute_common(ins, ir, regs),
    }
    Ok(false)
}

// Starts a thread to execute the program.
// When the thread exists, it returns how many times it sent a value.
fn start_exec_thread(
    instructions: &Arc<Vec<Instruction>>,
    program_id: i64,
    sender: mpsc::Sender<i64>,
    receiver: mpsc::Receiver<i64>,
) -> thread::JoinHandle<usize> {
    let instructions_for_child = instructions.clone();
    thread::spawn(move || {
        let mut regs = Registers::new();
        regs.set('p', program_id);

        let mut ir = 0;
        let mut sent_values_count = 0;
        while ir < instructions_for_child.len() {
            if let Ok(is_sent) = execute_send_receive(
                &instructions_for_child,
                &mut ir,
                &mut regs,
                &sender,
                &receiver,
            ) {
                if is_sent {
                    sent_values_count += 1;
                }
            } else {
                // On error, quit the thread
                break;
            }
        }
        sent_values_count
    })
}

#[allow(clippy::let_and_return)]
fn program_1_send_count(instructions: &Arc<Vec<Instruction>>) -> usize {
    // Create two channels, one for each direction.
    let (sender1to2, receiver1to2) = mpsc::channel();
    let (sender2to1, receiver2to1) = mpsc::channel();
    let h0 = start_exec_thread(&instructions.clone(), 0, sender1to2, receiver2to1);
    let h1 = start_exec_thread(&instructions.clone(), 1, sender2to1, receiver1to2);

    h0.join().unwrap();
    let r1 = h1.join().unwrap();

    r1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", recovered_frequency_value(&instructions));
    println!("Part 2: {}", program_1_send_count(&Arc::new(instructions)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(recovered_frequency_value(&build(INPUT_TEST_1)), 4);
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part2() {
        let instructions = build(INPUT_TEST_2);
        assert_eq!(program_1_send_count(&Arc::new(instructions)), 3);
    }
}
