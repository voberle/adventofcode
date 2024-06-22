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
