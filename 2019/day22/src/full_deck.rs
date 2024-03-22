//! Shuffle functions that manipulate the full deck.

use crate::Technique;

type Deck = Vec<u64>;

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
    let mut table: Deck = vec![u64::MAX; deck.len()];
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
            Technique::DealNewStack => deal_new_stack(deck),
            Technique::Cut(n) => cut(deck, *n),
            Technique::DealIncrement(n) => deal_with_increment(deck, *n),
        }
    }
}

fn create_deck(cards_count: usize) -> Deck {
    (0..cards_count).map(|v| v.try_into().unwrap()).collect()
}

// Part 1
#[allow(dead_code)]
pub fn shuffle_position_of(techniques: &[Technique], card: u64) -> usize {
    let mut deck = create_deck(10007);
    shuffle(techniques, &mut deck);
    deck.iter()
        .position(|&c| card == c)
        .expect("Didn't find card")
}

#[cfg(test)]
mod tests {
    use crate::build;
    use crate::tests::*;

    use super::*;

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
}
