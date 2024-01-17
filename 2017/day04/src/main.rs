use std::io::{self, Read};

use itertools::Itertools;

fn build_phrase(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(std::borrow::ToOwned::to_owned)
        .collect()
}

fn build(input: &str) -> Vec<Vec<String>> {
    input.lines().map(build_phrase).collect()
}

fn is_valid(s: &[String]) -> bool {
    s.iter().unique().count() == s.len()
}

fn valid_passphrases_count(passphrases: &[Vec<String>]) -> usize {
    passphrases.iter().filter(|phrase| is_valid(phrase)).count()
}

fn part2(passphrases: &[Vec<String>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let passphrases = build(&input);

    println!("Part 1: {}", valid_passphrases_count(&passphrases));
    println!("Part 2: {}", part2(&passphrases));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid(&build_phrase("aa bb cc dd ee")));
        assert!(!is_valid(&build_phrase("aa bb cc dd aa")));
        assert!(is_valid(&build_phrase("aa bb cc dd aaa")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
