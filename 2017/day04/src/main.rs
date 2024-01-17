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

fn is_valid_no_anagram(s: &[String]) -> bool {
    // If all the words have their letters in alphabetical order,
    // just checking if there are any unique ones tells us if there are anagrams.
    s.iter()
        .map(|w| w.as_bytes().iter().sorted().collect::<Vec<&u8>>())
        .unique()
        .count()
        == s.len()
}

fn valid_count(passphrases: &[Vec<String>], check_fn: fn(&[String]) -> bool) -> usize {
    passphrases.iter().filter(|p| check_fn(p)).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let passphrases = build(&input);

    println!("Part 1: {}", valid_count(&passphrases, is_valid));
    println!("Part 2: {}", valid_count(&passphrases, is_valid_no_anagram));
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
    fn test_is_valid_no_anagram() {
        assert!(is_valid_no_anagram(&build_phrase("abcde fghij")));
        assert!(!is_valid_no_anagram(&build_phrase("abcde xyz ecdab")));
        assert!(is_valid_no_anagram(&build_phrase("a ab abc abd abf abj")));
        assert!(is_valid_no_anagram(&build_phrase(
            "iiii oiii ooii oooi oooo"
        )));
        assert!(!is_valid_no_anagram(&build_phrase("oiii ioii iioi iiio")));
    }
}
