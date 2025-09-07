use std::io::{self, Read};

use once_cell::sync::Lazy;
use regex::Regex;

#[inline]
fn int<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Item {
    id: usize,
    code: String,
    quality: u32,
    cost: u32,
    materials: u32,
}

impl Item {
    fn build(line: &str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(\d+) (\w+) \| Quality : (\d+), Cost : (\d+), Unique Materials : (\d+)")
                .unwrap()
        });

        let p = RE.captures(line).unwrap();
        Self {
            id: int(&p[1]),
            code: p[2].to_string(),
            quality: int(&p[3]),
            cost: int(&p[4]),
            materials: int(&p[5]),
        }
    }
}

fn build(input: &str) -> Vec<Item> {
    input.lines().map(Item::build).collect()
}

fn five_highest_uniq_mat(items: &[Item]) -> u32 {
    let mut items = items.to_vec();
    items.sort_by_key(|i| i.quality * 1000 + i.cost);

    items[items.len() - 5..].iter().map(|i| i.materials).sum()
}

fn make_combi<const MAX_COST: u32>(
    remaining_items: &[Item],
    cost: u32,
    quality_total: u32,
    uniq_mat_total: u32,
    best_total_quality: &mut u32,
    smallest_uniq_mat_sum: &mut u32,
) {
    let mut items = remaining_items.to_vec();
    while let Some(q) = items.pop() {
        let new_cost = cost + q.cost;
        if new_cost > MAX_COST {
            continue;
        }

        let new_quality_total = quality_total + q.quality;
        let new_uniq_mat_total = uniq_mat_total + q.materials;
        if new_quality_total > *best_total_quality {
            *best_total_quality = new_quality_total;
            *smallest_uniq_mat_sum = new_uniq_mat_total;
        } else if new_quality_total == *best_total_quality
            && new_uniq_mat_total < *smallest_uniq_mat_sum
        {
            *smallest_uniq_mat_sum = new_uniq_mat_total;
        }

        make_combi::<MAX_COST>(
            &items,
            new_cost,
            new_quality_total,
            new_uniq_mat_total,
            best_total_quality,
            smallest_uniq_mat_sum,
        );
    }
}

fn optimal_combination_brute<const MAX_COST: u32>(items: &[Item]) -> u32 {
    // Sort the items by quality. Starting with the biggest quality, try to create all possible combinations.
    // Find the best one.
    // Then drop that item from the list and go on.

    let mut items = items.to_vec();
    items.sort_by_key(|i| i.quality);

    // Alternatively sorting by decreasing cost is too slow for real input.
    // items.sort_by_key(|i| i.cost);
    // items.reverse();

    let mut best_total_quality = 0;
    let mut smallest_uniq_mat_sum = 0;

    while let Some(q) = items.pop() {
        make_combi::<MAX_COST>(
            &items,
            q.cost,
            q.quality,
            q.materials,
            &mut best_total_quality,
            &mut smallest_uniq_mat_sum,
        );
    }

    // println!("best quality={best_total_quality}, smallest uniq mat sum={smallest_uniq_mat_sum}");
    best_total_quality * smallest_uniq_mat_sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let items = build(&input);

    println!("Part 1: {}", five_highest_uniq_mat(&items));
    println!("Part 2: {}", optimal_combination_brute::<30>(&items));
    // Too slow:
    // println!("Part 3: {}", optimal_combination_brute::<300>(&items));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let items = build(&INPUT_TEST);
        assert_eq!(five_highest_uniq_mat(&items), 90);
    }

    #[test]
    fn test_part2() {
        let items = build(&INPUT_TEST);
        assert_eq!(optimal_combination_brute::<30>(&items), 8256);
    }

    #[test]
    fn test_part3_brute_force() {
        let items = build(&INPUT_TEST);
        assert_eq!(optimal_combination_brute::<150>(&items), 59388);
    }
}
