use std::io::{self, Read};

struct Number(Vec<u32>);

impl Number {
    fn new(s: &str) -> Self {
        Self(s.chars().map(|c| c.to_digit(10).unwrap()).collect())
    }
}

fn build(input: &str) -> Vec<Number> {
    input.lines().map(Number::new).collect()
}

fn power_consumption(numbers: &[Number]) -> u32 {
    let len = numbers.first().unwrap().0.len();

    // Count the number of 0s and 1s for each position.
    let mut zeros = vec![0; len];
    let mut ones = vec![0; len];
    for n in numbers {
        for (i, b) in n.0.iter().enumerate() {
            if *b == 0 {
                zeros[i] += 1;
            } else if *b == 1 {
                ones[i] += 1;
            }
        }
    }

    let mut gamma_rate_bytes = vec![0; len]; // most common
    let mut epsilon_rate_bytes = vec![0; len]; // least common
    for i in 0..len {
        if zeros[i] > ones[i] {
            gamma_rate_bytes[i] = 0;
            epsilon_rate_bytes[i] = 1;
        } else {
            gamma_rate_bytes[i] = 1;
            epsilon_rate_bytes[i] = 0;
        }
    }

    let gamma_rate = gamma_rate_bytes.iter().fold(0, |acc, v| (acc << 1) + v);
    let epsilon_rate = epsilon_rate_bytes.iter().fold(0, |acc, v| (acc << 1) + v);

    gamma_rate * epsilon_rate
}

fn part2(numbers: &[Number]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(&input);

    println!("Part 1: {}", power_consumption(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(power_consumption(&build(INPUT_TEST)), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
