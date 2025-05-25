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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let items = build(&input);

    println!("Part 1: {}", five_highest_uniq_mat(&items));
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
}
