use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::Itertools;

fn build_towels(input: &str) -> FxHashSet<Vec<char>> {
    input.split(", ").map(|s| s.chars().collect()).collect()
}

fn build_pattern(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn build(input: &str) -> (FxHashSet<Vec<char>>, Vec<Vec<char>>) {
    let (towels, patterns) = input
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect_tuple()
        .unwrap();
    (
        build_towels(towels),
        patterns.lines().map(build_pattern).collect(),
    )
}

fn is_pattern_possible(
    towels: &FxHashSet<Vec<char>>,
    min_towel_size: usize,
    max_towel_size: usize,
    pattern: &[char],
) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let limit = max_towel_size.min(pattern.len());
    let mut result = false;
    // We first check the biggest chunks possible.
    // Does it matter??
    for i in (min_towel_size..=limit).rev() {
        let extract = &pattern[0..i];
        // println!("extract {:?}", extract);
        if towels.contains(extract) {
            // println!("towels contains {:?}. Remaining {:?}", extract, &pattern[i..]);
            // println!("towels contains {:?}. Remaining {}", extract, &pattern[i..].len());
            result |= is_pattern_possible(towels, min_towel_size, max_towel_size, &pattern[i..]);

            if result {
                break;
            }
        }
    }
    result
}

fn get_towel_size_minmax(towels: &FxHashSet<Vec<char>>) -> (usize, usize) {
    if let itertools::MinMaxResult::MinMax(min, max) =
        towels.iter().map(std::vec::Vec::len).minmax()
    {
        (min, max)
    } else {
        panic!("Couldn't find min max")
    }
}

fn possible_designs_count(towels: &FxHashSet<Vec<char>>, patterns: &[Vec<char>]) -> usize {
    let (min, max) = get_towel_size_minmax(towels);
    println!("min {min}, max {max}");
    patterns
        .iter()
        .filter(|pattern| {
            println!("Checking {:?}", pattern);
            is_pattern_possible(towels, min, max, pattern)
        })
        .count()
}

fn part2(towels: &FxHashSet<Vec<char>>, patterns: &[Vec<char>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (towels, patterns) = build(&input);

    println!("Part 1: {}", possible_designs_count(&towels, &patterns));
    println!("Part 2: {}", part2(&towels, &patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_pattern_possible() {
        let towels = build_towels("r, wr, b, g, bwu, rb, gb, br");
        let (min, max) = get_towel_size_minmax(&towels);
        let check = |p| is_pattern_possible(&towels, min, max, &build_pattern(p));
        assert!(check("brwrr"));
        assert!(check("bggr"));
        assert!(check("gbbr"));
        assert!(check("rrbgbr"));
        assert!(!check("ubwu"));
        assert!(check("bwurrg"));
        assert!(check("brgr"));
        assert!(!check("bbrgwb"));
    }

    #[test]
    fn test_part1() {
        let (towels, patterns) = build(INPUT_TEST);
        assert_eq!(possible_designs_count(&towels, &patterns), 6);
    }

    #[test]
    fn test_part2() {
        let (towels, patterns) = build(INPUT_TEST);
        assert_eq!(part2(&towels, &patterns), 0);
    }
}
