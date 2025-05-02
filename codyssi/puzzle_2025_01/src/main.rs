use std::io::{self, Read};

fn build(input: &str) -> (Vec<u32>, Vec<char>) {
    let lines: Vec<_> = input.lines().collect();

    let numbers = lines
        .iter()
        .take(lines.len() - 1)
        .map(|n| n.parse().unwrap())
        .collect();
    let symbols = lines.last().unwrap().chars().collect();
    (numbers, symbols)
}

fn accumulate(acc: i64, number: i64, symbol: char) -> i64 {
    match symbol {
        '+' => acc + number,
        '-' => acc - number,
        _ => panic!("Unknown symbol"),
    }
}

fn actual_offset(numbers: &[u32], symbols: &[char]) -> i64 {
    numbers
        .iter()
        .skip(1)
        .zip(symbols.iter())
        .fold(i64::from(numbers[0]), |acc, (&n, &s)| {
            accumulate(acc, i64::from(n), s)
        })
}

fn new_offset(numbers: &[u32], symbols: &[char]) -> i64 {
    numbers
        .iter()
        .skip(1)
        .zip(symbols.iter().rev())
        .fold(i64::from(numbers[0]), |acc, (&n, &s)| {
            accumulate(acc, i64::from(n), s)
        })
}

fn final_offset(numbers: &[u32], symbols: &[char]) -> i64 {
    fn digits_to_number(digits: &[u32]) -> i64 {
        i64::from(digits[0] * 10 + digits[1])
    }

    numbers
        .chunks(2)
        .skip(1)
        .zip(symbols.iter().rev())
        .fold(digits_to_number(&numbers[0..2]), |acc, (digits, &s)| {
            accumulate(acc, digits_to_number(digits), s)
        })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (numbers, symbols) = build(&input);

    println!("Part 1: {}", actual_offset(&numbers, &symbols));
    println!("Part 2: {}", new_offset(&numbers, &symbols));
    println!("Part 3: {}", final_offset(&numbers, &symbols));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (numbers, symbols) = build(&INPUT_TEST);
        assert_eq!(actual_offset(&numbers, &symbols), 21);
    }

    #[test]
    fn test_part2() {
        let (numbers, symbols) = build(&INPUT_TEST);
        assert_eq!(new_offset(&numbers, &symbols), 23);
    }

    #[test]
    fn test_part3() {
        let (numbers, symbols) = build(&INPUT_TEST);
        assert_eq!(final_offset(&numbers, &symbols), 189);
    }
}
