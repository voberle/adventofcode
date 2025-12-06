use std::io::{self, Read};

enum Operation {
    Addition,
    Multiplication,
}

impl Operation {
    fn build(s: &str) -> Self {
        match s {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => panic!("Invalid operation"),
        }
    }
}

struct Problem {
    operation: Operation,
    numbers: Vec<u64>,
}

fn build(input: &str) -> Vec<Problem> {
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();
    let operations = lines.last().unwrap();

    (0..operations.len())
        .map(|col| Problem {
            operation: Operation::build(operations[col]),
            numbers: (0..lines.len() - 1)
                .map(|row| lines[row][col].parse().unwrap())
                .collect(),
        })
        .collect()
}

fn grand_total(problems: &[Problem]) -> u64 {
    problems
        .iter()
        .map(|problem| match problem.operation {
            Operation::Addition => problem.numbers.iter().sum::<u64>(),
            Operation::Multiplication => problem.numbers.iter().product(),
        })
        .sum()
}

fn part2(problems: &[Problem]) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let problems = build(&input);

    println!("Part 1: {}", grand_total(&problems));
    println!("Part 2: {}", part2(&problems));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(grand_total(&build(INPUT_TEST)), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
