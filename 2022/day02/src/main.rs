use std::{
    cmp::Ordering,
    io::{self, Read},
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::{Paper, Rock, Scissors};

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Invalid element {}", c),
        }
    }
}

impl Shape {
    fn score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    // When the second column indicates how the round needs to end.
    fn round_end(self) -> Ordering {
        match self {
            Rock => Ordering::Less,        // X means you need to lose
            Paper => Ordering::Equal,      // Y means you need to end the round in a draw
            Scissors => Ordering::Greater, // Z means you need to win
        }
    }
}

// Using the Ord trait to indicate which shape wins.
impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if (*self == Rock && *other == Scissors)
            || (*self == Scissors && *other == Paper)
            || (*self == Paper && *other == Rock)
        {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Round {
    opponent: Shape,
    me: Shape,
}

impl Round {
    fn new(opponent: Shape, me: Shape) -> Self {
        Self { opponent, me }
    }

    fn outcome(&self) -> u32 {
        match self.me.cmp(&self.opponent) {
            Ordering::Greater => 6, // I won
            Ordering::Equal => 3,   // draw
            Ordering::Less => 0,    // I lost
        }
    }

    fn score(&self) -> u32 {
        self.me.score() + self.outcome()
    }
}

fn build(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let (opponent, me) = line.chars().step_by(2).collect_tuple().unwrap();
            Round::new(opponent.into(), me.into())
        })
        .collect()
}

fn total_score(strategy: &[Round]) -> u32 {
    strategy.iter().map(Round::score).sum()
}

fn total_score_second_meaning(strategy: &[Round]) -> u32 {
    // Convert the strategy into a new one with new meaning.
    strategy
        .iter()
        .map(|round| {
            let me = match round.me.round_end() {
                Ordering::Less => match round.opponent {
                    // I need to lose.
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                },
                Ordering::Equal => round.opponent,
                Ordering::Greater => match round.opponent {
                    // I need to win.
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock,
                },
            };
            Round::new(round.opponent, me).score()
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let strategy = build(&input);

    println!("Part 1: {}", total_score(&strategy));
    println!("Part 2: {}", total_score_second_meaning(&strategy));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(total_score(&build(INPUT_TEST)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(total_score_second_meaning(&build(INPUT_TEST)), 12);
    }
}
