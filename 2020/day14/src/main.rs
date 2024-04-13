use std::{
    io::{self, Read},
    mem::MaybeUninit,
};

use fxhash::FxHashMap;
use regex::Regex;

#[derive(Debug)]
struct Bitmask(Vec<Option<u8>>);

impl Bitmask {
    fn new(s: &str) -> Self {
        let mask = s
            .chars()
            .map(|c| {
                if c == '1' {
                    Some(1)
                } else if c == '0' {
                    Some(0)
                } else if c == 'X' {
                    None
                } else {
                    panic!("Invalid bitmask")
                }
            })
            .collect();
        Self(mask)
    }

    // Bitmask is 36 bits, so using 64 bits integers for the values.
    fn apply(&self, value: u64) -> u64 {
        (0..36)
            .rev()
            .map(|n| (value >> n) & 1)
            .zip(self.0.iter())
            .map(|(v, m)| match m {
                Some(1) => 1,
                Some(0) => 0,
                None => v,
                _ => panic!("Bug in apply"),
            })
            .fold(0, |acc, v| (acc << 1) + v)
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Bitmask),
    Memory(usize, u64),
}

fn build(input: &str) -> Vec<Instruction> {
    let re_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            if let Some(mem_parts) = re_mem.captures(line) {
                Instruction::Memory(mem_parts[1].parse().unwrap(), mem_parts[2].parse().unwrap())
            } else {
                let bitmask = line.trim_start_matches("mask = ");
                Instruction::Mask(Bitmask::new(bitmask))
            }
        })
        .collect()
}

fn memory_sum(instructions: &[Instruction]) -> u64 {
    let mut memory: FxHashMap<usize, u64> = FxHashMap::default();
    // We want bitmask to be a reference to the mask stored in the instructions, and not make a copy.
    // So we need to declare it here, and initialize it later.
    let mut bitmask = MaybeUninit::<&Bitmask>::uninit();
    for ins in instructions {
        match ins {
            Instruction::Mask(mask) => {
                bitmask.write(mask);
            }
            Instruction::Memory(address, value) => {
                let mask = unsafe { std::mem::MaybeUninit::assume_init(bitmask) };
                memory.insert(*address, mask.apply(*value));
            }
        }
    }
    memory.values().sum()
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", memory_sum(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_apply_mask() {
        assert_eq!(
            Bitmask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(11),
            73
        );
    }
    #[test]
    fn test_part1() {
        assert_eq!(memory_sum(&build(INPUT_TEST)), 165);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
