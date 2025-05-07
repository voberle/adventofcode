use std::io::{self, Read};

#[derive(Clone)]
struct Number {
    nb: String,
    base: u64,
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
            .fold(0, |acc, c| acc * self.base + char_to_digit(c))
    }
}

#[allow(clippy::cast_possible_truncation)]
fn to_base(n: u64, base: u64) -> String {
    #[rustfmt::skip]
    const CHARS: &[char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '!', '@', '#', '$', '%', '^',
    ];

    // We use a Vec instead of a String, as we need to reverse it at the end.
    let mut result = Vec::new();

    let mut n = n;
    while n != 0 {
        let rem = n % base;
        result.push(CHARS[rem as usize]);
        n /= base;
    }
    result.reverse();

    result.iter().collect()
}

fn build(input: &str) -> Vec<Number> {
    input.lines().map(Number::build).collect()
}

fn largest_number(numbers: &[Number]) -> u64 {
    numbers.iter().map(Number::value).max().unwrap()
}

fn sum_base68(numbers: &[Number]) -> String {
    let sum = numbers.iter().map(Number::value).sum();
    to_base(sum, 68)
}

fn smallest_base_for_4_chars(numbers: &[Number]) -> u64 {
    let sum = numbers.iter().map(Number::value).sum();

    // Largest number we can represent in base 2 with at most 4 chars:
    //   15 = 2 ^ 4 - 1
    // Largest number we can represent in base b with at most 4 chars:
    //   b ^ 4 - 1
    // So we need to find the first such number bigger than our sum.
    let mut b: u64 = 1;
    while b.pow(4) - 1 < sum {
        b += 1;
    }
    b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(&input);

    println!("Part 1: {}", largest_number(&numbers));
    println!("Part 2: {}", sum_base68(&numbers));
    println!("Part 3: {}", smallest_base_for_4_chars(&numbers));
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

    #[test]
    fn test_part2() {
        let numbers = build(&INPUT_TEST);
        assert_eq!(sum_base68(&numbers), "4iWAbo%6");
    }

    #[test]
    fn test_part3() {
        let numbers = build(&INPUT_TEST);
        assert_eq!(smallest_base_for_4_chars(&numbers), 2366);
    }
}
