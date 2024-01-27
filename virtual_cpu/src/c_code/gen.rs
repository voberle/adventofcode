use fxhash::FxHashSet;

use crate::{
    instruction::{Condition, Instruction},
    registers::Registers,
};

// Returns the list of register names used in the program.
fn get_register_names(instructions: &[Instruction]) -> FxHashSet<char> {
    let mut names: FxHashSet<char> = FxHashSet::default();
    for ins in instructions {
        names.extend(ins.get_register_names());
    }
    names
}

// Helper method to get the next alphabetical letter.
fn move_shift(data: &str, shift: usize) -> String {
    data.chars()
        .map(|c| (c as u8 + shift as u8) as char)
        .collect::<String>()
}

// Generate label name by taking the next letter in alphabetic order.
fn gen_free_label_name(next_label_name: &mut String) -> String {
    let free_name = next_label_name.clone();
    *next_label_name = move_shift(next_label_name, 1);
    free_name
}

// Generates C code for the instruction.
// labels contains the list of goto labels for the specified index
fn instruction_c_version(ins: &Instruction, ir: usize, labels: &[String]) -> String {
    match ins {
        Instruction::Set(x, y) => format!("{} = {}", x, y),
        Instruction::Add(x, y) => format!("{} += {}", x, y),
        Instruction::Sub(x, y) => format!("{} -= {}", x, y),
        Instruction::Mul(x, y) => format!("{} *= {}", x, y),
        Instruction::Mod(x, y) => format!("{} %= {}", x, y),
        Instruction::Div(x, y) => format!("{} /= {}", x, y),
        Instruction::JumpIf(cond, x, y, _) => {
            let index = (ir as i64 + y.get_integer()) as usize;
            match cond {
                Condition::NotZero => format!("if ({} != 0) goto {}", x, &labels[index]),
                Condition::GreaterThanZero => format!("if ({} > 0) goto {}", x, &labels[index]),
                Condition::True => format!("if (1) goto {}", &labels[index]),
                Condition::Even => format!("if ({} % 2 == 0) goto {}", x, &labels[index]),
                Condition::EqualOne => format!("if ({} == 1) goto {}", x, &labels[index]),
            }
        }
        Instruction::Nop => String::new(),
        _ => panic!("Unsupported instruction for C generation: {:?}", ins),
    }
}

// Transform the instructions into C.
// Save to a file and compile with `gcc -O3 main.c`.
#[allow(clippy::single_match)]
pub fn get_c_code(
    instructions: &[Instruction],
    initial_registers: &Registers<i64>,
    registers_to_print: &[char],
) -> String {
    let mut code = String::new();
    code += r"#include <stdio.h>

int main() {
";

    // Declare all the registers as variables.
    let registers = get_register_names(instructions);
    for r in registers {
        code += &format!("\tlong long {} = 0;\n", r);
    }
    code += "\n";

    // Initialize all registers that aren't 0
    initial_registers
        .regs
        .iter()
        .filter(|(_, val)| **val != 0)
        .for_each(|(name, val)| {
            code += &format!("\t{} = {};\n\n", name, val);
        });

    // We need to get all the labels before generating the code for each instruction.
    // We create the vector a bit bigger than needed to plan for jumping at the end of the program.
    let mut labels = vec![String::new(); instructions.len() + 2];
    let mut next_label_name = "A".to_string();
    for (i, ins) in instructions.iter().enumerate() {
        match ins {
            Instruction::JumpIf(_, _, y, _) => {
                let index = (i as i64 + y.get_integer()) as usize;
                labels[index] = gen_free_label_name(&mut next_label_name);
            }
            _ => {}
        }
    }

    for (i, label) in labels.iter().enumerate() {
        let mut line = String::new();
        if !label.is_empty() {
            line += &format!("{}: ", label);
        }
        line += "\t";
        if let Some(ins) = instructions.get(i) {
            line += &instruction_c_version(ins, i, &labels);
            line += ";";
        }
        line += "\n";

        code.push_str(&line);
    }

    for r in registers_to_print {
        code += &format!("\tprintf(\"%lli\\n\", {});\n", r);
    }
    code += "\treturn 0;\n";
    code += "}\n";

    code
}
