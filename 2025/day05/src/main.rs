use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    (
        parts[0]
            .lines()
            .map(|range| {
                range
                    .split('-')
                    .map(|r| r.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        parts[1].lines().map(|id| id.parse().unwrap()).collect(),
    )
}

fn is_fresh(fresh_ingredient_ranges: &[(u64, u64)], ingredient_id: u64) -> bool {
    fresh_ingredient_ranges
        .iter()
        .any(|range| ingredient_id >= range.0 && ingredient_id <= range.1)
}

fn fresh_ingredients_count(
    fresh_ingredient_ranges: &[(u64, u64)],
    available_ingredients: &[u64],
) -> usize {
    available_ingredients
        .iter()
        .filter(|ingredient_id| is_fresh(fresh_ingredient_ranges, **ingredient_id))
        .count()
}

fn part2(fresh_ingredient_ranges: &[(u64, u64)], available_ingredients: &[u64]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (fresh_ingredient_ranges, available_ingredients) = build(&input);

    println!(
        "Part 1: {}",
        fresh_ingredients_count(&fresh_ingredient_ranges, &available_ingredients)
    );
    println!(
        "Part 2: {}",
        part2(&fresh_ingredient_ranges, &available_ingredients)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (fresh_ingredient_ranges, available_ingredients) = build(INPUT_TEST);
        assert_eq!(
            fresh_ingredients_count(&fresh_ingredient_ranges, &available_ingredients),
            3
        );
    }

    #[test]
    fn test_part2() {
        let (fresh_ingredient_ranges, available_ingredients) = build(INPUT_TEST);
        assert_eq!(part2(&fresh_ingredient_ranges, &available_ingredients), 0);
    }
}
