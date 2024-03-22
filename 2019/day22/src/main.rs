use std::io::{self, Read};

mod card_only;
mod full_deck;

#[derive(Debug)]
enum Technique {
    DealNewStack,
    Cut(i32),
    DealIncrement(usize),
}

fn build(input: &str) -> Vec<Technique> {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                Technique::DealNewStack
            } else if let Ok(inc) = line.trim_start_matches("deal with increment ").parse() {
                Technique::DealIncrement(inc)
            } else if let Ok(cut) = line.trim_start_matches("cut ").parse() {
                Technique::Cut(cut)
            } else {
                panic!("Cannot parse technicques")
            }
        })
        .collect()
}

fn shuffle_position_of(techniques: &[Technique], card: u64) -> usize {
    // full_deck::shuffle_position_of(techniques, card)
    card_only::shuffle_position_of(techniques, card)
}

// Each shuffle function is a linear function in the form
//      f(x) = a * x + b
// Combining several linear functions still gives a linear function,
// meaning we can simplify the whole shuffle_reversed into such a simple function as well.
// We just need to find a and b, which we can do by observing the output of two function calls
// and resolving the equation.
fn calc_ab(techniques: &[Technique], deck_size: usize) -> (usize, usize) {
    // x0 = 0
    let mut pos = 0;
    card_only::shuffle_reversed(techniques, deck_size, &mut pos);
    let y0 = pos;

    // x1 = 1
    let mut pos = 1;
    card_only::shuffle_reversed(techniques, deck_size, &mut pos);
    let y1 = pos;

    let a = y1 - y0;
    let b = y0;
    (a, b)
}

// The shuffle method implemented as a linear function.
fn shuffle_reversed_linear(a: usize, b: usize, deck_size: usize, pos: usize) -> usize {
    (a * pos + b) % deck_size
}

// Verify that the normal shuffle and the linear one match.
fn assert_shuffle_functions_eq(
    techniques: &[Technique],
    a: usize,
    b: usize,
    deck_size: usize,
    position: usize,
) {
    let mut pos_normal = position;
    card_only::shuffle_reversed(techniques, deck_size, &mut pos_normal);

    let pos_lin = shuffle_reversed_linear(a, b, deck_size, position);

    assert_eq!(pos_normal, pos_lin);
}

use num::bigint::BigInt;
use num::ToPrimitive;

// Apply the linear function many times.
// Formula is:
//   ( p1 * pos + b * p2 ) % deck_size
// where
//   p1 = pow(a, shuffle_count) % deck_size
//   p2 = (p1 - 1) * pow(a - 1, deck_size - 2) % deck_size
#[allow(clippy::useless_conversion)] // Remove false warnings.
fn apply(a: usize, b: usize, shuffle_count: usize, pos: usize, deck_size: usize) -> usize {
    // BigInt is needed, i128 isn't enough.
    let a = BigInt::from(a);
    let b = BigInt::from(b);
    let shuffle_count = BigInt::from(shuffle_count);
    let pos = BigInt::from(pos);
    let deck_size = BigInt::from(deck_size);

    let p1 = a.modpow(&shuffle_count, &deck_size);

    let p2: BigInt = (p1.clone() - 1)
        * BigInt::from(a - 1).modpow(&BigInt::from(deck_size.clone() - 2), &deck_size);

    let result = (p1.clone() * pos + b * p2) % deck_size.clone();
    result.to_usize().unwrap()
}

fn shuffle_number_of_card_at(techniques: &[Technique], position: usize) -> u64 {
    const DECK_SIZE: usize = 119_315_717_514_047;
    const SHUFFLE_COUNT: usize = 101_741_582_076_661;

    // Get the params representing our shuffling as a linear function.
    let (a, b) = calc_ab(techniques, DECK_SIZE);

    assert_shuffle_functions_eq(techniques, a, b, DECK_SIZE, position);

    // Apply the function a trillion times.
    apply(a, b, SHUFFLE_COUNT, position, DECK_SIZE) as u64
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let techniques = build(&input);

    println!("Part 1: {}", shuffle_position_of(&techniques, 2019));
    println!("Part 2: {}", shuffle_number_of_card_at(&techniques, 2020));
}

#[cfg(test)]
mod tests {
    pub const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    pub const RESULT_1: [u64; 10] = [0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
    pub const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    pub const RESULT_2: [u64; 10] = [3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
    pub const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    pub const RESULT_3: [u64; 10] = [6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
    pub const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");
    pub const RESULT_4: [u64; 10] = [9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
}
