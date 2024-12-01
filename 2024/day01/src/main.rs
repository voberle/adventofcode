use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn total_distance(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .sorted_unstable()
        .zip(right_list.iter().sorted_unstable())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum()
}

fn similarity_score(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .map(|left_elt| {
            let count = right_list
                .iter()
                .filter(|right_elt| *right_elt == left_elt)
                .count();
            left_elt * u32::try_from(count).unwrap()
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (left_list, right_list) = build(&input);

    println!("Part 1: {}", total_distance(&left_list, &right_list));
    println!("Part 2: {}", similarity_score(&left_list, &right_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (left_list, right_list) = build(INPUT_TEST);
        assert_eq!(total_distance(&left_list, &right_list), 11);
    }

    #[test]
    fn test_part2() {
        let (left_list, right_list) = build(INPUT_TEST);
        assert_eq!(similarity_score(&left_list, &right_list), 31);
    }
}
