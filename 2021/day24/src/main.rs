use std::{
    collections::VecDeque,
    io::{self, Read},
};

enum Variable {
    W,
    X,
    Y,
    Z,
    Val(i32),
}

impl Variable {
    fn new(s: &str) -> Self {
        match s {
            "w" => Variable::W,
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            _ => Variable::Val(s.parse().unwrap()),
        }
    }
}

enum Instruction {
    Inp(Variable),
    Add(Variable, Variable),
    Mul(Variable, Variable),
    Div(Variable, Variable),
    Mod(Variable, Variable),
    Eql(Variable, Variable),
}

impl Instruction {
    fn new(s: &str) -> Self {
        let p: Vec<_> = s.split_ascii_whitespace().collect();
        let name = p[0];
        match name {
            "inp" => Instruction::Inp(Variable::new(p[1])),
            "add" => Instruction::Add(Variable::new(p[1]), Variable::new(p[2])),
            "mul" => Instruction::Mul(Variable::new(p[1]), Variable::new(p[2])),
            "div" => Instruction::Div(Variable::new(p[1]), Variable::new(p[2])),
            "mod" => Instruction::Mod(Variable::new(p[1]), Variable::new(p[2])),
            "eql" => Instruction::Eql(Variable::new(p[1]), Variable::new(p[2])),
            _ => panic!("Unregognized instruction"),
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

struct Alu {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
    input: VecDeque<i32>,
}

impl Alu {
    fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            input: VecDeque::new(),
        }
    }

    fn get(&self, var: &Variable) -> i32 {
        match var {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
            Variable::Val(value) => *value,
        }
    }

    fn set(&mut self, var: &Variable, value: i32) {
        match var {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
            Variable::Val(_) => panic!("Cannot set an interger"),
        }
    }

    fn read_input(&mut self) -> i32 {
        self.input.pop_front().unwrap()
    }

    fn exex(&mut self, ins: &Instruction) {
        match ins {
            Instruction::Inp(a) => {
                let i = self.read_input();
                self.set(a, i);
            }
            Instruction::Add(a, b) => self.set(a, self.get(a) + self.get(b)),
            Instruction::Mul(a, b) => self.set(a, self.get(a) * self.get(b)),
            Instruction::Div(a, b) => self.set(a, self.get(a) / self.get(b)),
            Instruction::Mod(a, b) => self.set(a, self.get(a) % self.get(b)),
            Instruction::Eql(a, b) => self.set(a, i32::from(self.get(a) == self.get(b))),
        }
    }
}

fn exec_program(instructions: &[Instruction], input: &[i32]) -> (i32, i32, i32, i32) {
    let mut alu = Alu::new();
    alu.input.extend(input);

    for ins in instructions {
        alu.exex(ins);
    }
    (alu.w, alu.x, alu.y, alu.z)
}

fn part1(instructions: &[Instruction]) -> i64 {
    0
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_execution() {
        assert_eq!(exec_program(&build(INPUT_TEST_1), &[22]), (0, -22, 0, 0));
        assert_eq!(exec_program(&build(INPUT_TEST_2), &[5, 9]), (0, 9, 0, 0));
        assert_eq!(exec_program(&build(INPUT_TEST_2), &[5, 15]), (0, 15, 0, 1));
        assert_eq!(exec_program(&build(INPUT_TEST_3), &[10]), (1, 0, 1, 0));
    }
}
