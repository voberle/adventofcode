use std::io::{self, Read};

use fxhash::FxHashMap;
use itertools::Itertools;
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

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
const fn wrapping_index(i: i32, len: usize) -> usize {
    // https://stackoverflow.com/a/45397704
    let c = len as i32;
    ((i % c + c) % c) as usize
}

#[test]
fn test_wrapping_index() {
    assert_eq!(wrapping_index(-1, 6), 5);
    assert_eq!(wrapping_index(0, 6), 0);
    assert_eq!(wrapping_index(3, 6), 3);
    assert_eq!(wrapping_index(6, 6), 0);
}

fn list_of_guests(happiness_factors: &HappinessFactors) -> Vec<String> {
    happiness_factors
        .keys()
        .map(|(g1, _)| g1)
        .unique()
        .cloned()
        .collect()
}

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
fn max_happiness_change(happiness_factors: &HappinessFactors) -> i32 {
    // Brute-forced by finding all possible permutations of guests
    list_of_guests(happiness_factors)
        .iter()
        .permutations(list_of_guests(happiness_factors).len())
        .unique()
        .map(|perm| {
            perm.iter()
                .enumerate()
                .map(|(i, g)| {
                    let n1 = perm[wrapping_index(i as i32 - 1, perm.len())];
                    let n2 = perm[wrapping_index(i as i32 + 1, perm.len())];
                    happiness_factors
                        .get(&((*g).to_string(), n1.to_string()))
                        .unwrap()
                        + happiness_factors
                            .get(&((*g).to_string(), n2.to_string()))
                            .unwrap()
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn happiness_with_me(happiness_factors: &mut HappinessFactors) -> i32 {
    const ME: &str = "Vincent";
    let guests = list_of_guests(happiness_factors);
    for g in &guests {
        happiness_factors.insert((ME.to_string(), g.to_string()), 0);
        happiness_factors.insert((g.to_string(), ME.to_string()), 0);
    }

    max_happiness_change(happiness_factors)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut happiness_factors = build(&input);

    println!("Part 1: {}", max_happiness_change(&happiness_factors));
    println!("Part 2: {}", happiness_with_me(&mut happiness_factors));
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
        assert_eq!(happiness_with_me(&mut build(INPUT_TEST)), 286);
    }
}
