// https://adventofcode.com/2023/day/7
// Part 1 test: 6440

use std::{cmp::Ordering, collections::HashMap, io};

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

impl Card {
    fn new(c: char) -> Self {
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

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard, // where all cards' labels are distinct: 23456
    OnePair, // where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    TwoPair, // where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    ThreeOfAKind, // where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    FullHouse, // where three cards have the same label, and the remaining two cards share a different label: 23332
    FourOfAKind,  // where four cards have the same label and one card has a different label: AA8AA
    FiveOfAKind,  // where all five cards have the same label: AAAAA
}

impl HandType {
    fn new(freq: &Vec<i32>) -> Self {
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

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>, // len is always 5
    hand_type: HandType,
}

impl Hand {
    fn new(s: &str) -> Self {
        let cards = s.chars().map(Card::new).collect();
        let hand_type = Self::recognize(&cards);
        Self {
            cards: cards,
            hand_type: hand_type,
        }
    }

    fn recognize(cards: &Vec<Card>) -> HandType {
        let frequencies_map = cards
            .iter()
            // .copied()
            .fold(HashMap::new(), |mut map, val| {
                map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                map
            });
        let mut frequencies: Vec<i32> = frequencies_map.values().cloned().collect();
        frequencies.sort();
        frequencies.reverse();
        HandType::new(&frequencies)
    }
}

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

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct HandBid {
    hand: Hand,
    bid: u32,
}

impl HandBid {
    fn new(hand: &str, bid: u32) -> Self {
        Self {
            hand: Hand::new(hand),
            bid: bid,
        }
    }
}
fn main() {
    let stdin = io::stdin();
    let mut hands: Vec<HandBid> = Vec::new();
    for l in stdin.lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split_whitespace().collect();
        hands.push(HandBid::new(v[0], v[1].parse().unwrap()));
    }

    hands.sort();
    let total_winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();
    println!("Part 1: {}", total_winnings);
}
