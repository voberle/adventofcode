use std::{
    cmp::Ordering,
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Ord for Signal {
    fn cmp(&self, right: &Self) -> Ordering {
        match self {
            Signal::Integer(left_val) => {
                match right {
                    Signal::Integer(right_val) => {
                        // Both values are integers.
                        left_val.cmp(right_val)
                    }
                    Signal::List(_) => {
                        // Left is integer, right is list.
                        // Convert left to list and retry.
                        Signal::List(vec![self.clone()]).cmp(right)
                    }
                }
            }
            Signal::List(left_list) => {
                match right {
                    Signal::Integer(_) => {
                        // Left is list, right is integer.
                        self.cmp(&Signal::List(vec![right.clone()]))
                    }
                    Signal::List(right_list) => {
                        // Both values are lists.
                        for (l, r) in left_list.iter().zip(right_list.iter()) {
                            let r = l.cmp(r);
                            if r.is_ne() {
                                return r;
                            }
                        }
                        left_list.len().cmp(&right_list.len())
                    }
                }
            }
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build(input: &str) -> Vec<(Signal, Signal)> {
    input
        .split("\n\n")
        .map(|pair| pair.lines().map(Into::into).collect_tuple().unwrap())
        .collect()
}

fn right_order_sum(pairs: &[(Signal, Signal)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(
            |(i, (left, right))| {
                if left < right {
                    Some(i + 1)
                } else {
                    None
                }
            },
        )
        .sum()
}

fn distress_signal_decoder_key(pairs: &[(Signal, Signal)]) -> usize {
    let divider_packets = [
        Signal::List(vec![Signal::List(vec![Signal::Integer(2)])]),
        Signal::List(vec![Signal::List(vec![Signal::Integer(6)])]),
    ];

    // Convert the list of pairs to a flat list.
    let mut packets: Vec<&Signal> = pairs.iter().flat_map(|pair| [&pair.0, &pair.1]).collect();
    // Add the divider packets.
    packets.push(&divider_packets[0]);
    packets.push(&divider_packets[1]);

    packets.sort_unstable();

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if divider_packets.contains(packet) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let pairs = build(&input);

    println!("Part 1: {}", right_order_sum(&pairs));
    println!("Part 2: {}", distress_signal_decoder_key(&pairs));
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
        assert_eq!(distress_signal_decoder_key(&build(INPUT_TEST)), 140);
    }
}
