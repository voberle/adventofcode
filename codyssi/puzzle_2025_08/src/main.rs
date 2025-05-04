use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn alphabetical_composition(file: &[Vec<char>]) -> usize {
    file.iter()
        .flatten()
        .filter(|c| c.is_ascii_alphabetic())
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let file = build(&input);

    println!("Part 1: {}", alphabetical_composition(&file));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let file = build(&INPUT_TEST);
        assert_eq!(alphabetical_composition(&file), 52);
    }
}
