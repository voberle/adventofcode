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
    const EMPTY: Rule = Rule::SubRule(Vec::new());

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

fn build(input: &str) -> (Vec<Rule>, Vec<Vec<char>>) {
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
    let max_rule_id = *rules_map.keys().max().unwrap();
    let rules = (0..=max_rule_id)
        .map(|i| {
            // On test case 2, some ids don't have rules.
            rules_map.get(&i).unwrap_or(&Rule::EMPTY).clone()
        })
        .collect();

    let mut messages = Vec::new();
    for line in it {
        messages.push(line.chars().collect());
    }
    (rules, messages)
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Char(char),
    Index(usize),
}

fn build_matching_messages(rules: &[Rule]) -> Vec<Vec<char>> {
    let mut messages: Vec<Vec<Element>> = Vec::new();
    messages.push(vec![Element::Index(0)]);
    loop {
        let mut new_messages: Vec<Vec<Element>> = Vec::new();
        for msg in &messages {
            // A vector, as each msg may produce several.
            let mut new_msg: Vec<Vec<Element>> = vec![Vec::new()];
            for e in msg {
                match e {
                    // Already a char, unchanged.
                    Element::Char(_) => {
                        for m in &mut new_msg {
                            m.push(*e);
                        }
                    }
                    // An index, go find the corresponding rule.
                    Element::Index(i) => {
                        let r = &rules[*i];
                        match r {
                            Rule::Char(c) => {
                                for m in &mut new_msg {
                                    m.push(Element::Char(*c));
                                }
                            }
                            Rule::SubRule(sub_rule) => {
                                if sub_rule.len() == 1 {
                                    for m in &mut new_msg {
                                        m.extend(sub_rule[0].iter().map(|s| Element::Index(*s)));
                                    }
                                } else if sub_rule.len() == 2 {
                                    let mut second_new_msg = new_msg.clone();
                                    for m in &mut new_msg {
                                        m.extend(sub_rule[0].iter().map(|s| Element::Index(*s)));
                                    }
                                    for m in &mut second_new_msg {
                                        m.extend(sub_rule[1].iter().map(|s| Element::Index(*s)));
                                    }
                                    new_msg.extend(second_new_msg);
                                } else {
                                    panic!("Invalid sub rule count");
                                }
                            }
                        }
                    }
                }
            }
            if !new_msg.is_empty() {
                new_messages.extend(new_msg);
            }
        }
        std::mem::swap(&mut messages, &mut new_messages);

        // We can optimize things by putting ready messages into a separate list.
        if messages
            .iter()
            .flatten()
            .all(|e| matches!(e, Element::Char(_)))
        {
            break;
        }
    }

    messages
        .iter()
        .map(|msg| {
            msg.iter()
                .map(|e| match e {
                    Element::Char(c) => *c,
                    Element::Index(_) => panic!("Impossible"),
                })
                .collect()
        })
        .collect()
}

fn messages_matching_rule0(rules: &[Rule], messages: &[Vec<char>]) -> usize {
    let matching_messages = build_matching_messages(rules);
    // println!("{:?}", messages);
    // println!("{:?}", matching_messages);
    messages
        .iter()
        .filter(|msg| matching_messages.contains(msg))
        .count()
}

fn part2(rules: &[Rule], messages: &[Vec<char>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (rules, messages) = build(&input);

    println!("Part 1: {}", messages_matching_rule0(&rules, &messages));
    println!("Part 2: {}", part2(&rules, &messages));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        let (rules, messages) = build(INPUT_TEST_1);
        assert_eq!(messages_matching_rule0(&rules, &messages), 2);
        
        let (rules, messages) = build(INPUT_TEST_2);
        assert_eq!(messages_matching_rule0(&rules, &messages), 3);
    }

    #[test]
    fn test_part2() {
    }
}
