use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn uncorrupted_count(log: &[char]) -> usize {
    log.iter().filter(|c| c.is_ascii_alphabetic()).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let log = build(&input);

    println!("Part 1: {}", uncorrupted_count(&log));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let log = build(&INPUT_TEST);
        assert_eq!(uncorrupted_count(&log), 59);
    }
}
