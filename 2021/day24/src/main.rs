use std::{
    collections::VecDeque,
    fmt,
    io::{self, Read},
};

use fxhash::FxHashSet;

enum Variable {
    W,
    X,
    Y,
    Z,
    Val(i128),
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

    fn is_zero(&self) -> bool {
        matches!(self, Variable::Val(_))
    }

    fn get_value(&self) -> i128 {
        match self {
            Variable::Val(val) => *val,
            _ => panic!("Not a Val"),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Variable::W => "w".to_string(),
                Variable::X => "x".to_string(),
                Variable::Y => "y".to_string(),
                Variable::Z => "z".to_string(),
                Variable::Val(v) => v.to_string(),
            }
        )
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

    fn to_rust(&self) -> String {
        match self {
            Instruction::Inp(a) => format!("{a} = *it.next().unwrap();"),
            Instruction::Add(a, b) => format!("{a} += {b};"),
            Instruction::Mul(a, b) => {
                if b.is_zero() {
                    format!("{a} = 0;")
                } else {
                    format!("{a} *= {b};")
                }
            }
            Instruction::Div(a, b) => format!("{a} /= {b};"),
            Instruction::Mod(a, b) => format!("{a} %= {b};"),
            Instruction::Eql(a, b) => format!("{a} = if {a} == {b} {{ 1 }} else {{ 0 }};"),
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

struct Alu {
    w: i128,
    x: i128,
    y: i128,
    z: i128,
    input: VecDeque<i128>,
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

    fn get(&self, var: &Variable) -> i128 {
        match var {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
            Variable::Val(value) => *value,
        }
    }

    fn set(&mut self, var: &Variable, value: i128) {
        match var {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
            Variable::Val(_) => panic!("Cannot set an interger"),
        }
    }

    fn read_input(&mut self) -> i128 {
        self.input.pop_front().unwrap()
    }

    fn exec(&mut self, ins: &Instruction) {
        match ins {
            Instruction::Inp(a) => {
                let i = self.read_input();
                self.set(a, i);
            }
            Instruction::Add(a, b) => {
                self.set(a, self.get(a) + self.get(b));
            }
            Instruction::Mul(a, b) => {
                self.set(a, self.get(a) * self.get(b));
            }
            Instruction::Div(a, b) => {
                self.set(a, self.get(a) / self.get(b));
            }
            Instruction::Mod(a, b) => {
                self.set(a, self.get(a) % self.get(b));
            }
            Instruction::Eql(a, b) => {
                self.set(a, i128::from(self.get(a) == self.get(b)));
            }
        }
    }
}

#[allow(dead_code)]
fn exec_program(instructions: &[Instruction], input: &[i128]) -> (i128, i128, i128, i128) {
    let mut alu = Alu::new();
    alu.input.extend(input);

    for ins in instructions {
        alu.exec(ins);
    }
    (alu.w, alu.x, alu.y, alu.z)
}

// Convert the input into Rust code.
#[allow(dead_code)]
fn convert_program(instructions: &[Instruction]) -> String {
    let mut program = String::new();
    program.push_str(
        r"
fn compiled_input(input: &[i128]) -> i128 {
    let mut it = input.iter();

    let mut w: i128 = 0;
    let mut x: i128 = 0;
    let mut y: i128 = 0;
    let mut z: i128 = 0;
",
    );
    for ins in instructions {
        if matches!(ins, Instruction::Inp(_)) {
            program.push('\n');
        }
        program.push_str("    ");
        program.push_str(&ins.to_rust());
        program.push('\n');
    }
    program.push_str(
        r"
    z
}
",
    );
    program
}

// The input is composed of 14 parts that are almost identical
// except for 3 constants in each part.
// This function extracts those constants.
fn extract_constants(instructions: &[Instruction]) -> Vec<(i128, i128, i128)> {
    instructions
        .chunks(18)
        .map(|part| {
            let v1 = match &part[4] {
                Instruction::Div(_, b) => b.get_value(),
                _ => panic!("Invalid instruction"),
            };
            let v2 = match &part[5] {
                Instruction::Add(_, b) => b.get_value(),
                _ => panic!("Invalid instruction"),
            };
            let v3 = match &part[15] {
                Instruction::Add(_, b) => b.get_value(),
                _ => panic!("Invalid instruction"),
            };
            (v1, v2, v3)
        })
        .collect()
}

// From 2018/day14
fn get_digits(n: u64) -> Vec<i128> {
    fn inner(n: u64, xs: &mut Vec<i128>) {
        if n >= 10 {
            inner(n / 10, xs);
        }
        xs.push((n % 10).into());
    }
    let mut xs = Vec::new();
    inner(n, &mut xs);
    xs
}

// Checks for a bunch of serial numbers that the interpreted input and the
// optimized Rust version produce the same result.
#[allow(dead_code)]
fn validate_compiled(instructions: &[Instruction], from: u64, count: usize) {
    let mut n: u64 = from;

    for _ in 0..count {
        let sn = get_digits(n);
        if sn.contains(&0) {
            continue;
        }

        let z_ins = exec_program(instructions, &sn).3;
        let z_opt = optimized_program(instructions, &sn);
        // println!("{}\t{:?}", z_ins, sn);

        assert_eq!(z_ins, z_opt);

        n += 1;
    }
}

// Rust version of the individual part of the input.
// v1,v2,v3 are the constants extracted by extract_constants.
fn digit_fn(w: i128, z: i128, v1: i128, v2: i128, v3: i128) -> i128 {
    if z % 26 + v2 == w {
        z / v1
    } else {
        z / v1 * 26 + w + v3
    }
}

// Rust version of the input.
fn optimized_program(instructions: &[Instruction], input: &[i128]) -> i128 {
    let mut z: i128 = 0;
    let constants = extract_constants(instructions);

    for (i, (v1, v2, v3)) in constants.iter().copied().enumerate() {
        z = digit_fn(input[i], z, v1, v2, v3);
    }
    z
}

// Returns a list of which digits can be used at each 14 steps so that
// we get 0 at the end.
fn find_all_applicable_digits(instructions: &[Instruction]) -> [FxHashSet<i128>; 14] {
    let constants = extract_constants(instructions);

    // Part 1: Go through the 14 parts in order,
    // and save all possible z's that can generated for each part.
    let mut zs: Vec<FxHashSet<i128>> = Vec::new();
    zs.push(FxHashSet::default());
    zs[0].insert(0);

    for (i, (v1, v2, v3)) in constants.iter().copied().enumerate() {
        let mut tzs: FxHashSet<i128> = FxHashSet::default();
        for z in &zs[i] {
            for w in 1..=9 {
                tzs.insert(digit_fn(w, *z, v1, v2, v3));
            }
        }
        // println!("Z count: {}; Contains 0: {}", tzs.len(), tzs.contains(&0));
        zs.push(tzs);
    }

    // The last of zs contains all the Z produced by the last calculation.
    // It must contain 0.
    assert!(zs.last().unwrap().contains(&0));

    // Part 2: Start from the end and select the z's that can produce 0 at the end.
    // Save the w's that work.
    let mut applicable_digits: [FxHashSet<i128>; 14] = Default::default();

    let mut expected_zs: FxHashSet<i128> = FxHashSet::default();
    expected_zs.insert(0);
    for (i, (v1, v2, v3)) in constants.iter().rev().copied().enumerate() {
        // println!("Expected Z count: {}", expected_zs.len());
        let tzs = &zs[13 - i];

        let mut next_expected_zs: FxHashSet<i128> = FxHashSet::default();
        for exp_z in &expected_zs {
            for z in tzs {
                for w in 1..=9 {
                    if digit_fn(w, *z, v1, v2, v3) == *exp_z {
                        // println!("{}: {},{}", i, w, z);
                        applicable_digits[13 - i].insert(w);
                        next_expected_zs.insert(*z);
                    }
                }
            }
        }
        std::mem::swap(&mut expected_zs, &mut next_expected_zs);
    }
    assert!(expected_zs.contains(&0));

    applicable_digits
}

fn largest_accepted_number(applicable_digits: &[FxHashSet<i128>]) -> i128 {
    applicable_digits
        .iter()
        .map(|digits| digits.iter().max().unwrap())
        .fold(0, |acc, v| acc * 10 + v)
}

fn smallest_accepted_number(applicable_digits: &[FxHashSet<i128>]) -> i128 {
    applicable_digits
        .iter()
        .map(|digits| digits.iter().min().unwrap())
        .fold(0, |acc, v| acc * 10 + v)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    // println!("{}", convert_program(&instructions));

    // validate_compiled(&instructions, 52_399_999_999_999, 10_000);

    let applicable_digits = find_all_applicable_digits(&instructions);

    println!("Part 1: {}", largest_accepted_number(&applicable_digits));
    println!("Part 2: {}", smallest_accepted_number(&applicable_digits));
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
