use std::io::{self, Read};

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

impl Pair {
    fn contains(&self, other: &Pair) -> bool {
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

impl SectionAssignment {
    fn fully_contained(&self) -> bool {
        self.p1.contains(&self.p2) || self.p2.contains(&self.p1)
    }
}

fn build(input: &str) -> Vec<SectionAssignment> {
    input.lines().map(Into::into).collect()
}

fn fully_contained_count(sections: &[SectionAssignment]) -> usize {
    sections.iter().filter(|sa| sa.fully_contained()).count()
}

fn part2(sections: &[SectionAssignment]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sections = build(&input);

    println!("Part 1: {}", fully_contained_count(&sections));
    println!("Part 2: {}", part2(&sections));
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
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
