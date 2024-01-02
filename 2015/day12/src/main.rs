use std::io::{self, Read};

use regex::Regex;

fn sum_all_numbers(input: &str) -> i32 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let numbers: Vec<i32> = re
        .find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    numbers.iter().sum()
}

// Finds the position of the first closing bracket after the indicated position.
// We must be careful to skip in-between pairs. Meaning if we have "..{..}..}",
// we need to return the position of the last one.
fn closing_bracket_position(s: &str, pos: usize) -> Option<usize> {
    let mut i = pos;
    let mut open = 0;
    for c in s.chars().skip(i) {
        if c == '}' {
            if open == 0 {
                return Some(i);
            } else {
                open -= 1;
            }
        }
        if c == '{' {
            open += 1;
        }
        i += 1;
    }
    None
}

fn opening_bracket_position(s: &str, pos: usize) -> Option<usize> {
    let mut i = s.len() - pos - 1;
    let mut open = 0;
    for c in s.chars().rev().skip(i) {
        if c == '{' {
            if open == 0 {
                return Some(s.len() - i);
            } else {
                open -= 1;
            }
        }
        if c == '}' {
            open += 1;
        }
        i += 1;
    }
    None
}

fn sum_all_minus_red(input: &str) -> i32 {
    // We search the position of :"red", as the semi-colon indicates it's a red in a an object, not in an array.
    // Then for each red, we search for the opening bracket before and the closing one after, and remove the section in between.
    // Just need to be careful to get the correct bracket - done with the opening/closing_bracket_position functions.

    // The string with the red parts removed
    let mut new = input.to_string();

    const PATTERN: &str = r#":"red""#;
    while let Some(pos) = new.find(PATTERN) {
        if let Some(opening_bracket) = opening_bracket_position(&new, pos) {
            if let Some(closing_bracket) = closing_bracket_position(&new, pos) {
                // -1/+1 are to remove also the brackets
                let start = opening_bracket - 1;
                let end = closing_bracket + 1;
                new.replace_range(start..end, "");
            }
        }
    }
    sum_all_numbers(&new)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", sum_all_numbers(&input));
    println!("Part 2: {}", sum_all_minus_red(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(sum_all_numbers(r#"{"a":{"b":4},"c":-1}"#), 3);
    }

    #[test]
    fn test_bracket_position() {
        assert_eq!(opening_bracket_position("ab{..{..}.P.}...", 10), Some(3));
        assert_eq!(closing_bracket_position("abP..{..}...}...", 2), Some(12));
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_all_minus_red(r#"[1,2,3]"#), 6);
        assert_eq!(sum_all_minus_red(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(sum_all_minus_red(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(sum_all_minus_red(r#"[1,"red",5]"#), 6);
    }
}
