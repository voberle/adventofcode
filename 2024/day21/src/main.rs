use std::io::{self, Read};

use model::NumKey;

mod model;

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn complexities_sum(codes: &[Vec<char>]) -> i64 {
    0
}

fn part2(codes: &[Vec<char>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let codes = build(&input);

    // model::print_numeric_keypad(&[]);

    for p in NumKey::K1.find_all_paths_to(NumKey::K9) {
        println!();
        model::print_numeric_keypad(&p);
    }

    println!("Part 1: {}", complexities_sum(&codes));
    println!("Part 2: {}", part2(&codes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(complexities_sum(&build(INPUT_TEST)), 126384);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
