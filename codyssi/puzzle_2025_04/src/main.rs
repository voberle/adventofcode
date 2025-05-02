use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn memory_units_count(message: &[Vec<char>]) -> usize {
    message
        .iter()
        .flatten()
        .map(|&c| c as usize - 'A' as usize + 1)
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let message = build(&input);

    println!("Part 1: {}", memory_units_count(&message));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let message = build(&INPUT_TEST);
        assert_eq!(memory_units_count(&message), 1247);
    }
}
