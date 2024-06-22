use std::{cmp::Ordering, collections::HashMap};

use crate::HandType;

// variants are ordered by their top-to-bottom discriminant order
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
enum Card {
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

impl From<char> for Card {
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

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>, // len is always 5
    hand_type: HandType,
}

impl Hand {
    fn new(s: &str) -> Self {
        let cards: Vec<Card> = s.chars().map(Into::into).collect();
        let hand_type = Self::recognize(&cards);
        Self { cards, hand_type }
    }

    fn recognize(cards: &[Card]) -> HandType {
        let frequencies_map = cards
            .iter()
            // .copied()
            .fold(HashMap::new(), |mut map, val| {
                map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                map
            });
        let mut frequencies: Vec<i32> = frequencies_map.values().copied().collect();
        frequencies.sort_unstable();
        frequencies.reverse();
        HandType::new(&frequencies)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.hand_type.cmp(&other.hand_type);
        if ord == Ordering::Equal {
            self.cards.iter().cmp(other.cards.iter())
        } else {
            ord
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct HandBid {
    hand: Hand,
    bid: u32,
}

impl HandBid {
    fn new(hand: &str, bid: u32) -> Self {
        Self {
            hand: Hand::new(hand),
            bid,
        }
    }
}

pub fn build(input: &str) -> Vec<HandBid> {
    let mut hands: Vec<HandBid> = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            HandBid::new(v[0], v[1].parse().unwrap())
        })
        .collect();

    hands.sort();
    hands
}

pub fn total_winnings(hands: &[HandBid]) -> u32 {
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (u32::try_from(i).unwrap() + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_recognize() {
        assert_eq!(Hand::new("AAAAA").hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand::new("AA8AA").hand_type, HandType::FourOfAKind);
        assert_eq!(Hand::new("23332").hand_type, HandType::FullHouse);
        assert_eq!(Hand::new("TTT98").hand_type, HandType::ThreeOfAKind);
        assert_eq!(Hand::new("23432").hand_type, HandType::TwoPair);
        assert_eq!(Hand::new("A23A4").hand_type, HandType::OnePair);
        assert_eq!(Hand::new("23456").hand_type, HandType::HighCard);
    }

    #[test]
    fn check_ordering() {
        let mut hands = vec![
            Hand::new("32T3K"),
            Hand::new("T55J5"),
            Hand::new("KK677"),
            Hand::new("KTJJT"),
            Hand::new("QQQJA"),
        ];
        hands.sort();
        assert_eq!(hands[0], Hand::new("32T3K"));
        assert_eq!(hands[1], Hand::new("KTJJT"));
        assert_eq!(hands[2], Hand::new("KK677"));
        assert_eq!(hands[3], Hand::new("T55J5"));
        assert_eq!(hands[4], Hand::new("QQQJA"));
    }
}
