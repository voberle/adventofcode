use std::io::{self, Read};

fn total_string_code(input: &str) -> usize {
    input.len() - input.chars().filter(|&c| c == '\n').count()
}

fn total_string_chars(input: &str) -> usize {
    let mut count = 0;
    let mut iter = input.chars().peekable();
    while let Some(c) = iter.next() {
        if c == '\n' || c == '"' {
            continue;
        } else if c == '\\' {
            if let Some(&n) = iter.peek() {
                if n == '\\' || n == '"' {
                    iter.next();
                } else if n == 'x' {
                    iter.next();
                    iter.next();
                    iter.next();
                }
            }
        }
        count += 1;
    }
    count
}

fn newly_encoded_string_code_size(line: &str) -> usize {
    // 2 is for the quotes at the beginning and end of each line
    line.len() + 2 + line.chars().filter(|&c| c == '\\' || c == '"').count()
}

fn total_newly_encoded_string_code(input: &str) -> usize {
    input
        .lines()
        .map(newly_encoded_string_code_size)
        .sum()
}

fn part1(input: &str) -> usize {
    total_string_code(input) - total_string_chars(input)
}

fn part2(input: &str) -> usize {
    total_newly_encoded_string_code(input) - total_string_code(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_total_string_chars() {
        assert_eq!(total_string_chars(r#""""#), 0);
        assert_eq!(total_string_chars(r#""abc""#), 3);
        assert_eq!(total_string_chars(r#""aaa\"aaa""#), 7);
        assert_eq!(total_string_chars(r#""\x27""#), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(total_string_code(INPUT_TEST), 23);
        assert_eq!(total_string_chars(INPUT_TEST), 11);
        assert_eq!(part1(INPUT_TEST), 12);
    }

    #[test]
    fn test_total_newly_encoded_string_code() {
        assert_eq!(newly_encoded_string_code_size(r#""""#), 6);
        assert_eq!(newly_encoded_string_code_size(r#""abc""#), 9);
        assert_eq!(newly_encoded_string_code_size(r#""aaa\"aaa""#), 16);
        assert_eq!(newly_encoded_string_code_size(r#""\x27""#), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(total_newly_encoded_string_code(INPUT_TEST), 42);
        assert_eq!(part2(INPUT_TEST), 19);
    }
}
