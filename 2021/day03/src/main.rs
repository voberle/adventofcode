use std::io::{self, Read};

#[derive(Clone)]
struct Number(Vec<u32>);

impl Number {
    fn new(s: &str) -> Self {
        Self(s.chars().map(|c| c.to_digit(10).unwrap()).collect())
    }
}

fn build(input: &str) -> Vec<Number> {
    input.lines().map(Number::new).collect()
}

// Count the number of 0s and 1s for the specified position.
fn count_values_at(numbers: &[Number], pos: usize) -> (usize, usize) {
    let mut zeros = 0;
    let mut ones = 0;
    for n in numbers {
        let b = n.0[pos];
        if b == 0 {
            zeros += 1;
        } else if b == 1 {
            ones += 1;
        }
    }
    (zeros, ones)
}

fn power_consumption(numbers: &[Number]) -> u32 {
    let len = numbers.first().unwrap().0.len();

    let mut gamma_rate_bytes = vec![0; len]; // most common
    let mut epsilon_rate_bytes = vec![0; len]; // least common
    for i in 0..len {
        let (zeros, ones) = count_values_at(numbers, i);
        if zeros > ones {
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

fn get_rating(numbers: &[Number], keep_zeros_fn: fn(usize, usize) -> bool) -> u32 {
    let mut numbers = numbers.to_vec();
    let mut current_bit_pos = 0;

    while numbers.len() > 1 {
        let (zeros, ones) = count_values_at(&numbers, current_bit_pos);
        if keep_zeros_fn(zeros, ones) {
            numbers.retain(|n| n.0[current_bit_pos] == 0);
        } else {
            numbers.retain(|n| n.0[current_bit_pos] == 1);
        }
        current_bit_pos += 1;
    }
    numbers[0].0.iter().fold(0, |acc, v| (acc << 1) + v)
}

fn life_support_rating(numbers: &[Number]) -> u32 {
    let oxygen_generator_rating = get_rating(numbers, |zeros, ones| zeros > ones);
    let co2_scrubber_rating = get_rating(numbers, |zeros, ones| zeros <= ones);

    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers = build(&input);

    println!("Part 1: {}", power_consumption(&numbers));
    println!("Part 2: {}", life_support_rating(&numbers));
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
        assert_eq!(life_support_rating(&build(INPUT_TEST)), 230);
    }
}
