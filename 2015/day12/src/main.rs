use std::io::{self, Read};

use regex::Regex;

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let numbers: Vec<i32> = re
        .find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    numbers.iter().sum()
}

fn part2(input: &str) -> i64 {
    0
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
