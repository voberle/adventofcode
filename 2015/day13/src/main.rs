use std::io::{self, Read};

use fxhash::FxHashMap;
use regex::Regex;

type HappinessFactors = FxHashMap<(String, String), i32>;

fn build(input: &str) -> HappinessFactors {
    let re =
        Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let units: i32 = caps[3].parse().unwrap();
            let level = if &caps[2] == "gain" {
                units
            } else if &caps[2] == "lose" {
                -units
            } else {
                panic!("Parsing error")
            };
            ((caps[1].to_string(), caps[4].to_string()), level)
        })
        .collect()
}

fn max_happiness_change(happiness_factors: &HappinessFactors) -> i32 {
    0
}

fn part2(happiness_factors: &HappinessFactors) -> i32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let happiness_factors = build(&input);
    println!("{:#?}", happiness_factors);
    println!("Part 1: {}", max_happiness_change(&happiness_factors));
    println!("Part 2: {}", part2(&happiness_factors));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(max_happiness_change(&build(INPUT_TEST)), 330);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
