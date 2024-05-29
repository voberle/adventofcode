use std::io::{self, Read};

use fxhash::FxHashSet;

struct Rucksack {
    comp1: FxHashSet<char>,
    comp2: FxHashSet<char>,
}

impl From<&str> for Rucksack {
    fn from(line: &str) -> Self {
        let elts: Vec<char> = line.chars().collect();
        let mid = elts.len() / 2;
        let mut comp1 = FxHashSet::default();
        comp1.extend(&elts[0..mid]);
        let mut comp2 = FxHashSet::default();
        comp2.extend(&elts[mid..]);
        Self { comp1, comp2 }
    }
}

impl Rucksack {
    fn shared_item(&self) -> char {
        *self.comp1.intersection(&self.comp2).next().unwrap()
    }
}

fn build(input: &str) -> Vec<Rucksack> {
    input.lines().map(Into::into).collect()
}

fn priority(shared_item: char) -> u32 {
    let p = match shared_item {
        'a'..='z' => shared_item as u8 - b'a' + 1,
        'A'..='Z' => shared_item as u8 - b'A' + 27,
        _ => panic!("Invalid element"),
    };
    u32::from(p)
}

fn priorities_sum(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().map(|r| priority(r.shared_item())).sum()
}

fn part2(rucksacks: &[Rucksack]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rucksacks = build(&input);

    println!("Part 1: {}", priorities_sum(&rucksacks));
    println!("Part 2: {}", part2(&rucksacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(priorities_sum(&build(INPUT_TEST)), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
