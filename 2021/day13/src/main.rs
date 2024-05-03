use std::io::{self, Read};

use itertools::Itertools;

enum FoldInstruction {
    X(u32),
    Y(u32),
}

fn build(input: &str) -> (Vec<(u32, u32)>, Vec<FoldInstruction>) {
    let mut it = input.lines();
    let mut coords = Vec::new();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        coords.push(
            line.split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }

    let mut instructions = Vec::new();
    for line in it {
        let ins = if let Some(x) = line.strip_prefix("fold along x=") {
            FoldInstruction::X(x.parse().unwrap())
        } else if let Some(y) = line.strip_prefix("fold along y=") {
            FoldInstruction::Y(y.parse().unwrap())
        } else {
            panic!("Invalid instruction")
        };
        instructions.push(ins);
    }

    (coords, instructions)
}

fn dots_after_folding(coords: &[(u32, u32)], instructions: &[FoldInstruction]) -> usize {
    0
}

fn part2(coords: &[(u32, u32)], instructions: &[FoldInstruction]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (coords, instructions) = build(&input);

    println!("Part 1: {}", dots_after_folding(&coords, &instructions));
    println!("Part 2: {}", part2(&coords, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (coords, instructions) = build(INPUT_TEST);
        assert_eq!(dots_after_folding(&coords, &instructions), 17);
    }

    #[test]
    fn test_part2() {
        let (coords, instructions) = build(INPUT_TEST);
        assert_eq!(part2(&coords, &instructions), 0);
    }
}
