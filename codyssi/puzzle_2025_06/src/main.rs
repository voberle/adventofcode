use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn uncorrupted_count(log: &[char]) -> usize {
    log.iter().filter(|c| c.is_ascii_alphabetic()).count()
}

fn value_of(c: char) -> Option<i64> {
    match c {
        'a'..='z' => Some(c as i64 - 'a' as i64 + 1),
        'A'..='Z' => Some(c as i64 - 'A' as i64 + 27),
        _ => None,
    }
}

fn uncorrupted_values_sum(log: &[char]) -> i64 {
    log.iter().map(|&c| value_of(c).unwrap_or(0)).sum()
}

fn total_value(log: &[char]) -> i64 {
    let mut values = Vec::new();
    for c in log {
        let value = if let Some(val) = value_of(*c) {
            val
        } else {
            // Previous char value.
            let mut val = *values.last().unwrap();
            val = val * 2 - 5;
            while val < 1 {
                val += 52;
            }
            while val > 52 {
                val -= 52;
            }
            val
        };
        values.push(value);
    }
    values.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let log = build(&input);

    println!("Part 1: {}", uncorrupted_count(&log));
    println!("Part 2: {}", uncorrupted_values_sum(&log));
    println!("Part 3: {}", total_value(&log));
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

    #[test]
    fn test_part2() {
        let log = build(&INPUT_TEST);
        assert_eq!(uncorrupted_values_sum(&log), 1742);
    }

    #[test]
    fn test_part3() {
        let log = build(&INPUT_TEST);
        assert_eq!(total_value(&log), 2708);
    }
}
