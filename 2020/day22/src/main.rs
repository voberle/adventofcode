use std::{
    collections::VecDeque,
    io::{self, Read},
};

use fxhash::FxHashSet;

fn build(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();

    let mut it = input.lines();
    assert_eq!(it.next().unwrap(), "Player 1:");
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        deck1.push(line.parse().unwrap());
    }
    assert_eq!(it.next().unwrap(), "Player 2:");
    for line in it {
        deck2.push(line.parse().unwrap());
    }
    (deck1, deck2)
}

fn add_card_to_winning_deck(deck: &mut VecDeque<u32>, winner_card: u32, looser_card: u32) {
    // winner card goes first
    deck.push_back(winner_card);
    deck.push_back(looser_card);
}

fn play_normal(deck1: &mut VecDeque<u32>, deck2: &mut VecDeque<u32>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();

        if top1 > top2 {
            add_card_to_winning_deck(deck1, top1, top2);
        } else {
            add_card_to_winning_deck(deck2, top2, top1);
        }
    }
}

fn get_subdeck(deck: &VecDeque<u32>, nb_cards: usize) -> VecDeque<u32> {
    let mut copy_deck: VecDeque<u32> = VecDeque::new();
    // A bit hackish, but avoids making it contiguous.
    let (s1, s2) = deck.as_slices();
    if nb_cards < s1.len() {
        copy_deck.extend(&s1[0..nb_cards]);
    } else {
        copy_deck.extend(s1);
        copy_deck.extend(&s2[0..nb_cards - s1.len()]);
    }
    copy_deck
}

fn play_recursive(deck1: &mut VecDeque<u32>, deck2: &mut VecDeque<u32>) {
    // Is a VecDeque the best to put in the set?
    let mut prev_decks1: FxHashSet<VecDeque<u32>> = FxHashSet::default();
    let mut prev_decks2: FxHashSet<VecDeque<u32>> = FxHashSet::default();
    recursive_combat(deck1, deck2, &mut prev_decks1, &mut prev_decks2);
}

fn recursive_combat(
    deck1: &mut VecDeque<u32>,
    deck2: &mut VecDeque<u32>,
    prev_decks1: &mut FxHashSet<VecDeque<u32>>,
    prev_decks2: &mut FxHashSet<VecDeque<u32>>,
) {
    while !deck1.is_empty() && !deck2.is_empty() {
        // Infinite games prevention.
        if !prev_decks1.insert(deck1.clone()) || !prev_decks2.insert(deck2.clone()) {
            // Player 1 won, clearing deck 2 to make him loose.
            deck2.clear();
            return;
        }

        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();
        if deck1.len() >= top1 as usize && deck2.len() >= top2 as usize {
            // Recursive combat.
            let mut copy_deck1 = get_subdeck(deck1, top1 as usize);
            let mut copy_deck2 = get_subdeck(deck2, top2 as usize);

            play_recursive(&mut copy_deck1, &mut copy_deck2);

            if copy_deck2.is_empty() {
                add_card_to_winning_deck(deck1, top1, top2);
            } else {
                add_card_to_winning_deck(deck2, top2, top1);
            }
        } else {
            // Normal combat.
            if top1 > top2 {
                add_card_to_winning_deck(deck1, top1, top2);
            } else {
                add_card_to_winning_deck(deck2, top2, top1);
            }
        }
    }
}

fn calculate_score(winning_deck: &VecDeque<u32>) -> u32 {
    winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, e)| acc + (u32::try_from(i).unwrap() + 1) * e)
}

fn winning_score(
    deck1: &[u32],
    deck2: &[u32],
    play_fn: fn(&mut VecDeque<u32>, &mut VecDeque<u32>),
) -> u32 {
    let mut deck1: VecDeque<u32> = VecDeque::from(deck1.to_vec());
    let mut deck2: VecDeque<u32> = VecDeque::from(deck2.to_vec());

    play_fn(&mut deck1, &mut deck2);

    let winning_deck = if deck1.is_empty() { &deck2 } else { &deck1 };
    calculate_score(winning_deck)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (deck1, deck2) = build(&input);

    println!("Part 1: {}", winning_score(&deck1, &deck2, play_normal));
    println!("Part 2: {}", winning_score(&deck1, &deck2, play_recursive));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (deck1, deck2) = build(INPUT_TEST);
        assert_eq!(winning_score(&deck1, &deck2, play_normal), 306);
    }

    #[test]
    fn test_part2() {
        let (deck1, deck2) = build(INPUT_TEST);
        assert_eq!(winning_score(&deck1, &deck2, play_recursive), 291);
    }
}
