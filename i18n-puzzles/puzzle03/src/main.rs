use std::io::{self, Read};

fn build(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

fn is_valid(password: &str) -> bool {
    // A length of at least 4 and at most 12.
    // We don't use len() as it would return the byte length, not the characters one.
    let len = password.chars().count();
    if !(4..=12).contains(&len) {
        return false;
    }

    // At least one digit.
    if password.chars().all(|c| !c.is_ascii_digit()) {
        // same as is_digit(10)
        return false;
    }

    // At least one uppercase letter (with or without accents, examples: A or Ż).
    if password.chars().all(|c| !c.is_uppercase()) {
        return false;
    }

    // At least one lowercase letter (with or without accents, examples: a or ŷ).
    if password.chars().all(|c| !c.is_lowercase()) {
        return false;
    }

    // At least one character that is outside the standard 7-bit ASCII character set (examples: Ű, ù or ř).
    if password.is_ascii() {
        return false;
    }

    true
}

fn answer(lines: &[String]) -> usize {
    // Valid passwords count
    lines.iter().filter(|password| is_valid(password)).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Answer: {}", answer(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(answer(&build(INPUT_TEST)), 2);
    }
}
