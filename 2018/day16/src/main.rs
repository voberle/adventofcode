use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
};

use fxhash::FxHashSet;

#[derive(Debug, PartialEq, Clone)]
struct Registers(Vec<u32>);

impl Registers {
    fn new() -> Self {
        Self(vec![0; 4])
    }

    fn build(s: &str) -> Self {
        let start = s.chars().position(|c| c == '[').unwrap() + 1;
        Self(
            s[start..s.len() - 1]
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<u32>>(),
        )
    }
}

impl Index<u32> for Registers {
    type Output = u32;
    fn index(&self, reg: u32) -> &Self::Output {
        assert!((0..4).contains(&reg));
        &self.0[reg as usize]
    }
}

impl IndexMut<u32> for Registers {
    fn index_mut(&mut self, reg: u32) -> &mut Self::Output {
        assert!((0..4).contains(&reg));
        &mut self.0[reg as usize]
    }
}

#[derive(Debug)]
struct Instruction(Vec<u32>);

impl Instruction {
    fn build(s: &str) -> Self {
        Self(s.split(' ').map(|i| i.parse().unwrap()).collect())
    }

    fn opcode(&self) -> u32 {
        self.0[0]
    }

    fn a(&self) -> u32 {
        self.0[1]
    }

    fn b(&self) -> u32 {
        self.0[2]
    }

    fn c(&self) -> u32 {
        self.0[3]
    }
}

#[derive(Debug)]
struct Sample {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

fn build(input: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let mut samples = Vec::new();
    let mut it = input.lines();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        samples.push(Sample {
            before: Registers::build(line),
            instruction: Instruction::build(it.next().unwrap()),
            after: Registers::build(it.next().unwrap()),
        });
        it.next();
    }

    let mut test_program: Vec<Instruction> = Vec::new();
    for line in it {
        if line.is_empty() {
            continue;
        }
        test_program.push(Instruction::build(line));
    }

    (samples, test_program)
}

type InstructionFn = fn(&mut Registers, u32, u32, u32);

#[rustfmt::skip]
const INSTRUCTIONS: [(&str, InstructionFn); 16] = [
    ("addr", |regs, a, b, c| regs[c] = regs[a] + regs[b]),
    ("addi", |regs, a, b, c| regs[c] = regs[a] + b),
    ("mulr", |regs, a, b, c| regs[c] = regs[a] * regs[b]),
    ("muli", |regs, a, b, c| regs[c] = regs[a] * b),
    ("banr", |regs, a, b, c| regs[c] = regs[a] & regs[b]),
    ("bani", |regs, a, b, c| regs[c] = regs[a] & b),
    ("borr", |regs, a, b, c| regs[c] = regs[a] | regs[b]),
    ("bori", |regs, a, b, c| regs[c] = regs[a] | b),
    ("setr", |regs, a, _, c| regs[c] = regs[a]),
    ("seti", |regs, a, _, c| regs[c] = a),
    ("gtir", |regs, a, b, c| regs[c] = u32::from(a > regs[b])),
    ("gtri", |regs, a, b, c| regs[c] = u32::from(regs[a] > b)),
    ("gtrr", |regs, a, b, c| regs[c] = u32::from(regs[a] > regs[b])),
    ("eqit", |regs, a, b, c| regs[c] = u32::from(a == regs[b])),
    ("eqri", |regs, a, b, c| regs[c] = u32::from(regs[a] == b)),
    ("eqrr", |regs, a, b, c| regs[c] = u32::from(regs[a] == regs[b])),
];

fn is_sample_matching_instruction(sample: &Sample, instruction_fn: InstructionFn) -> bool {
    let mut regs = sample.before.clone();
    instruction_fn(
        &mut regs,
        sample.instruction.a(),
        sample.instruction.b(),
        sample.instruction.c(),
    );
    regs == sample.after
}

