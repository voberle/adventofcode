///! Shuffle functions that track only one card.
///! It's the position of the card that is tracked.

use std::cmp::Ordering;

use crate::Technique;

fn deal_new_stack(deck_size: usize, pos: usize) -> usize {
    deck_size - 1 - pos
}

fn deal_new_stack_reversed(deck_size: usize, pos: usize) -> usize {
    deck_size - 1 - pos
}

#[allow(clippy::cast_sign_loss)]
fn cut(deck_size: usize, n: i32, pos: usize) -> usize {
    // This could be simplified as
    // (pos as isize - n as isize).rem_euclid(deck_size as isize) as usize
    match n.cmp(&0) {
        Ordering::Greater => (pos + deck_size - n as usize).rem_euclid(deck_size),
        Ordering::Less => (pos + n.unsigned_abs() as usize).rem_euclid(deck_size),
        Ordering::Equal => pos,
    }
}

#[allow(clippy::cast_sign_loss)]
fn cut_reversed(deck_size: usize, n: i32, pos: usize) -> usize {
    match n.cmp(&0) {
        Ordering::Greater => (pos + n as usize).rem_euclid(deck_size),
        Ordering::Less => (pos + deck_size - n.unsigned_abs() as usize).rem_euclid(deck_size),
        Ordering::Equal => pos,
    }
}

fn deal_with_increment(deck_size: usize, n: usize, pos: usize) -> usize {
    // If numbers are positive, equivalent to (pos * n).rem_euclid(deck_size)
    (pos * n) % deck_size
}

// modular multiplicative inverse of the n modulo n.
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn modinverse(n: usize, p: usize) -> usize {
    modinverse::modinverse(n as i128, p as i128)
        .unwrap()
        .try_into()
        .unwrap()
}

#[allow(clippy::cast_sign_loss)]
fn deal_with_increment_reversed(deck_size: usize, n: usize, pos: usize) -> usize {
    // To get the original position of the card:
    // 1. Find the modular multiplicative inverse of the increment (n) modulo the deck size.
    let mod_inverse = modinverse(n, deck_size) as i128;
    // 2. Multiply the position of the card by the modular multiplicative inverse.
    let m = pos as i128 * mod_inverse;
    // 3. Take the result modulo the deck size.
    (m % deck_size as i128) as usize
}

pub fn shuffle(techniques: &[Technique], deck_size: usize, pos: &mut usize) {
    for t in techniques {
        match t {
            Technique::DealNewStack => *pos = deal_new_stack(deck_size, *pos),
            Technique::Cut(n) => *pos = cut(deck_size, *n, *pos),
            Technique::DealIncrement(n) => *pos = deal_with_increment(deck_size, *n, *pos),
        }
    }
}

pub fn shuffle_reversed(techniques: &[Technique], deck_size: usize, pos: &mut usize) {
    for t in techniques.iter().rev() {
        match t {
            Technique::DealNewStack => *pos = deal_new_stack_reversed(deck_size, *pos),
            Technique::Cut(n) => *pos = cut_reversed(deck_size, *n, *pos),
            Technique::DealIncrement(n) => *pos = deal_with_increment_reversed(deck_size, *n, *pos),
        }
    }
}

// Part 1
#[allow(dead_code)]
pub fn shuffle_position_of(techniques: &[Technique], card: u64) -> usize {
    const DECK_SIZE: usize = 10007;
    // In the initial deck, the cards are ordered, so the position is the same as card number.
    let mut pos = card as usize;
    shuffle(techniques, DECK_SIZE, &mut pos);
    pos
}

#[cfg(test)]
mod tests {
    use crate::build;
    use crate::tests::*;

    use super::*;

    #[test]
    fn test_deal_new_stack() {
        // 9 8 7 6 5 4 3 2 1 0
        // 0 1 2 3 4 5 6 7 8 9
        assert_eq!(deal_new_stack(10, 7), 2);
    }

    #[test]
    fn test_deal_new_stack_reversed() {
        // 9 8 7 6 5 4 3 2 1 0
        // 0 1 2 3 4 5 6 7 8 9
        assert_eq!(deal_new_stack_reversed(10, 7), 2);
    }

    #[test]
    fn test_cut_positive() {
        // 0 1 2 3 4 5 6 7 8 9
        // 3 4 5 6 7 8 9 0 1 2
        assert_eq!(cut(10, 3, 7), 4);
    }

    #[test]
    fn test_cut_negative() {
        // 0 1 2 3 4 5 6 7 8 9
        // 6 7 8 9 0 1 2 3 4 5
        assert_eq!(cut(10, -4, 7), 1);
    }

    #[test]
    fn test_cut_reversed_positive() {
        // 3 4 5 6 7 8 9 0 1 2
        // 0 1 2 3 4 5 6 7 8 9
        assert_eq!(cut_reversed(10, 3, 7), 0);
    }

    #[test]
    fn test_cut_reversed_negative() {
        // 6 7 8 9 0 1 2 3 4 5
        // 0 1 2 3 4 5 6 7 8 9
        assert_eq!(cut_reversed(10, -4, 7), 3);
        assert_eq!(cut_reversed(10, -4, 2), 8);
    }

    #[test]
    fn test_deal_with_increment() {
        // 0 1 2 3 4 5 6 7 8 9
        // 0 7 4 1 8 5 2 9 6 3
        assert_eq!(deal_with_increment(10, 3, 0), 0);
        assert_eq!(deal_with_increment(10, 3, 7), 1);
        assert_eq!(deal_with_increment(10, 3, 4), 2);
        assert_eq!(deal_with_increment(10, 3, 1), 3);
        assert_eq!(deal_with_increment(10, 3, 8), 4);
        assert_eq!(deal_with_increment(10, 3, 5), 5);
        assert_eq!(deal_with_increment(10, 3, 2), 6);
        assert_eq!(deal_with_increment(10, 3, 9), 7);
        assert_eq!(deal_with_increment(10, 3, 6), 8);
        assert_eq!(deal_with_increment(10, 3, 3), 9);
    }

    #[test]
    fn test_deal_with_increment_reversed() {
        // 0 7 4 1 8 5 2 9 6 3
        // 0 1 2 3 4 5 6 7 8 9
        assert_eq!(deal_with_increment_reversed(10, 3, 0), 0);
        assert_eq!(deal_with_increment_reversed(10, 3, 1), 7);
        assert_eq!(deal_with_increment_reversed(10, 3, 2), 4);
        assert_eq!(deal_with_increment_reversed(10, 3, 3), 1);
        assert_eq!(deal_with_increment_reversed(10, 3, 4), 8);
        assert_eq!(deal_with_increment_reversed(10, 3, 5), 5);
        assert_eq!(deal_with_increment_reversed(10, 3, 6), 2);
        assert_eq!(deal_with_increment_reversed(10, 3, 7), 9);
        assert_eq!(deal_with_increment_reversed(10, 3, 8), 6);
        assert_eq!(deal_with_increment_reversed(10, 3, 9), 3);
    }

    fn shuffle_10(techniques: &[Technique], pos_to_find: usize) -> usize {
        let mut pos = pos_to_find;
        shuffle_reversed(techniques, 10, &mut pos);
        pos
    }

    #[test]
    fn test_shuffle_reversed() {
        assert_eq!(shuffle_10(&build(INPUT_TEST_1), 7), 1);
        assert_eq!(shuffle_10(&build(INPUT_TEST_2), 7), 2);
        assert_eq!(shuffle_10(&build(INPUT_TEST_3), 7), 5);
        assert_eq!(shuffle_10(&build(INPUT_TEST_4), 7), 0);
    }
}
