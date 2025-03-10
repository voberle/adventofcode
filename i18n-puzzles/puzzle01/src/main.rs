use std::io::{self, Read};

fn build(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

fn get_bytes_len(line: &str) -> usize {
    // String::len() returns the length in bytes.
    line.len()
}

fn get_chars_len(line: &str) -> usize {
    // Iterates over Unicode characters, so counting them gives us the number of characters.
    line.chars().count()
}

fn get_cost(line: &str) -> usize {
    const SMS_MAX: usize = 160;
    const TWITTER_MAX: usize = 140;

    let bytes = get_bytes_len(line);
    let chars = get_chars_len(line);
    // println!("bytes={bytes}; chars={chars}");

    if bytes <= SMS_MAX {
        if chars <= TWITTER_MAX { 13 } else { 11 }
    } else if chars <= TWITTER_MAX {
        7
    } else {
        0
    }
}

fn answer(lines: &[String]) -> usize {
    lines.iter().map(|line| get_cost(line)).sum()
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
        assert_eq!(answer(&build(INPUT_TEST)), 31);
    }
}
