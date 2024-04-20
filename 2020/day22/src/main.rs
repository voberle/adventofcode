use std::{
    collections::VecDeque,
    io::{self, Read},
};

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

fn play(deck1: &mut VecDeque<u32>, deck2: &mut VecDeque<u32>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();
        if top1 > top2 {
            // winner card goes first
            deck1.push_back(top1);
            deck1.push_back(top2);
        } else {
            deck2.push_back(top2);
            deck2.push_back(top1);
        }
    }
}

fn winning_player_score(deck1: &[u32], deck2: &[u32]) -> u32 {
    let mut deck1: VecDeque<u32> = VecDeque::from(deck1.to_vec());
    let mut deck2: VecDeque<u32> = VecDeque::from(deck2.to_vec());

    play(&mut deck1, &mut deck2);

    let winning_deck = if deck1.is_empty() { &deck2 } else { &deck1 };

    winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, e)| acc + (u32::try_from(i).unwrap() + 1) * e)
}

fn part2(deck1: &[u32], deck2: &[u32]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (deck1, deck2) = build(&input);

    println!("Part 1: {}", winning_player_score(&deck1, &deck2));
    println!("Part 2: {}", part2(&deck1, &deck2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (deck1, deck2) = build(INPUT_TEST);
        assert_eq!(winning_player_score(&deck1, &deck2), 306);
    }

    #[test]
    fn test_part2() {
        let (deck1, deck2) = build(INPUT_TEST);
        assert_eq!(part2(&deck1, &deck2), 0);
    }
}
