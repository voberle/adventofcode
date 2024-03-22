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

// We want to know the card that is at a specific position at the end.
// Could we start with the end, and follow what is at that position backwards?
fn shuffle_number_of_card_at(techniques: &[Technique], position: usize) -> u64 {
    const DECK_SIZE: usize = 119_315_717_514_047;
    const SHUFFLE_COUNT: usize = 101_741_582_076_661;

    let mut pos = position;
    for _ in 0..SHUFFLE_COUNT {
        // shuffle_reversed(techniques, DECK_SIZE, &mut pos);
    }
    pos as u64
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
