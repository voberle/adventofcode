use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
};

#[derive(Clone)]
struct Number(Vec<u32>);

impl Number {
    fn new(len: usize) -> Self {
        Self(vec![0; len])
    }

    fn to_number(&self) -> u32 {
        self.0.iter().fold(0, |acc, v| (acc << 1) + v)
    }
}

impl From<&str> for Number {
    fn from(s: &str) -> Self {
        Self(s.chars().map(|c| c.to_digit(10).unwrap()).collect())
    }
}

impl Index<usize> for Number {
    type Output = u32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Number {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

fn build(input: &str) -> Vec<Number> {
    input.lines().map(Number::from).collect()
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

    let mut gamma_rate_bytes = Number::new(len); // most common
    let mut epsilon_rate_bytes = Number::new(len); // least common
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

    gamma_rate_bytes.to_number() * epsilon_rate_bytes.to_number()
}

fn get_rating(numbers: &[Number], keep_zeros_fn: fn(usize, usize) -> bool) -> u32 {
    let mut numbers = numbers.to_vec();
    let mut current_bit_pos = 0;

    while numbers.len() > 1 {
        let (zeros, ones) = count_values_at(&numbers, current_bit_pos);
        if keep_zeros_fn(zeros, ones) {
            numbers.retain(|n| n[current_bit_pos] == 0);
        } else {
            numbers.retain(|n| n[current_bit_pos] == 1);
        }
        current_bit_pos += 1;
    }
    numbers.first().unwrap().to_number()
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
