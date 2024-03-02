use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn exec(program: &mut [u32]) {
    let mut ip = 0;
    loop {
        let opcode = program[ip];
        match opcode {
            1 => {
                let a = program[ip + 1] as usize;
                let b = program[ip + 2] as usize;
                let c = program[ip + 3] as usize;
                program[c] = program[a] + program[b];
                ip += 4;
            }
            2 => {
                let a = program[ip + 1] as usize;
                let b = program[ip + 2] as usize;
                let c = program[ip + 3] as usize;
                program[c] = program[a] * program[b];
                ip += 4;
            }
            99 => break,
            _ => panic!("Unknown opcode"),
        }
        // println!("{}: {:?}", opcode, program);
    }
}

fn run(program: &[u32], noun: u32, verb: u32) -> u32 {
    let mut program = program.to_vec();
    program[1] = noun;
    program[2] = verb;
    exec(&mut program);
    program[0]
}

fn find_noun_verb(program: &[u32]) -> u32 {
    const TARGET: u32 = 19_690_720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let output = run(program, noun, verb);
            if output == TARGET {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Target output not found")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = build(&input);

    println!("Part 1: {}", run(&program, 12, 2));
    println!("Part 2: {}", find_noun_verb(&program));
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn exec_w(input: &str) -> String {
        let mut program = build(input);
        exec(&mut program);
        program.iter().join(",")
    }

    #[test]
    fn test_exec() {
        assert_eq!(
            exec_w("1,9,10,3,2,3,11,0,99,30,40,50"),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
        assert_eq!(exec_w("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(exec_w("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(exec_w("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(exec_w("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }
}
