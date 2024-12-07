use std::io::{self, Read};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn build(input: &str) -> Self {
        let (p1, p2) = input.split(": ").collect_tuple().unwrap();
        Self {
            test_value: p1.parse().unwrap(),
            numbers: p2
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        }
    }

    fn check(&self) -> bool {
        use Operation::{Add, Mul};
        // In the real input, the biggest numbers list contains 12 values,
        // so trying all combinations is 2^11 = 2048 possibilities.
        itertools::repeat_n([Add, Mul].iter(), self.numbers.len() - 1)
            .multi_cartesian_product()
            .any(|operations| {
                let result = operations.iter().zip(self.numbers.iter().skip(1)).fold(
                    self.numbers[0],
                    |acc, (op, nb)| match op {
                        Add => acc + *nb,
                        Mul => acc * *nb,
                    },
                );
                result == self.test_value
            })
    }
}

fn build(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::build).collect()
}

fn total_calibration_result(equations: &[Equation]) -> u64 {
    // println!("Max {}", equations.iter().map(|eq| eq.numbers.len()).max().unwrap());
    equations
        .iter()
        .filter(|eq| eq.check())
        .map(|eq| eq.test_value)
        .sum()
}

fn part2(equations: &[Equation]) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let equations = build(&input);

    println!("Part 1: {}", total_calibration_result(&equations));
    println!("Part 2: {}", part2(&equations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_check() {
        assert!(Equation::build("190: 10 19").check());
        assert!(Equation::build("3267: 81 40 27").check());
        assert!(Equation::build("292: 11 6 16 20").check());

        assert!(!Equation::build("83: 17 5").check());
        assert!(!Equation::build("156: 15 6").check());
        assert!(!Equation::build("7290: 6 8 6 15").check());
        assert!(!Equation::build("161011: 16 10 13").check());
        assert!(!Equation::build("192: 17 8 14").check());
        assert!(!Equation::build("21037: 9 7 18 13").check());
    }

    #[test]
    fn test_part1() {
        assert_eq!(total_calibration_result(&build(INPUT_TEST)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
