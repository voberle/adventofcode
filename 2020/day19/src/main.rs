use std::{
    collections::HashMap,
    fmt,
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

    // Checks if there are no sub-rules.
    fn is_empty(&self) -> bool {
        match self {
            Rule::Char(_) => false,
            Rule::SubRule(sub_rule) => sub_rule.is_empty(),
        }
    }

    // Checks if any of the sub-rules contain any of the rule numbers.
    fn contains(&self, rule_numbers: &[usize]) -> bool {
        match self {
            Rule::Char(_) => false,
            Rule::SubRule(sub_rule) => sub_rule
                .iter()
                .any(|r| r.iter().any(|i| rule_numbers.contains(i))),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rule::Char(c) => c.to_string(),
                Rule::SubRule(sub_rule) => {
                    sub_rule.iter().map(|r| r.iter().join(" ")).join(" | ")
                }
            }
        )
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

fn build_matching_messages_for(rules: &[Rule], rule_nb: usize) -> Vec<Vec<char>> {
    let mut messages: Vec<Vec<Element>> = Vec::new();
    messages.push(vec![Element::Index(rule_nb)]);
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

fn messages_matching_0(rules: &[Rule], messages: &[Vec<char>]) -> usize {
    let matching_messages = build_matching_messages_for(rules, 0);
    messages
        .iter()
        .filter(|msg| matching_messages.contains(msg))
        .count()
}

fn matching_messages_to_string(messages: &[Vec<char>]) -> String {
    messages.iter().map(|r| r.iter().join("")).join(" | ")
}

// Builds each rule as far as possible.
// nb_to_ignore allows to specify some rule numbers to skip, in our case the ones that are looping.
#[allow(dead_code)]
fn print_expanded_rules(rules: &[Rule], nb_to_ignore: &[usize]) {
    for rule_nb in 0..rules.len() {
        if rules[rule_nb].is_empty() {
            continue;
        }
        if nb_to_ignore.contains(&rule_nb) {
            println!("{}: {}", rule_nb, rules[rule_nb]);
            continue;
        }
        if rules[rule_nb].contains(nb_to_ignore) {
            println!("{}: {}", rule_nb, rules[rule_nb]);
            continue;
        }
        let m = build_matching_messages_for(rules, rule_nb);
        println!("{}: {}", rule_nb, matching_messages_to_string(&m));
    }
}

fn messages_matching_0_updated(rules: &[Rule], messages: &[Vec<char>]) -> usize {
    // print_expanded_rules(rules, &[8, 11]);

    // The message must start with a sequence from 42 and finish with one of 31.
    let starting_tokens = build_matching_messages_for(rules, 42);
    let ending_tokens = build_matching_messages_for(rules, 31);

    let token_size = starting_tokens.first().unwrap().len();
    assert_eq!(ending_tokens.first().unwrap().len(), token_size);

    messages
        .iter()
        .filter(|msg| {
            let chunks: Vec<Vec<char>> = msg.chunks(token_size).map(<[char]>::to_vec).collect();
            // Size must be at least 3.
            if chunks.len() < 3 {
                return false;
            }
            // First 2 must be 42, last must be 31
            if !starting_tokens.contains(&chunks[0]) {
                return false;
            }
            if !starting_tokens.contains(&chunks[1]) {
                return false;
            }
            if !ending_tokens.contains(chunks.last().unwrap()) {
                return false;
            }
            let mut count_42 = 0;
            let mut count_31 = 0;

            let mut zone = 1;
            for chunk in chunks {
                if zone == 1 {
                    if starting_tokens.contains(&chunk) {
                        count_42 += 1;
                        continue;
                    } else if ending_tokens.contains(&chunk) {
                        count_31 += 1;
                        zone = 2;
                        continue;
                    }
                    return false;
                }
                if zone == 2 {
                    if ending_tokens.contains(&chunk) {
                        count_31 += 1;
                        continue;
                    }
                    return false;
                }
                panic!("Should never get here");
            }
            // There must be more 42 than 31.
            if count_42 <= count_31 {
                return false;
            }
            true
        })
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (rules, messages) = build(&input);

    println!("Part 1: {}", messages_matching_0(&rules, &messages));
    println!("Part 2: {}", messages_matching_0_updated(&rules, &messages));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        let (rules, messages) = build(INPUT_TEST_1);
        assert_eq!(messages_matching_0(&rules, &messages), 2);

        let (rules, messages) = build(INPUT_TEST_2);
        assert_eq!(messages_matching_0(&rules, &messages), 3);
    }
}
