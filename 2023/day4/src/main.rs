// https://adventofcode.com/2023/day/4
// Part 1 test: 13
// Part 2 test: 30

use std::{collections::HashSet, io, usize, ops::AddAssign};

fn set_or_inc<T: AddAssign>(v: &mut Vec<T>, i: usize, val: T) {
    if v.get(i).is_some() {
        v[i] += val;
    } else {
        v.insert(i, val);
    }
}

fn main() {
    let mut total = 0;
    let mut copies_count: Vec<u32> = Vec::new();
    let mut i = 0;
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    for l in io::stdin().lines() {
        let line = l.unwrap();
        let sets: Vec<HashSet<u32>> = line[line.find(": ").unwrap() + 1..]
            .split(" | ")
            .map(|s| {
                s.split_whitespace() // better than split(" ") as this handles multiple spaces
                    .map(|n| n.parse().unwrap())
                    .collect::<HashSet<u32>>()
            })
            .collect();
        let winning_number_count = sets[0].intersection(&sets[1]).count();

        set_or_inc(&mut copies_count, i, 1);
        for k in 0..winning_number_count {
            let val = copies_count[i];
            set_or_inc(&mut copies_count, i + k + 1, val);
        }
        if winning_number_count > 0 {
            total += 2_u32.pow(winning_number_count as u32 - 1);
        }
        i += 1;
    }
    println!("Part 1: {}", total);
    println!("Part 2: {}", copies_count.iter().sum::<u32>());
}
