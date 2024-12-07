use std::io::{self, Read};

use itertools::Itertools;

enum Operation {
    Add,
    Mul,
    Concat,
}

fn concat(a: u64, b: u64) -> u64 {
    let nb_digits_in_b = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10_u64.pow(nb_digits_in_b) + b
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
                let result = operations.iter().zip(self.numbers.iter().skip(1)).fold(
                    self.numbers[0],
                    |acc, (op, nb)| match op {
                        Operation::Add => acc + *nb,
                        Operation::Mul => acc * *nb,
                        Operation::Concat => concat(acc, *nb),
                    },
                );
                result == self.test_value
            })
    }

    fn check_simple(&self) -> bool {
        self.check(&[Operation::Add, Operation::Mul])
    }

    fn check_with_concatenation(&self) -> bool {
        self.check(&[Operation::Add, Operation::Mul, Operation::Concat])
    }
}

fn build(input: &str) -> Vec<Equation> {
    input.lines().map(Equation::build).collect()
}

fn total_calibration_result(equations: &[Equation]) -> u64 {
    // println!("Max {}", equations.iter().map(|eq| eq.numbers.len()).max().unwrap());
    equations
        .iter()
        .filter(|eq| eq.check_simple())
        .map(|eq| eq.test_value)
        .sum()
}

fn result_with_concatenation(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|eq| eq.check_with_concatenation())
        .map(|eq| eq.test_value)
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let equations = build(&input);

    println!("Part 1: {}", total_calibration_result(&equations));
    println!("Part 2: {}", result_with_concatenation(&equations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_check() {
        assert!(Equation::build("190: 10 19").check_simple());
        assert!(Equation::build("3267: 81 40 27").check_simple());
        assert!(Equation::build("292: 11 6 16 20").check_simple());

        assert!(!Equation::build("83: 17 5").check_simple());
        assert!(!Equation::build("156: 15 6").check_simple());
        assert!(!Equation::build("7290: 6 8 6 15").check_simple());
        assert!(!Equation::build("161011: 16 10 13").check_simple());
        assert!(!Equation::build("192: 17 8 14").check_simple());
        assert!(!Equation::build("21037: 9 7 18 13").check_simple());
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 345), 12345);
    }

    #[test]
    fn test_check_with_concatenation() {
        assert!(Equation::build("190: 10 19").check_with_concatenation());
        assert!(Equation::build("3267: 81 40 27").check_with_concatenation());
        assert!(Equation::build("292: 11 6 16 20").check_with_concatenation());
        assert!(Equation::build("156: 15 6").check_with_concatenation());
        assert!(Equation::build("7290: 6 8 6 15").check_with_concatenation());
        assert!(Equation::build("192: 17 8 14").check_with_concatenation());

        assert!(!Equation::build("83: 17 5").check_with_concatenation());
        assert!(!Equation::build("161011: 16 10 13").check_with_concatenation());
        assert!(!Equation::build("21037: 9 7 18 13").check_with_concatenation());
    }

    #[test]
    fn test_part1() {
        assert_eq!(total_calibration_result(&build(INPUT_TEST)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(result_with_concatenation(&build(INPUT_TEST)), 11387);
    }
}
