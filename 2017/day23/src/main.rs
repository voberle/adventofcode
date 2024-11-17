use std::{
    fmt, fs,
    io::{self, Read},
    process::Command,
};

use fxhash::{FxHashMap, FxHashSet};

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

    fn get_integer(&self) -> &T {
        if let IntChar::Integer(i) = self {
            i
        } else {
            panic!("Wanted an integer")
        }
    }
}

impl<T> fmt::Display for IntChar<T>
where
    T: std::str::FromStr,
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer(v) => v.to_string(),
                Self::Char(v) => v.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Set(char, IntChar<i64>),
    Sub(char, IntChar<i64>),
    Mul(char, IntChar<i64>),
    JumpNotZero(IntChar<i64>, IntChar<i64>),
    Nop,
}

impl Instruction {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        match *parts.first().unwrap() {
            "set" => Self::Set(char(parts[1]), IntChar::new(parts[2])),
            "sub" => Self::Sub(char(parts[1]), IntChar::new(parts[2])),
            "mul" => Self::Mul(char(parts[1]), IntChar::new(parts[2])),
            "jnz" => Self::JumpNotZero(IntChar::new(parts[1]), IntChar::new(parts[2])),
            "nop" => Self::Nop,
            _ => panic!("Unknown instruction"),
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::build).collect()
}

fn execute_common(ins: &Instruction, ir: &mut usize, regs: &mut Registers<i64>) {
    match ins {
        Instruction::Set(x, y) => {
            regs.set(*x, regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::Sub(x, y) => {
            regs.set(*x, regs.get(*x) - regs.get_ic(*y));
            *ir += 1;
        }
        Instruction::JumpNotZero(x, y) =>
        {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::cast_possible_wrap
            )]
            if regs.get_ic(*x) != 0 {
                *ir = (*ir as i64 + regs.get_ic(*y)) as usize;
            } else {
                *ir += 1;
            }
        }
        Instruction::Nop => *ir += 1,
        Instruction::Mul(..) => panic!("Wrong use of this function"),
    }
}

fn execute(instructions: &[Instruction], ir: &mut usize, regs: &mut Registers<i64>) -> bool {
    let ins = &instructions[*ir];
    match ins {
        Instruction::Mul(x, y) => {
            regs.set(*x, regs.get(*x) * regs.get_ic(*y));
            *ir += 1;
            return true;
        }
        _ => execute_common(ins, ir, regs),
    }
    false
}

fn mul_count(instructions: &[Instruction]) -> usize {
    let mut mul_invocations = 0;
    let mut regs = Registers::new();
    let mut ir = 0;
    while ir < instructions.len() {
        if execute(instructions, &mut ir, &mut regs) {
            mul_invocations += 1;
        }
    }
    mul_invocations
}

// Returns the list of register names used in the program.
fn get_register_names(instructions: &[Instruction]) -> FxHashSet<char> {
    let mut names: FxHashSet<char> = FxHashSet::default();
    for ins in instructions {
        match ins {
            Instruction::Set(x, y) | Instruction::Sub(x, y) | Instruction::Mul(x, y) => {
                names.insert(*x);
                if let IntChar::Char(c) = y {
                    names.insert(*c);
                }
            }
            Instruction::JumpNotZero(x, y) => {
                if let IntChar::Char(c) = x {
                    names.insert(*c);
                }
                if let IntChar::Char(c) = y {
                    names.insert(*c);
                }
            }
            Instruction::Nop => {}
        }
    }
    names
}

// Helper method to get the next alphabetical letter.
fn move_shift(data: &str, shift: usize) -> String {
    data.chars()
        .map(|c| (c as u8 + u8::try_from(shift).unwrap()) as char)
        .collect::<String>()
}

// Generate label name by taking the next letter in alphabetic order.
fn gen_free_label_name(next_label_name: &mut String) -> String {
    let free_name = next_label_name.clone();
    *next_label_name = move_shift(next_label_name, 1);
    free_name
}

// Transform the instructions into C.
// Save to a file and compile with `gcc -O3 main.c`.
#[allow(clippy::single_match)]
fn get_c_code(instructions: &[Instruction], use_optimization: bool) -> String {
    let mut code = String::new();
    code += r"#include <stdio.h>

int main() {
";

    // Declare all the registers as variables.
    let registers = get_register_names(instructions);
    for r in registers {
        code += &format!("\tlong long {r} = 0;\n");
    }
    code += "\n";

    code += "\ta = 1;\n\n";

    // We need to get all the labels before generating the code for each instruction.
    // We create the vector a bit bigger than needed to plan for jumping at the end of the program.
    let mut labels = vec![String::new(); instructions.len() + 2];
    let mut next_label_name = "A".to_string();
    for (i, ins) in instructions.iter().enumerate() {
        match ins {
            Instruction::JumpNotZero(_, y) => {
                #[allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    clippy::cast_possible_wrap
                )]
                let index = (i as i64 + y.get_integer()) as usize;

                labels[index] = gen_free_label_name(&mut next_label_name);
            }
            _ => {}
        }
    }

    for (i, label) in labels.iter().enumerate() {
        // Inner loop optimization (from Reddit)
        if use_optimization && (11..=19).contains(&i) {
            if i == 11 {
                code += "\t// Inner loop optimization
\tif (b % d == 0 && b / d != 1) {
\t\tf = 0;
\t}
";
            }
            continue;
        }
        let mut line = String::new();
        if !label.is_empty() {
            line += &format!("{label}: ");
        }
        line += "\t";
        if let Some(ins) = instructions.get(i) {
            line += &match ins {
                Instruction::Set(x, y) => format!("{x} = {y}"),
                Instruction::Sub(x, y) => format!("{x} -= {y}"),
                Instruction::Mul(x, y) => format!("{x} *= {y}"),
                Instruction::JumpNotZero(x, y) => {
                    #[allow(
                        clippy::cast_possible_truncation,
                        clippy::cast_sign_loss,
                        clippy::cast_possible_wrap
                    )]
                    let index = (i as i64 + y.get_integer()) as usize;

                    format!("if ({} != 0) goto {}", x, &labels[index])
                }
                Instruction::Nop => String::new(),
            };
            line += ";";
        }
        line += "\n";

        code.push_str(&line);
    }

    code += "\tprintf(\"%lli\\n\", h);\n";
    code += "\treturn 0;\n";
    code += "}\n";

    code
}

fn exec_c_code(instructions: &[Instruction]) -> String {
    // Generate the C code.
    let c_code = get_c_code(instructions, true);
    // Write the C file.
    fs::write("resources/main.c", c_code).expect("Unable to write file");
    // Compile it.
    let _ = Command::new("gcc")
        .arg("-O3")
        .arg("resources/main.c")
        .output();
    // Run it.
    let output = Command::new("./a.out").output().unwrap();
    // Clean the files.
    let _ = Command::new("rm").arg("./a.out").output();
    let _ = Command::new("rm").arg("resources/main.c").output();

    String::from_utf8(output.stdout).unwrap()
}

// This would be the brute-force Rust version, but it's way too slow.
#[allow(dead_code)]
fn value_of_h_at_end(instructions: &[Instruction]) -> i64 {
    let mut regs = Registers::new();
    regs.set('a', 1);
    let mut ir = 0;
    while ir < instructions.len() {
        // println!("{}: Exec {:?} for {:?}", ir, instructions[ir], regs);
        execute(instructions, &mut ir, &mut regs);
    }
    regs.get('h')
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", mul_count(&instructions));
    println!("Part 2: {}", exec_c_code(&instructions));
}
