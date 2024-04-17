use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    SubRule(Vec<Vec<usize>>),
}

impl Rule {
    fn build(input: &str) -> Self {
        if input.starts_with('"') {
            let c = input.chars().nth(1).unwrap();
            assert!(c == 'a' || c == 'b');
            Self::Char(c)
        } else {
            Self::SubRule(
                input
                    .split('|')
                    .map(|rule_part| {
                        rule_part
                            .split_whitespace()
                            .map(|id| id.parse().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        }
    }
}

fn build(input: &str) -> (Vec<Rule>, Vec<String>) {
    let mut it = input.lines();
    // Rules are not sorted in the input.
    let mut rules_map: HashMap<usize, Rule> = HashMap::new();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        let (index, rule) = line.split(": ").collect_tuple().unwrap();
        rules_map.insert(index.parse().unwrap(), Rule::build(rule));
    }
    // Convert the rule map to a vector.
    let rules = rules_map
        .iter()
        .sorted_by_key(|(k, _)| *k)
        .enumerate()
        .map(|(i, (k, v))| {
            assert_eq!(i, *k);
            v.clone()
        })
        .collect();

    let mut messages = Vec::new();
    for line in it {
        messages.push(line.to_string());
    }
    (rules, messages)
}

fn messages_matching_rule0(rules: &[Rule], messages: &[String]) -> usize {
    0
}

fn part2(rules: &[Rule], messages: &[String]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (rules, messages) = build(&input);
    // println!("{:?}", rules);
    // println!("Messages: {:?} {}-{}", messages.len(), messages.first().unwrap(), messages.last().unwrap());

    println!("Part 1: {}", messages_matching_rule0(&rules, &messages));
    println!("Part 2: {}", part2(&rules, &messages));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (rules, messages) = build(INPUT_TEST);
        assert_eq!(messages_matching_rule0(&rules, &messages), 2);
    }

    #[test]
    fn test_part2() {
        let (rules, messages) = build(INPUT_TEST);
        assert_eq!(part2(&rules, &messages), 0);
    }
}
