use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    for (e1, e2) in input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|e| e.parse().unwrap())
            .collect_tuple()
            .unwrap()
    }) {
        list1.push(e1);
        list2.push(e2);
    }
    list1.sort_unstable();
    list2.sort_unstable();
    (list1, list2)
}

fn total_distance(list1: &[u32], list2: &[u32]) -> u32 {
    list1
        .iter()
        .zip(list2.iter())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum()
}

fn part2(list1: &[u32], list2: &[u32]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (list1, list2) = build(&input);

    println!("Part 1: {}", total_distance(&list1, &list2));
    println!("Part 2: {}", part2(&list1, &list2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (list1, list2) = build(INPUT_TEST);
        assert_eq!(total_distance(&list1, &list2), 11);
    }

    #[test]
    fn test_part2() {
        let (list1, list2) = build(INPUT_TEST);
        assert_eq!(part2(&list1, &list2), 0);
    }
}
