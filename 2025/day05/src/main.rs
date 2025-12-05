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

use std::cmp::Ordering;

/// Takes a list of ranges and simplifies it into an ordered non-overlapping list.
/// Works only with ranges that have inclusive start and exclusive end.
fn simplify_ranges<T>(ranges: &[(T, T)]) -> Vec<(T, T)>
where
    T: PartialOrd + Copy,
{
    // Algorith from https://cs.stackexchange.com/a/106978
    // List of beginning and ending points. The boolean tells the type (true is start, false is end).
    let mut positions: Vec<(bool, T)> = ranges
        .iter()
        .flat_map(|r| [(true, r.0), (false, r.1)])
        .collect();

    // Sort by position, with starting points comparing below ending points when positions are equal.
    positions.sort_by(|a, b| {
        if a.1 == b.1 {
            if a.0 && !b.0 {
                // a is start, b is end
                Ordering::Less
            } else if !a.0 && b.0 {
                // a is end, b is start
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    });
    assert!(positions.len() > 1 && positions.first().unwrap().0);

    let mut simplified_ranges = Vec::new();
    let mut start: T = positions.first().unwrap().1; // we need a value to initialize start with.
    let mut c = 0;
    for (pos_type, pos) in positions {
        if pos_type {
            // 0->1 transition
            if c == 0 {
                start = pos;
            }
            c += 1;
        } else {
            // 1->0 transition
            if c == 1 {
                let end = pos;
                simplified_ranges.push((start, end));
            }
            c -= 1;
        }
    }
    simplified_ranges
}

fn total_fresh_ids(fresh_ingredient_ranges: &[(u64, u64)]) -> u64 {
    let sanitized_ranges = simplify_ranges(fresh_ingredient_ranges);

    sanitized_ranges.iter().map(|r| r.1 - r.0 + 1).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (fresh_ingredient_ranges, available_ingredients) = build(&input);

    println!(
        "Part 1: {}",
        fresh_ingredients_count(&fresh_ingredient_ranges, &available_ingredients)
    );
    println!("Part 2: {}", total_fresh_ids(&fresh_ingredient_ranges));
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
        assert_eq!(total_fresh_ids(&fresh_ingredient_ranges), 14);
    }
}
