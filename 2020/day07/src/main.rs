use std::io::{self, Read};

use regex::Regex;

#[derive(Debug)]
struct Rule {
    outer_bag: String,
    inner_bags: Vec<(usize, String)>,
}

impl Rule {
    fn new(outer_bag: String, inner_bags: Vec<(usize, String)>) -> Self {
        Self {
            outer_bag,
            inner_bags,
        }
    }
}

fn build(input: &str) -> Vec<Rule> {
    let re = Regex::new(r"(\d+) (.+) bag(s?)").unwrap();
    input
        .lines()
        .map(|line| {
            let p: Vec<_> = line.split(" bags contain ").collect();
            let outer_bag = p[0].to_string();
            let inner_bags: Vec<_> = p[1]
                .trim_end_matches('.')
                .split(", ")
                .filter_map(|i| {
                    re.captures(i)
                        .map(|ip| (ip[1].parse().unwrap(), ip[2].to_string()))
                })
                .collect();
            Rule {
                outer_bag,
                inner_bags,
            }
        })
        .collect()
}

fn part1(input: &[Rule]) -> i64 {
    0
}

fn part2(input: &[Rule]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rules = build(&input);
    println!("{:?}", rules);

    println!("Part 1: {}", part1(&rules));
    println!("Part 2: {}", part2(&rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
