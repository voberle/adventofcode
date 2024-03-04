use std::io::{self, Read};

fn build(input: &str) -> Vec<i32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

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
        // println!("{}: {:?}", opcode, modes);
        match opcode {
            1 => { // Add
                let a = get_val(program, ip + 1, modes[0]);
                let b = get_val(program, ip + 2, modes[1]);
                assert_eq!(modes[2], Position);
                let c = program[ip + 3] as usize;
                program[c] = a + b;
                ip += 4;
            }
            2 => { // Mult
                let a = get_val(program, ip + 1, modes[0]);
                let b = get_val(program, ip + 2, modes[1]);
                assert_eq!(modes[2], Position);
                let c = program[ip + 3] as usize;
                program[c] = a * b;
                ip += 4;
            }
            3 => { // Input
                assert_eq!(modes[0], Position);
                let a = program[ip + 1] as usize;
                // Should input be a VecDeque to pop from front?
                program[a] = input.pop().unwrap();
                ip += 2;
            }
            4 => { // Output
                let a = get_val(program, ip + 1, modes[0]);
                output.push(a);
                ip += 2;
            }
            5 => { // JumpIfTrue
                let a = get_val(program, ip + 1, modes[0]);
                if a != 0 {
                    let b = get_val(program, ip + 2, modes[1]);
                    ip = b as usize;
                } else {
                    ip += 3;
                }
            }
            6 => { // JumpIfFalse
                let a = get_val(program, ip + 1, modes[0]);
                if a == 0 {
                    let b = get_val(program, ip + 2, modes[1]);
                    ip = b as usize;
                } else {
                    ip += 3;
                }
            }
            7 => { // LessThan
                let a = get_val(program, ip + 1, modes[0]);
                let b = get_val(program, ip + 2, modes[1]);
                assert_eq!(modes[2], Position);
                let c = program[ip + 3] as usize;
                program[c] = i32::from(a < b);
                ip += 4;
            }
            8 => { // Equal
                let a = get_val(program, ip + 1, modes[0]);
                let b = get_val(program, ip + 2, modes[1]);
                assert_eq!(modes[2], Position);
                let c = program[ip + 3] as usize;
                program[c] = i32::from(a == b);
                ip += 4;
            }
            99 => break, // Halt
            _ => panic!("Unknown opcode"),
        }
    }
}

fn run_diagnostic_test(program: &[i32], system_to_test_id: i32) -> i32 {
    let mut program = program.to_vec();
    let mut input = vec![system_to_test_id];
    let mut output = Vec::new();
    exec(&mut program, &mut input, &mut output);
    // println!("{:?}", output);
    *output.last().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = build(&input);

    println!("Part 1: {}", run_diagnostic_test(&program, 1));
    println!("Part 2: {}", run_diagnostic_test(&program, 5));
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

    fn run(code: &str, input: i32) -> i32 {
        let mut program = build(code);
        let mut input = vec![input];
        let mut output = Vec::new();
        exec(&mut program, &mut input, &mut output);
        *output.last().unwrap()
    }

    #[test]
    fn test_cmp() {
        let c = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(run(c, 8), 1);
        assert_eq!(run(c, 3), 0);
        let c = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(run(c, 3), 1);
        assert_eq!(run(c, 9), 0);
        let c = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(run(c, 8), 1);
        assert_eq!(run(c, 3), 0);
        let c = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(run(c, 3), 1);
        assert_eq!(run(c, 9), 0);
    }

    #[test]
    fn test_jump() {
        let c = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(run(c, 0), 0);
        assert_eq!(run(c, 4), 1);
        let c = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(run(c, 0), 0);
        assert_eq!(run(c, 4), 1);
    }

    #[test]
    fn test_larger_program() {
        let c = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(run(c, 1), 999);
        assert_eq!(run(c, 8), 1000);
        assert_eq!(run(c, 45), 1001);
    }
}
