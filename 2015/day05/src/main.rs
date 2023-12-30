use std::{
    collections::HashMap,
    io::{self, Read},
};

fn is_nice_first_rules(s: &str) -> bool {
    let vowels_count = s.chars().filter(|c| "aeiou".contains(*c)).count();
    // Converting to array of bytes to be able to use windows()
    let has_double_letter = s.as_bytes().windows(2).any(|p| p[0] == p[1]);
    let has_bad_string = ["ab", "cd", "pq", "xy"].iter().any(|i| s.contains(i));
    vowels_count >= 3 && has_double_letter && !has_bad_string
}

fn nice_strings_count(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_nice_first_rules(line))
        .count()
}

// This method is tricky. Regex could have worked if Rust supported back-reference.
fn contains_double_pair(s: &str) -> bool {
    if s.len() < 4 {
        // can't have double pair, and below algo doesn't support such short strings
        return false;
    }

    // Find all pairs
    let mut all_pairs_with_pos: Vec<_> = s.as_bytes().windows(2).enumerate().collect();
    all_pairs_with_pos.sort_by_key(|e| e.1);

    // Group all pairs in order to get all duplicates (seems there isn't a nicer way)
    let mut map = HashMap::new();
    for e in all_pairs_with_pos {
        map.entry(e.1).or_insert(vec![]).push(e);
    }

    map.values()
        .filter(|v| v.len() > 1) // only look at dupes
        .map(|dupe_list| {
            // a filter doesn't work here, trouble with borrow checker and mutable dupes
            let mut dupes = dupe_list.clone();
            dupes.sort_by_key(|d| d.0);
            dupes.first().unwrap().0.abs_diff(dupes.last().unwrap().0) > 1
        })
        .filter(|b| *b)
        .count()
        > 0
}

fn contains_repeating_letter(s: &str) -> bool {
    s.as_bytes().windows(3).any(|t| t[0] == t[2])
}

fn is_nice_second_rules(s: &str) -> bool {
    contains_double_pair(s) && contains_repeating_letter(s)
}

fn new_rules_nice_count(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_nice_second_rules(line))
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", nice_strings_count(&input));
    println!("Part 2: {}", new_rules_nice_count(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_double_pair() {
        assert!(contains_double_pair("xyxy"));
        assert!(contains_double_pair("aabcdefgaa"));
        assert!(!contains_double_pair("aaa"));

        assert!(contains_double_pair("qjhvhtzxzqqjkmpb"));
        assert!(contains_double_pair("xxyxx"));
        assert!(contains_double_pair("uurcxstgmygtbstg"));
        assert!(!contains_double_pair("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_contains_repeating_letter() {
        assert!(contains_repeating_letter("xyx"));
        assert!(contains_repeating_letter("abcdefeghi"));
        assert!(contains_repeating_letter("aaa"));

        assert!(contains_repeating_letter("qjhvhtzxzqqjkmpb"));
        assert!(contains_repeating_letter("xxyxx"));
        assert!(!contains_repeating_letter("uurcxstgmygtbstg"));
        assert!(contains_repeating_letter("ieodomkazucvgmuy"));
    }

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(nice_strings_count(INPUT_TEST_1), 2);
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part2() {
        assert_eq!(new_rules_nice_count(INPUT_TEST_2), 2);
    }
}
