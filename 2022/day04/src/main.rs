use std::{
    fmt::Display,
    io::{self, Read},
};

use itertools::Itertools;

struct Pair {
    a: u32,
    b: u32,
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let (a, b) = value
            .split('-')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { a, b }
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.a, self.b)
    }
}

impl Pair {
    fn contains(&self, val: u32) -> bool {
        self.a <= val && val <= self.b
    }

    fn contains_pair(&self, other: &Pair) -> bool {
        self.a <= other.a && other.b <= self.b
    }
}

struct SectionAssignment {
    p1: Pair,
    p2: Pair,
}

impl From<&str> for SectionAssignment {
    fn from(value: &str) -> Self {
        let (p1, p2) = value.split(',').map(Into::into).collect_tuple().unwrap();
        Self { p1, p2 }
    }
}

impl Display for SectionAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.p1, self.p2)
    }
}

impl SectionAssignment {
    fn fully_contained(&self) -> bool {
        self.p1.contains_pair(&self.p2) || self.p2.contains_pair(&self.p1)
    }

    fn overlap(&self) -> bool {
        self.p1.contains(self.p2.a)
            || self.p1.contains(self.p2.b)
            || self.p2.contains_pair(&self.p1)
    }
}

fn build(input: &str) -> Vec<SectionAssignment> {
    input.lines().map(Into::into).collect()
}

fn fully_contained_count(sections: &[SectionAssignment]) -> usize {
    sections.iter().filter(|sa| sa.fully_contained()).count()
}

fn overlap_count(sections: &[SectionAssignment]) -> usize {
    sections.iter().filter(|sa| sa.overlap()).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sections = build(&input);

    println!("Part 1: {}", fully_contained_count(&sections));
    println!("Part 2: {}", overlap_count(&sections));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(fully_contained_count(&build(INPUT_TEST)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(overlap_count(&build(INPUT_TEST)), 4);
    }
}
