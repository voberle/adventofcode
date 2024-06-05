use std::{
    collections::VecDeque,
    io::{self, Read},
};

use itertools::Itertools;

// Intermediary data structure to help with parsing.
#[derive(Debug)]
enum Token {
    Number(u32),
    Open,
    Close,
    Separator,
}

fn parse_to_tokens(input: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                tokens.push_back(Token::Number(current_number.parse().unwrap()));
                current_number.clear();
            }
            tokens.push_back(match c {
                '[' => Token::Open,
                ']' => Token::Close,
                ',' => Token::Separator,
                _ => panic!("Invalid char"),
            });
        }
    }
    if !current_number.is_empty() {
        tokens.push_back(Token::Number(current_number.parse().unwrap()));
    }
    tokens
}

#[derive(Debug, Clone)]
enum Signal {
    Integer(u32),
    List(Vec<Signal>),
}

impl Signal {
    // Helper method for From.
    fn build(tokens: &mut VecDeque<Token>) -> Self {
        let mut items = Vec::new();
        while let Some(token) = tokens.pop_front() {
            match token {
                Token::Number(v) => items.push(Signal::Integer(v)),
                Token::Open => items.push(Signal::build(tokens)),
                Token::Close => break,
                Token::Separator => {}
            }
        }
        Signal::List(items)
    }
}

impl From<&str> for Signal {
    fn from(s: &str) -> Self {
        let mut tokens = parse_to_tokens(s);
        assert!(matches!(tokens.pop_front(), Some(Token::Open)));
        Signal::build(&mut tokens)
    }
}

fn compare_signals(left: &Signal, right: &Signal) -> Option<bool> {
    match left {
        Signal::Integer(left_val) => {
            match right {
                Signal::Integer(right_val) => {
                    // Both values are integers.
                    match left_val.cmp(right_val) {
                        std::cmp::Ordering::Less => Some(true),
                        std::cmp::Ordering::Greater => Some(false),
                        std::cmp::Ordering::Equal => None,
                    }
                }
                Signal::List(_) => {
                    // Left is integer, right is list.
                    // Convert left to list and retry.
                    compare_signals(&Signal::List(vec![left.clone()]), right)
                }
            }
        }
        Signal::List(left_list) => {
            match right {
                Signal::Integer(_) => {
                    // Left is list, right is integer.
                    compare_signals(left, &Signal::List(vec![right.clone()]))
                }
                Signal::List(right_list) => {
                    // Both values are lists.
                    for (l, r) in left_list.iter().zip(right_list.iter()) {
                        let r = compare_signals(l, r);
                        if r.is_some() {
                            return r;
                        }
                    }
                    match left_list.len().cmp(&right_list.len()) {
                        std::cmp::Ordering::Less => Some(true),
                        std::cmp::Ordering::Greater => Some(false),
                        std::cmp::Ordering::Equal => None,
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Signal,
    right: Signal,
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let (left, right) = s.lines().map(Into::into).collect_tuple().unwrap();
        Self { left, right }
    }
}

impl Pair {
    fn compare(&self) -> bool {
        compare_signals(&self.left, &self.right).unwrap()
    }
}

fn build(input: &str) -> Vec<Pair> {
    input.split("\n\n").map(Into::into).collect()
}

fn right_order_sum(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| if pair.compare() { Some(i + 1) } else { None })
        .sum()
}

fn part2(pairs: &[Pair]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let pairs = build(&input);

    println!("Part 1: {}", right_order_sum(&pairs));
    println!("Part 2: {}", part2(&pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(right_order_sum(&build(INPUT_TEST)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