fn matching_instructions(sample: &Sample) -> Vec<usize> {
    INSTRUCTIONS
        .iter()
        .enumerate()
        .filter_map(|(i, ins_fn)| {
            if is_sample_matching_instruction(sample, ins_fn.1) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn samples_matching_3_or_more(samples: &[Sample]) -> usize {
    samples
        .iter()
        .filter(|s| matching_instructions(s).len() >= 3)
        .count()
}

// Returns a vector where indexes are the opcodes, and value the index in the INSTRUCTIONS list.
fn find_opcodes(samples: &[Sample]) -> Vec<usize> {
    // Vector index is the opcode. The value are the index of the possible instructions matching.
    let mut opcodes2ins = vec![FxHashSet::default(); 16];

    for sample in samples {
        let matching_ins_indexes = matching_instructions(sample);

        let set = matching_ins_indexes
            .iter()
            .copied()
            .collect::<FxHashSet<_>>();
        let o = sample.instruction.opcode() as usize;
        if opcodes2ins[o].is_empty() {
            opcodes2ins[o] = set;
        } else {
            opcodes2ins[o] = opcodes2ins[o].intersection(&set).copied().collect();
        }
    }

    // Algo to reduce each set of instruction candidates to one only.
    while opcodes2ins.iter().any(|ins_list| ins_list.len() > 1) {
        // Get all opcodes for which we know the instruction
        let opcodes_we_know: Vec<_> = opcodes2ins
            .iter()
            .enumerate()
            .filter_map(|(o, ins_list)| if ins_list.len() == 1 { Some(o) } else { None })
            .collect();

        // Remove that opcode from the other lists
        for o in opcodes_we_know {
            assert_eq!(opcodes2ins[o].len(), 1);
            let ins_idx = *opcodes2ins[o].iter().next().unwrap();
            for (i, set) in opcodes2ins.iter_mut().enumerate() {
                if i != o {
                    set.remove(&ins_idx);
                }
            }
        }
    }

    opcodes2ins
        .iter()
        .map(|sets| *sets.iter().next().unwrap())
        .collect()
}

// Verifies the opcodes mapping we found work for all the samples.
fn verify_opcodes(samples: &[Sample], opcodes_to_inst_index: &[usize]) {
    for sample in samples {
        let opcode = sample.instruction.opcode() as usize;
        let ins_fn = INSTRUCTIONS[opcodes_to_inst_index[opcode]].1;
        let mut regs = sample.before.clone();
        ins_fn(
            &mut regs,
            sample.instruction.a(),
            sample.instruction.b(),
            sample.instruction.c(),
        );
        assert_eq!(regs, sample.after);
    }
}

fn reg0_at_end(samples: &[Sample], test_program: &[Instruction]) -> u32 {
    let opcodes_to_inst_index = find_opcodes(samples);
    verify_opcodes(samples, &opcodes_to_inst_index);

    let mut regs = Registers::new();
    for ins in test_program {
        let ins_idx = opcodes_to_inst_index[ins.opcode() as usize];
        let ins_fn = INSTRUCTIONS[ins_idx].1;

        // println!(
        //     "{}: [{}] {}, {} {} {}",
        //     ins.opcode(),
        //     ins_idx,
        //     INSTRUCTIONS[ins_idx].0,
        //     ins.a(),
        //     ins.b(),
        //     ins.c()
        // );

        ins_fn(&mut regs, ins.a(), ins.b(), ins.c());
        // println!("{:?}", regs);
    }
    regs[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (samples, test_program) = build(&input);

    println!("Part 1: {}", samples_matching_3_or_more(&samples));
    println!("Part 2: {}", reg0_at_end(&samples, &test_program));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_sample_matching_instruction() {
        let sample = &build(INPUT_TEST).0[0];
        assert!(!is_sample_matching_instruction(sample, INSTRUCTIONS[0].1));
        assert!(is_sample_matching_instruction(sample, INSTRUCTIONS[1].1));
    }

    #[test]
    fn test_part1() {
        let (samples, _) = build(INPUT_TEST);
        assert_eq!(samples_matching_3_or_more(&samples), 1);
    }
}
