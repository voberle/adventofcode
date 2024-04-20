use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[allow(dead_code)]
fn cups_to_string(cups: &[u32], current: usize) -> String {
    cups.iter()
        .enumerate()
        .map(|(i, c)| {
            if i == current {
                format!("({})", c)
            } else {
                c.to_string()
            }
        })
        .join(" ")
}

fn cups_after_1(cups: &[u32], moves: usize) -> String {
    let mut cups = cups.to_vec();
    let min_cup = *cups.iter().min().unwrap();
    let max_cup = *cups.iter().max().unwrap();

    let mut current: usize = 0;
    for _m in 0..moves {
        // println!("-- move {} --", _m + 1);
        // println!("cups: {}", cups_to_string(&cups, current));

        let current_cup = cups[current];

        // Pick up 3 cups.
        let picks = [
            cups[(current + 1).rem_euclid(cups.len())],
            cups[(current + 2).rem_euclid(cups.len())],
            cups[(current + 3).rem_euclid(cups.len())],
        ];
        cups.retain(|v| !picks.contains(v));
        // println!("pick up: {:?}", picks);

        // Cup label to use for the destination cup
        let mut destination_cup = current_cup - 1;
        if destination_cup < min_cup {
            destination_cup = max_cup;
        }
        loop {
            if let Some(dest_cup_pos) = cups.iter().position(|c| *c == destination_cup) {
                // println!("destination: {}", destination_cup);

                // Insert picked up cups after destination.
                cups.insert(dest_cup_pos + 1, picks[0]);
                cups.insert(dest_cup_pos + 2, picks[1]);
                cups.insert(dest_cup_pos + 3, picks[2]);

                // Select new current cup.
                let current_cup_pos = cups.iter().position(|c| *c == current_cup).unwrap();
                current = (current_cup_pos + 1).rem_euclid(cups.len());

                break;
            }
            // Try to find next destination cup.
            destination_cup -= 1;
            if destination_cup < min_cup {
                destination_cup = max_cup;
            }
        }
    }

    // Return the list of cups after cup 1.
    cups.iter()
        .cycle()
        .skip_while(|v| **v != 1)
        .skip(1)
        .take(cups.len() - 1)
        .map(ToString::to_string)
        .collect()
}

fn part2(cups: &[u32]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cups = build(input.trim());

    // println!("Part 1: {}", cups_after_1(&cups, 10));

    println!("Part 1: {}", cups_after_1(&cups, 100));
    // println!("Part 2: {}", part2(&cups));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "389125467";

    #[test]
    fn test_part1() {
        assert_eq!(cups_after_1(&build(INPUT_TEST), 10), "92658374");
        assert_eq!(cups_after_1(&build(INPUT_TEST), 100), "67384529");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
