use std::io::{self, Read};

use fxhash::FxHashMap;

fn build(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

#[allow(clippy::maybe_infinite_iter)]
fn digits_count(s: u64) -> usize {
    (0..).take_while(|i| 10u64.pow(*i) <= s).count()
}

fn split(mut s: u64, digits_count: usize) -> (u64, u64) {
    let half_digits_count = digits_count / 2;

    let get_half = |s: &mut u64| -> u64 {
        (0..half_digits_count)
            .map(|p| {
                let d = *s % 10;
                *s /= 10;
                d * 10u64.pow(u32::try_from(p).unwrap())
            })
            .sum()
    };

    let right = get_half(&mut s);
    let left = get_half(&mut s);
    (left, right)
}

fn blink(stones: &FxHashMap<u64, usize>) -> FxHashMap<u64, usize> {
    let mut new_stones = FxHashMap::default();
    for (&s, &cnt) in stones {
        let digits_count = digits_count(s);
        if s == 0 {
            *new_stones.entry(1).or_default() += cnt;
        } else if digits_count.is_multiple_of(2) {
            let (left, right) = split(s, digits_count);
            *new_stones.entry(left).or_default() += cnt;
            *new_stones.entry(right).or_default() += cnt;
        } else {
            *new_stones.entry(s * 2024).or_default() += cnt;
        }
    }
    new_stones
}

fn stones_list_to_map(stones: &[u64]) -> FxHashMap<u64, usize> {
    let mut stones_map: FxHashMap<u64, usize> = FxHashMap::default();
    for &s in stones {
        *stones_map.entry(s).or_default() += 1;
    }
    stones_map
}

fn stones_count(stones: &[u64], blink_count: usize) -> usize {
    let mut stones_map = stones_list_to_map(stones);
    for _b in 0..blink_count {
        stones_map = blink(&stones_map);
    }
    stones_map.values().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stones = build(input.trim());

    println!("Part 1: {}", stones_count(&stones, 25));
    println!("Part 2: {}", stones_count(&stones, 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_split() {
        assert_eq!(split(1234, 4), (12, 34));
        assert_eq!(split(1000, 4), (10, 0));
    }

    #[test]
    fn test_blink() {
        let stones = build("0 1 10 99 999");
        assert_eq!(
            blink(&stones_list_to_map(&stones)),
            stones_list_to_map(&build("1 2024 1 0 9 9 2021976"))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(stones_count(&build(INPUT_TEST), 6), 22);
        assert_eq!(stones_count(&build(INPUT_TEST), 25), 55312);
    }
}
