use std::{
    io::{self, Read},
    ops::AddAssign,
};

use fxhash::FxHashSet;

fn set_or_inc<T: AddAssign + Default>(v: &mut Vec<T>, i: usize, val: T) {
    if i >= v.len() {
        v.resize_with(i + 1, Default::default);
    }
    v[i] += val;
}

fn analyze(input: &str) -> (usize, usize) {
    let mut total = 0;
    let mut copies_count: Vec<usize> = Vec::new();

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    for (i, line) in input.lines().enumerate() {
        let sets: Vec<FxHashSet<u32>> = line[line.find(": ").unwrap() + 1..]
            .split(" | ")
            .map(|s| {
                s.split_whitespace() // better than split(" ") as this handles multiple spaces
                    .map(|n| n.parse().unwrap())
                    .collect::<FxHashSet<u32>>()
            })
            .collect();
        let winning_number_count = sets[0].intersection(&sets[1]).count();

        set_or_inc(&mut copies_count, i, 1);
        for k in 0..winning_number_count {
            let val = copies_count[i];
            set_or_inc(&mut copies_count, i + k + 1, val);
        }
        if winning_number_count > 0 {
            total += 2_usize.pow(u32::try_from(winning_number_count).unwrap() - 1);
        }
    }
    let total_scratchpads = copies_count.iter().sum();
    (total, total_scratchpads)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (total_points, total_scratchpads) = analyze(input.as_str());

    println!("Part 1: {}", total_points);
    println!("Part 2: {}", total_scratchpads);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1() {
        let (total_points, _) = analyze(INPUT_TEST);
        assert_eq!(total_points, 13);
    }

    #[test]
    fn test_part2() {
        let (_, total_scratchpads) = analyze(INPUT_TEST);
        assert_eq!(total_scratchpads, 30);
    }
}
