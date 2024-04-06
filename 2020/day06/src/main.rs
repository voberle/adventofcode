use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::Itertools;

type Group = Vec<Vec<char>>;

fn build(input: &str) -> Vec<Group> {
    let mut groups = vec![Vec::new()];
    for line in input.lines() {
        if line.is_empty() {
            groups.push(Vec::new());
            continue;
        }
        groups.last_mut().unwrap().push(line.chars().collect());
    }
    groups
}

fn sum_of_counts_anyone(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|g| g.iter().flatten().sorted_unstable().dedup().count())
        .sum()
}

fn sum_of_counts_everyone(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(|g| {
            let mut set: FxHashSet<char> = g[0].iter().copied().collect();
            for p in g {
                let other_set: FxHashSet<char> = p.iter().copied().collect();
                set = set.intersection(&other_set).copied().collect();
            }
            set.len()
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let groups = build(&input);

    println!("Part 1: {}", sum_of_counts_anyone(&groups));
    println!("Part 2: {}", sum_of_counts_everyone(&groups));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(sum_of_counts_anyone(&build(INPUT_TEST)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_of_counts_everyone(&build(INPUT_TEST)), 6);
    }
}
