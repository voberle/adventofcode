use std::io::{self, Read};

#[derive(Clone)]
struct Number {
    nb: String,
    base: u32,
}

impl Number {
    fn build(line: &str) -> Self {
        let parts: Vec<_> = line.split_whitespace().collect();
        Self {
            nb: parts[0].to_string(),
            base: parts[1].parse().unwrap(),
        }
    }

    fn value(&self) -> u64 {
        // Can only be used in the range [2, 36].
        // u64::from_str_radix(&self.nb, self.base).unwrap()

        fn char_to_digit(c: char) -> u64 {
            match c {
                '0'..='9' => c as u64 - '0' as u64,
                'A'..='Z' => c as u64 - 'A' as u64 + 10,
                'a'..='z' => c as u64 - 'a' as u64 + 36,
                _ => panic!("Unsupported char"),
            }
        }

        self.nb
            .chars()
            .fold(0, |acc, c| acc * u64::from(self.base) + char_to_digit(c))
    }
}

fn build(input: &str) -> Vec<Number> {
    input.lines().map(Number::build).collect()
}

fn largest_number(numbers: &[Number]) -> u64 {
    numbers.iter().map(Number::value).max().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(&input);

    println!("Part 1: {}", largest_number(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_value() {
        let nb = Number {
            nb: "32IED4E6L4".to_string(),
            base: 22,
        };
        assert_eq!(nb.value(), 3778113247770);
    }

    #[test]
    fn test_part1() {
        let numbers = build(&INPUT_TEST);
        assert_eq!(largest_number(&numbers), 9047685997827);
    }
}
