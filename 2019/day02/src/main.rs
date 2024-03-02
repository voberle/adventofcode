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

fn val_pos0(program: &[u32]) -> u32 {
    let mut program = program.to_vec();
    program[1] = 12;
    program[2] = 2;
    exec(&mut program);
    program[0]
}

fn part2(program: &[u32]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let program = build(&input);

    println!("Part 1: {}", val_pos0(&program));
    println!("Part 2: {}", part2(&program));
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn run(input: &str) -> String {
        let mut program = build(input);
        exec(&mut program);
        program.iter().join(",")
    }

    #[test]
    fn test_exec() {
        assert_eq!(
            run("1,9,10,3,2,3,11,0,99,30,40,50"),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
        assert_eq!(run("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(run("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(run("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(run("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
