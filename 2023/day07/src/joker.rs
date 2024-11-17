use std::cmp::Ordering;

use fxhash::FxHashMap;
use itertools::Itertools;

use crate::CardGeneric;
use crate::HandType;

type Card = CardGeneric<0>;

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
        let mut highest_hand_type = HandType::HighCard;
        if cards.contains(&Card::J) {
            for replacement_card in Card::all_cards() {
                let mut cards_copy = cards.to_vec();
                for c in &mut cards_copy.iter_mut() {
                    if *c == Card::J {
                        *c = replacement_card;
                    }
                }
                highest_hand_type = highest_hand_type.max(Self::find_hand_type(&cards_copy));
            }
        }
        highest_hand_type.max(Self::find_hand_type(cards))
    }

    fn find_hand_type(cards: &[Card]) -> HandType {
        let frequencies_map = cards.iter().fold(FxHashMap::default(), |mut map, val| {
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
struct HandBid {
    hand: Hand,
    bid: u32,
}

pub fn total_winnings(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_whitespace().collect_tuple().unwrap();
            HandBid {
                hand: Hand::new(hand),
                bid: bid.parse().unwrap(),
            }
        })
        .sorted_unstable()
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
        let mut hands = [
            Hand::new("32T3K"),
            Hand::new("T55J5"),
            Hand::new("KK677"),
            Hand::new("KTJJT"),
            Hand::new("QQQJA"),
        ];
        hands.sort();
        assert_eq!(hands[0], Hand::new("32T3K"));
        assert_eq!(hands[1], Hand::new("KK677"));
        assert_eq!(hands[2], Hand::new("T55J5"));
        assert_eq!(hands[3], Hand::new("QQQJA"));
        assert_eq!(hands[4], Hand::new("KTJJT"));
    }
}
