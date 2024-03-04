use std::io::{self, Read};

fn build(input: &str) -> Vec<i32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Add,
    Mult,
    Input,
    Output,
    Halt,
}

impl Opcode {}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParamModes {
    Position,
    Immediate,
}
use ParamModes::{Immediate, Position};

impl ParamModes {
    fn new(i: i32) -> Self {
        match i {
            0 => Position,
            1 => Immediate,
            _ => panic!("Invalid parameter mode {}", i),
        }
    }
}

fn get_opcode_parameter_mode(i: i32) -> (i32, [ParamModes; 3]) {
    assert_eq!(i / 10000, 0); // no more than 3 param modes
    let opcode = i % 100;
    let remain = i / 100;
    (
        opcode,
        [
            ParamModes::new(remain % 10),
            ParamModes::new(remain / 10 % 10),
            ParamModes::new(remain / 100 % 10),
        ],
    )
}

fn get_val(program: &mut [i32], loc: usize, mode: ParamModes) -> i32 {
    match mode {
        Position => {
            let addr = program[loc] as usize;
            program[addr]
        }
        Immediate => program[loc],
    }
}

fn exec(program: &mut [i32], input: &mut Vec<i32>, output: &mut Vec<i32>) {
    let mut ip = 0;
    loop {
        let (opcode, modes) = get_opcode_parameter_mode(program[ip]);
        println!("{}: {:?}", opcode, modes);
        match opcode {
            1 => {
                let a = get_val(program, ip + 1, modes[0]);
                let b = get_val(program, ip + 2, modes[1]);
                assert_eq!(modes[2], Position);
                let c = program[ip + 3] as usize;
                program[c] = a + b;
                ip += 4;
            }
            2 => {
                let a = get_val(program, ip + 1, modes[0]);
                let b = get_val(program, ip + 2, modes[1]);
                assert_eq!(modes[2], Position);
                let c = program[ip + 3] as usize;
                program[c] = a * b;
                ip += 4;
            }
            3 => {
                assert_eq!(modes[0], Position);
                let a = program[ip + 1] as usize;
                // Should input be a VecDeque to pop from front?
                program[a] = input.pop().unwrap();
                ip += 2;
            }
            4 => {
                let a = get_val(program, ip + 1, modes[0]);
                output.push(a);
                ip += 2;
            }
            99 => break,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn run_diagnostic_test(program: &[i32]) -> i32 {
    let mut program = program.to_vec();
    let mut input = vec![1];
    let mut output = Vec::new();
    exec(&mut program, &mut input, &mut output);
    println!("{:?}", output);
    *output.last().unwrap()
}

fn part2(program: &[i32]) -> i32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = build(&input);

    println!("Part 1: {}", run_diagnostic_test(&program));
    println!("Part 2: {}", part2(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opcode_parameter_mode() {
        assert_eq!(
            get_opcode_parameter_mode(3),
            (3, [Position, Position, Position])
        );
        assert_eq!(
            get_opcode_parameter_mode(101),
            (1, [Immediate, Position, Position])
        );
        assert_eq!(
            get_opcode_parameter_mode(1002),
            (2, [Position, Immediate, Position])
        );
    }

    #[test]
    fn test_part1() {
        // assert_eq!(part1(&build(INPUT_TEST)), 0);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
