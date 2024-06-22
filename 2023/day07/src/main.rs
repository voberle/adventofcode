use std::cmp::Ordering;
use std::io::{self, Read};

mod base;
mod joker;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard,     // where all cards' labels are distinct: 23456
    OnePair, // where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    TwoPair, // where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    ThreeOfAKind, // where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    FullHouse, // where three cards have the same label, and the remaining two cards share a different label: 23332
    FourOfAKind, // where four cards have the same label and one card has a different label: AA8AA
    FiveOfAKind, // where all five cards have the same label: AAAAA
}

impl HandType {
    fn new(freq: &[i32]) -> Self {
        assert_eq!(freq.iter().sum::<i32>(), 5);
        // Slice patterns https://doc.rust-lang.org/reference/patterns.html#slice-patterns
        match freq[..] {
            [5] => Self::FiveOfAKind,
            [4, 1] => Self::FourOfAKind,
            [3, 2] => Self::FullHouse,
            [3, 1, 1] => Self::ThreeOfAKind,
            [2, 2, 1] => Self::TwoPair,
            [2, 1, 1, 1] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => panic!("Invalid frequency array"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum CardGeneric<const J_ORD: usize> {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl<const J_ORD: usize> From<char> for CardGeneric<J_ORD> {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => panic!("Invalid char"),
        }
    }
}

impl<const J_ORD: usize> CardGeneric<J_ORD> {
    fn index(self) -> usize {
        match self {
            // 0 kept free for J.
            CardGeneric::C2 => 1,
            CardGeneric::C3 => 2,
            CardGeneric::C4 => 3,
            CardGeneric::C5 => 4,
            CardGeneric::C6 => 5,
            CardGeneric::C7 => 6,
            CardGeneric::C8 => 7,
            CardGeneric::C9 => 8,
            CardGeneric::T => 9,
            CardGeneric::J => J_ORD,
            CardGeneric::Q => 11,
            CardGeneric::K => 12,
            CardGeneric::A => 13,
        }
    }

    fn all_cards() -> [CardGeneric<J_ORD>; 13] {
        [
            CardGeneric::C2,
            CardGeneric::C3,
            CardGeneric::C4,
            CardGeneric::C5,
            CardGeneric::C6,
            CardGeneric::C7,
            CardGeneric::C8,
            CardGeneric::C9,
            CardGeneric::T,
            CardGeneric::J,
            CardGeneric::Q,
            CardGeneric::K,
            CardGeneric::A,
        ]
    }
}

impl<const J_ORD: usize> PartialOrd for CardGeneric<J_ORD> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const J_ORD: usize> Ord for CardGeneric<J_ORD> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index().cmp(&other.index())
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let hands = base::build(&input);
    println!("Part 1: {}", base::total_winnings(&hands));

    let hands = joker::build(&input);
    println!("Part 2: {}", joker::total_winnings(&hands));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1() {
        assert_eq!(base::total_winnings(&base::build(INPUT_TEST)), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(joker::total_winnings(&joker::build(INPUT_TEST)), 5905);
    }
}
