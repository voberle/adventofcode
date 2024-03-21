use std::io::{self, Read};

#[derive(Debug)]
enum Technique {
    DealNewStack,
    Cut(i32),
    DealIncrement(usize),
}
use Technique::{Cut, DealIncrement, DealNewStack};

fn build(input: &str) -> Vec<Technique> {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                DealNewStack
            } else if let Ok(inc) = line.trim_start_matches("deal with increment ").parse() {
                DealIncrement(inc)
            } else if let Ok(cut) = line.trim_start_matches("cut ").parse() {
                Cut(cut)
            } else {
                panic!("Cannot parse technicques")
            }
        })
        .collect()
}

type Deck = Vec<u32>;

fn deal_new_stack(deck: &mut Deck) {
    deck.reverse();
    // For reversing, a Vec is better than a VecDeque.
    // For a VecDeque use: deck.make_contiguous().reverse();
}

#[allow(clippy::cast_sign_loss)]
fn cut(deck: &mut Deck, n: i32) {
    if n > 0 {
        // A VecDeque performs better than a Vec for rotating.
        deck.rotate_left(n as usize);
    }
    if n < 0 {
        deck.rotate_right(-n as usize);
    }
}

fn deal_with_increment(deck: &mut Deck, n: usize) {
    let mut table: Deck = vec![u32::MAX; deck.len()];
    let mut pos = 0;
    let len = deck.len();
    for card in &mut *deck {
        table[pos] = *card;
        pos = (pos + n).rem_euclid(len);
    }
    std::mem::swap(deck, &mut table);
}

fn shuffle(techniques: &[Technique], deck: &mut Deck) {
    for t in techniques {
        match t {
            DealNewStack => deal_new_stack(deck),
            Cut(n) => cut(deck, *n),
            DealIncrement(n) => deal_with_increment(deck, *n),
        }
    }
}

fn create_deck(cards_count: usize) -> Deck {
    (0..cards_count).map(|v| v.try_into().unwrap()).collect()
}

fn shuffle_position_of(techniques: &[Technique], card: u32) -> usize {
    let mut deck = create_deck(10007);
    shuffle(techniques, &mut deck);
    deck.iter()
        .position(|&c| card == c)
        .expect("Didn't find card")
}

fn part2(techniques: &[Technique]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let techniques = build(&input);

    println!("Part 1: {}", shuffle_position_of(&techniques, 2019));
    println!("Part 2: {}", part2(&techniques));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const RESULT_1: [u32; 10] = [0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const RESULT_2: [u32; 10] = [3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    const RESULT_3: [u32; 10] = [6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
    const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");
    const RESULT_4: [u32; 10] = [9, 2, 5, 8, 1, 4, 7, 0, 3, 6];

    fn shuffle_10(techniques: &[Technique]) -> Deck {
        let mut deck = create_deck(10);
        shuffle(techniques, &mut deck);
        deck
    }

    #[test]
    fn test_cut_positive() {
        let mut deck = create_deck(10);
        cut(&mut deck, 3);
        assert_eq!(deck, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let mut deck = create_deck(10);
        cut(&mut deck, -4);
        assert_eq!(deck, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = create_deck(10);
        deal_with_increment(&mut deck, 3);
        assert_eq!(deck, [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_shuffle() {
        assert_eq!(shuffle_10(&build(INPUT_TEST_1)), RESULT_1);
        assert_eq!(shuffle_10(&build(INPUT_TEST_2)), RESULT_2);
        assert_eq!(shuffle_10(&build(INPUT_TEST_3)), RESULT_3);
        assert_eq!(shuffle_10(&build(INPUT_TEST_4)), RESULT_4);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
