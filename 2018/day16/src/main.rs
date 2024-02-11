use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
    usize,
};

#[derive(Debug, PartialEq, Clone)]
struct Registers(Vec<u32>);

impl Registers {
    fn new() -> Self {
        Self(vec![0; 4])
    }
}

impl From<Vec<u32>> for Registers {
    fn from(item: Vec<u32>) -> Self {
        assert_eq!(item.len(), 4);
        Self(item)
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

struct Sample {
    before: Registers,
    instruction: Vec<u32>,
    after: Registers,
}

impl Sample {
    fn opcode(&self) -> u32 {
        self.instruction[0]
    }

    fn a(&self) -> u32 {
        self.instruction[1]
    }

    fn b(&self) -> u32 {
        self.instruction[2]
    }

    fn c(&self) -> u32 {
        self.instruction[3]
    }
}

fn regs_list(s: &str) -> Registers {
    let start = s.chars().position(|c| c == '[').unwrap() + 1;
    Registers::from(
        s[start..s.len() - 1]
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect::<Vec<u32>>(),
    )
}

fn build(input: &str) -> Vec<Sample> {
    let mut samples = Vec::new();
    let mut it = input.lines();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        samples.push(Sample {
            before: regs_list(line),
            instruction: it
                .next()
                .unwrap()
                .split(' ')
                .map(|i| i.parse().unwrap())
                .collect(),
            after: regs_list(it.next().unwrap()),
        });
        it.next();
    }
    samples
}

type InstructionFn = fn(&mut Registers, u32, u32, u32);

const INSTRUCTIONS: [InstructionFn; 16] = [
    |regs, a, b, c| regs[c] = regs[a] + regs[b],      // addr
    |regs, a, b, c| regs[c] = regs[a] + b,            // addi
    |regs, a, b, c| regs[c] = regs[a] * regs[b],      // mulr
    |regs, a, b, c| regs[c] = regs[a] * b,            // muli
    |regs, a, b, c| regs[c] = regs[a] & regs[b],      // banr
    |regs, a, b, c| regs[c] = regs[a] & b,            // bani
    |regs, a, b, c| regs[c] = regs[a] | regs[b],      // borr
    |regs, a, b, c| regs[c] = regs[a] | b,            // bori
    |regs, a, _, c| regs[c] = regs[a],                // setr
    |regs, a, _, c| regs[c] = a,                      // seti
    |regs, a, b, c| regs[c] = u32::from(a > regs[b]), // gtit
    |regs, a, b, c| regs[c] = u32::from(regs[a] > b), // gtri
    |regs, a, b, c| regs[c] = u32::from(regs[a] > regs[b]), // gtrr
    |regs, a, b, c| regs[c] = u32::from(a == regs[b]), // eqit
    |regs, a, b, c| regs[c] = u32::from(regs[a] == b), // eqri
    |regs, a, b, c| regs[c] = u32::from(regs[a] == regs[b]), // eqrr
];

fn is_sample_matching_instruction(sample: &Sample, instruction_fn: InstructionFn) -> bool {
    let mut regs = sample.before.clone();
    instruction_fn(&mut regs, sample.a(), sample.b(), sample.c());
    regs == sample.after
}

fn count_matching_instructions(sample: &Sample) -> usize {
    INSTRUCTIONS
        .iter()
        .filter(|ins_fn| is_sample_matching_instruction(sample, **ins_fn))
        .count()
}

fn samples_matching_3_or_more(samples: &[Sample]) -> usize {
    samples
        .iter()
        .filter(|s| count_matching_instructions(s) >= 3)
        .count()
}

fn part2(samples: &[Sample]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let samples = build(&input);

    println!("Part 1: {}", samples_matching_3_or_more(&samples));
    println!("Part 2: {}", part2(&samples));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_sample_matching_instruction() {
        let sample = &build(INPUT_TEST)[0];
        assert!(!is_sample_matching_instruction(sample, INSTRUCTIONS[0]));
        assert!(is_sample_matching_instruction(sample, INSTRUCTIONS[1]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(samples_matching_3_or_more(&build(INPUT_TEST)), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
