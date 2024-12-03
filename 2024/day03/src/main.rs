use std::io::{self, Read};

use regex::Regex;

fn multiplication_result(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap())
        .sum()
}

fn better_result(input: &str) -> u64 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result: u64 = 0;
    let mut enabled = true;

    for m in re.find_iter(input).map(|m| m.as_str()) {
        if m == "do()" {
            enabled = true;
        } else if m == "don't()" {
            enabled = false;
        } else if enabled {
            assert!(m.starts_with("mul"));
            let mul_result: u64 = re_mul
                .captures_iter(m)
                .map(|c| c.extract())
                .map(|(_, [x, y])| x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap())
                .sum();
            result += mul_result;
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", multiplication_result(&input));
    println!("Part 2: {}", better_result(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(multiplication_result(INPUT_TEST_1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(better_result(INPUT_TEST_2), 48);
    }
}
