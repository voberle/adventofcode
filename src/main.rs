// https://adventofcode.com/2023/day/4
// Part 1 test: 13
// Part 1: 21158

use std::{collections::HashSet, io};

fn main() {
    let mut total = 0;
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    for line in io::stdin().lines() {
        let sets: Vec<HashSet<u32>> = line
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" | ")
            .map(|s| {
                s.split_whitespace() // better than split(" ") as this handles multiple spaces
                    .map(|n| n.parse().unwrap())
                    .collect::<HashSet<u32>>()
            })
            .collect();
        let winning_number_count = sets[0].intersection(&sets[1]).count();
        if winning_number_count > 0 {
            total += 2_u32.pow(winning_number_count as u32 - 1);
        }
    }
    println!("{}", total);
}
