use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};
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
    cache: &mut FxHashMap<Vec<char>, bool>,
) -> bool {
    // Memoization is a must here, otherwise impossible patterns never finish.
    if let Some(val) = cache.get(pattern) {
        return *val;
    }

    if pattern.is_empty() {
        return true;
    }
    let limit = max_towel_size.min(pattern.len());
    let mut result = false;
    // We first check the biggest chunks possible, which is a small optimization (doesn't change very much).
    for i in (min_towel_size..=limit).rev() {
        let extract = &pattern[0..i];
        if towels.contains(extract) {
            result |=
                is_pattern_possible(towels, min_towel_size, max_towel_size, &pattern[i..], cache);

            // If we find that the rest of the pattern is possible, there is no need to check for other towel sizes.
            // It's another small optimization.
            if result {
                break;
            }
        }
    }

    cache.insert(pattern.to_vec(), result);
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

    let mut cache: FxHashMap<Vec<char>, bool> = FxHashMap::default();
    patterns
        .iter()
        .filter(|pattern| is_pattern_possible(towels, min, max, pattern, &mut cache))
        .count()
}

fn different_ways(
    towels: &FxHashSet<Vec<char>>,
    min_towel_size: usize,
    max_towel_size: usize,
    pattern: &[char],
    cache: &mut FxHashMap<Vec<char>, usize>,
) -> usize {
    // Memoization is a must here, otherwise impossible patterns never finish.
    if let Some(val) = cache.get(pattern) {
        return *val;
    }

    if pattern.is_empty() {
        return 1;
    }
    let limit = max_towel_size.min(pattern.len());
    let mut result = 0;
    // We first check the biggest chunks possible, which is a small optimization (doesn't change very much).
    for i in (min_towel_size..=limit).rev() {
        let extract = &pattern[0..i];
        if towels.contains(extract) {
            result += different_ways(towels, min_towel_size, max_towel_size, &pattern[i..], cache);
        }
    }

    cache.insert(pattern.to_vec(), result);
    result
}

fn different_ways_total(towels: &FxHashSet<Vec<char>>, patterns: &[Vec<char>]) -> usize {
    let (min, max) = get_towel_size_minmax(towels);

    let mut cache: FxHashMap<Vec<char>, usize> = FxHashMap::default();
    patterns
        .iter()
        .map(|pattern| different_ways(towels, min, max, pattern, &mut cache))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (towels, patterns) = build(&input);

    println!("Part 1: {}", possible_designs_count(&towels, &patterns));
    println!("Part 2: {}", different_ways_total(&towels, &patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_pattern_possible() {
        let towels = build_towels("r, wr, b, g, bwu, rb, gb, br");
        let (min, max) = get_towel_size_minmax(&towels);
        let mut cache: FxHashMap<Vec<char>, bool> = FxHashMap::default();

        let mut check = |p| is_pattern_possible(&towels, min, max, &build_pattern(p), &mut cache);
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
        assert_eq!(different_ways_total(&towels, &patterns), 16);
    }
}
