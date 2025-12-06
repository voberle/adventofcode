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

impl Problem {
    fn new(operation: Operation, numbers: Vec<u64>) -> Self {
        Self { operation, numbers }
    }
}

fn build_part1(input: &str) -> Vec<Problem> {
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();
    let operations = lines.last().unwrap();

    (0..operations.len())
        .map(|col| {
            Problem::new(
                Operation::build(operations[col]),
                (0..lines.len() - 1)
                    .map(|row| lines[row][col].parse().unwrap())
                    .collect(),
            )
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

fn build_part2(input: &str) -> Vec<Problem> {
    // Convert each column into a String.
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let columns: Vec<String> = (0..lines.first().unwrap().len())
        .map(|col| (0..lines.len()).map(|row| lines[row][col]).collect())
        .collect();

    let mut problems: Vec<Problem> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();
    // Starting at the end is easier since the operation is in the first column.
    for column in columns.iter().rev() {
        if column.trim().is_empty() {
            continue;
        }
        match column.chars().last() {
            Some('+') => {
                numbers.push(column.trim_end_matches('+').trim().parse().unwrap());
                problems.push(Problem::new(Operation::Addition, numbers));
                numbers = Vec::new();
            }
            Some('*') => {
                numbers.push(column.trim_end_matches('*').trim().parse().unwrap());
                problems.push(Problem::new(Operation::Multiplication, numbers));
                numbers = Vec::new();
            }
            Some(_) => numbers.push(column.trim().parse().unwrap()),
            None => panic!("Impossible"),
        }
    }
    problems
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", grand_total(&build_part1(&input)));
    println!("Part 2: {}", grand_total(&build_part2(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(grand_total(&build_part1(INPUT_TEST)), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(grand_total(&build_part2(INPUT_TEST)), 3263827);
    }
}
