use std::{
    fmt,
    io::{self, Read},
    mem::MaybeUninit,
};

use fxhash::FxHashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct Bitmask(String);

impl Bitmask {
    fn new(s: &str) -> Self {
        // We store a string, as it's much nicer to display than a vec.
        Self(s.to_string())
    }

    // Bitmask is 36 bits, so using 64 bits integers for the values.
    fn apply_to_value(&self, value: u64) -> u64 {
        // Going from most to least significant.
        (0..36)
            .rev()
            .map(|n| (value >> n) & 1)
            .zip(self.0.chars())
            .map(|(v, m)| match m {
                '0' => 0,
                '1' => 1,
                'X' => v,
                _ => panic!("Bug in apply_to_value"),
            })
            .fold(0, |acc, v| (acc << 1) + v)
    }

    fn apply_to_address(&self, address: u64) -> Bitmask {
        Bitmask(
            (0..36)
                .rev()
                .map(|n| (address >> n) & 1)
                .zip(self.0.chars())
                .map(|(a, m)| match m {
                    '0' => {
                        if a == 1 {
                            '1'
                        } else if a == 0 {
                            '0'
                        } else {
                            panic!("Bug in apply_to_address")
                        }
                    }
                    '1' => '1',
                    'X' => 'X',
                    _ => panic!("Bug in apply_to_address"),
                })
                .collect(),
        )
    }

    fn count_x(&self) -> usize {
        self.0.chars().filter(|&v| v == 'X').count()
    }
}

impl fmt::Display for Bitmask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Bitmask),
    Memory(u64, u64),
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

fn memory_sum_v1(instructions: &[Instruction]) -> u64 {
    let mut memory: FxHashMap<u64, u64> = FxHashMap::default();
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
                memory.insert(*address, mask.apply_to_value(*value));
            }
        }
    }
    memory.values().sum()
}

fn memory_sum_v2(instructions: &[Instruction]) -> u64 {
    let mut memory: FxHashMap<u64, u64> = FxHashMap::default();
    let mut bitmask = MaybeUninit::<&Bitmask>::uninit();
    for ins in instructions {
        match ins {
            Instruction::Mask(mask) => {
                bitmask.write(mask);
            }
            Instruction::Memory(address, value) => {
                let mask = unsafe { std::mem::MaybeUninit::assume_init(bitmask) };
                let masked_address = mask.apply_to_address(*address);

                // Get how many Xs are in the masked address and how many different addresses this makes.
                let x_count = masked_address.count_x();
                let addresses_count = 2_u64.pow(u32::try_from(x_count).unwrap());

                for i in 0..addresses_count {
                    // Replace the Xs.
                    let mut i_as_str: String = (0..x_count)
                        .map(|n| if (i >> n) & 1 == 1 { '1' } else { '0' })
                        .collect();
                    let bstr: String = masked_address
                        .0
                        .chars()
                        .map(|v| if v == 'X' { i_as_str.pop().unwrap() } else { v })
                        .collect();

                    // Convert to int.
                    let a = u64::from_str_radix(&bstr, 2).expect("Not a binary number!");

                    // Add to memory.
                    memory.insert(a, *value);
                }
            }
        }
    }
    memory.values().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", memory_sum_v1(&instructions));
    println!("Part 2: {}", memory_sum_v2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_apply_mask() {
        assert_eq!(
            Bitmask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply_to_value(11),
            73
        );
    }
    #[test]
    fn test_part1() {
        assert_eq!(memory_sum_v1(&build(INPUT_TEST_1)), 165);
    }

    #[test]
    fn test_part2() {
        assert_eq!(memory_sum_v2(&build(INPUT_TEST_2)), 208);
    }
}
