use std::{
    cmp::Ordering,
    io::{self, Read},
};

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

// Initial brute-force version.
// Sort the items by quality. Starting with the biggest quality, try to create all possible combinations.
// Find the best one.
// Then drop that item from the list and go on.
#[allow(dead_code)]
fn optimal_combination_brute<const MAX_COST: u32>(items: &[Item]) -> u32 {
    let mut items = items.to_vec();
    items.sort_by_key(|i| i.quality);
    // Alternatively sorting by decreasing cost is too slow for real input.

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

    best_total_quality * smallest_uniq_mat_sum
}

// Optimized version: Solving the 0/1 Knapsack Problem by using dynamic programming.
fn optimal_combination(items: &[Item], max_cost: usize) -> u32 {
    // An entry opt_qual_mat[i][w] will store the maximum quality / least unique materials that can be obtained
    // using the first i items with a total cost of at most w.
    let mut opt_qual_mat = vec![vec![(0, 0); max_cost + 1]; items.len() + 1];

    // We leave the first row and column initialized with 0.

    for i in 1..=items.len() {
        for c in 1..=max_cost {
            if items[i - 1].cost as usize > c {
                // The current item's cost is greater than the current capacity c.
                // We can't include item i, the max is the same as the max without this item.
                opt_qual_mat[i][c] = opt_qual_mat[i - 1][c];
            } else {
                // The current item's cost is less than or equal to the current capacity c.

                // If we don't include the item, the value is the same as the max without this item.
                let dont_include = opt_qual_mat[i - 1][c];

                // If we include it, the value is
                //   the value of the current item
                // plus
                //   the maximum value from the remaining items with the remaining cost capacity.
                let include_quality =
                    items[i - 1].quality + opt_qual_mat[i - 1][c - items[i - 1].cost as usize].0;
                let include_uniq_mat =
                    items[i - 1].materials + opt_qual_mat[i - 1][c - items[i - 1].cost as usize].1;

                // Take the maximum of these two options.
                match dont_include.0.cmp(&include_quality) {
                    Ordering::Greater => opt_qual_mat[i][c] = dont_include,
                    Ordering::Less => opt_qual_mat[i][c] = (include_quality, include_uniq_mat),
                    Ordering::Equal => {
                        if dont_include.1 < include_uniq_mat {
                            opt_qual_mat[i][c] = dont_include;
                        } else {
                            opt_qual_mat[i][c] = (include_quality, include_uniq_mat);
                        }
                    }
                }
            }
        }
    }
    let best = opt_qual_mat[items.len()][max_cost];
    best.0 * best.1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let items = build(&input);

    println!("Part 1: {}", five_highest_uniq_mat(&items));

    // println!("Part 2 brute force: {}", optimal_combination_brute::<30>(&items));
    // Too slow:
    // println!("Part 3: {}", optimal_combination_brute::<300>(&items));

    println!("Part 2: {}", optimal_combination(&items, 30));
    println!("Part 3: {}", optimal_combination(&items, 300));
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
    fn test_part2_3_brute_force() {
        let items = build(&INPUT_TEST);
        assert_eq!(optimal_combination_brute::<30>(&items), 8256);
        assert_eq!(optimal_combination_brute::<150>(&items), 59388);
    }

    #[test]
    fn test_part2() {
        let items = build(&INPUT_TEST);
        assert_eq!(optimal_combination(&items, 30), 8256);
    }

    #[test]
    fn test_part3() {
        let items = build(&INPUT_TEST);
        assert_eq!(optimal_combination(&items, 150), 59388);
    }
}
