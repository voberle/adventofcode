use std::io::{self, Read};

use itertools::Itertools;

enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn concat(a: u64, b: u64) -> u64 {
        let nb_digits_in_b = b.checked_ilog10().unwrap_or(0) + 1;
        a * 10_u64.pow(nb_digits_in_b) + b
    }

    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operation::Add => left + right,
            Operation::Mul => left * right,
            Operation::Concat => Self::concat(left, right),
        }
    }
}

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

    fn check(&self, operations_list: &[Operation]) -> bool {
        // In the real input, the biggest numbers list contains 12 values,
        // so trying all combinations for part 1 is 2^11 = 2048 possibilities.
        itertools::repeat_n(operations_list.iter(), self.numbers.len() - 1)
            .multi_cartesian_product()
            .any(|operations| {
                let result = operations
                    .iter()
                    .zip(self.numbers.iter().skip(1))
                    .fold(self.numbers[0], |acc, (op, nb)| op.apply(acc, *nb));
                result == self.test_value
            })
    }
}

fn build(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::build).collect()
}

fn total_calibration_result(equations: &[Equation], operations_list: &[Operation]) -> u64 {
    equations
        .iter()
        .filter(|eq| eq.check(operations_list))
        .map(|eq| eq.test_value)
        .sum()
}

fn result_simple(equations: &[Equation]) -> u64 {
    total_calibration_result(equations, &[Operation::Add, Operation::Mul])
}

fn result_with_concatenation(equations: &[Equation]) -> u64 {
    total_calibration_result(
        equations,
        &[Operation::Add, Operation::Mul, Operation::Concat],
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let equations = build(&input);

    println!("Part 1: {}", result_simple(&equations));
    println!("Part 2: {}", result_with_concatenation(&equations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    fn check_simple(s: &str) -> bool {
        let eq = Equation::build(s);
        eq.check(&[Operation::Add, Operation::Mul])
    }

    fn check_with_concatenation(s: &str) -> bool {
        let eq = Equation::build(s);
        eq.check(&[Operation::Add, Operation::Mul, Operation::Concat])
    }

    #[test]
    fn test_check() {
        assert!(check_simple("190: 10 19"));
        assert!(check_simple("3267: 81 40 27"));
        assert!(check_simple("292: 11 6 16 20"));

        assert!(!check_simple("83: 17 5"));
        assert!(!check_simple("156: 15 6"));
        assert!(!check_simple("7290: 6 8 6 15"));
        assert!(!check_simple("161011: 16 10 13"));
        assert!(!check_simple("192: 17 8 14"));
        assert!(!check_simple("21037: 9 7 18 13"));
    }

    #[test]
    fn test_concat() {
        assert_eq!(Operation::concat(12, 345), 12345);
    }

    #[test]
    fn test_check_with_concatenation() {
        assert!(check_with_concatenation("190: 10 19"));
        assert!(check_with_concatenation("3267: 81 40 27"));
        assert!(check_with_concatenation("292: 11 6 16 20"));
        assert!(check_with_concatenation("156: 15 6"));
        assert!(check_with_concatenation("7290: 6 8 6 15"));
        assert!(check_with_concatenation("192: 17 8 14"));

        assert!(!check_with_concatenation("83: 17 5"));
        assert!(!check_with_concatenation("161011: 16 10 13"));
        assert!(!check_with_concatenation("21037: 9 7 18 13"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(result_simple(&build(INPUT_TEST)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(result_with_concatenation(&build(INPUT_TEST)), 11387);
    }
}
