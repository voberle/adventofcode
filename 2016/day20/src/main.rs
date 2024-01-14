use core::fmt::Debug;
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::{self, Display},
    io::{self, Read},
};

/// Range.
/// start is inclusive, end exclusive.
///
/// Using our own range implementation, as Rust's one isn't very powerfull
/// https://kaylynn.gay/blog/post/rust_ranges_and_suffering
#[derive(Debug)]
struct Range<T>
where
    T: PartialOrd + Copy + Default + Display,
{
    start: T,
    end: T,
}

impl<T> Range<T>
where
    T: PartialOrd + Copy + Default + Display,
{
    fn new(start: T, end: T) -> Self {
        assert!(start < end);
        Self { start, end }
    }

    fn contains(&self, val: T) -> bool {
        self.start <= val && val <= self.end
    }

    /// Check if self is subset of other
    fn is_subset_of(&self, other: &Range<T>) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

// Conversion from Rust ranges.
impl<T> From<std::ops::Range<T>> for Range<T>
where
    T: PartialOrd + Copy + Default + Display,
{
    fn from(item: std::ops::Range<T>) -> Self {
        Range::new(item.start, item.end)
    }
}

// impl<T> From<std::ops::RangeInclusive<T>> for Range<T> where T: PartialOrd + Copy + Default + Display + std::ops::Add<T, Output = T> {
//     fn from(item: std::ops::RangeInclusive<T>) -> Self {
//         Range::new(*item.start(), *item.end() + T::from(1))
//     }
// }

impl<T> fmt::Display for Range<T>
where
    T: PartialOrd + Copy + Default + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

/// Takes a list of ranges and simplifies it into an ordered non-overlapping list.
fn simplify_ranges<T>(ranges: &[Range<T>]) -> Vec<Range<T>>
where
    T: PartialOrd + Copy + Default + Debug + Display,
{
    // Algorith from https://cs.stackexchange.com/a/106978
    // Works only with ranges that have inclusive start and exclusive end.
    // List of beginning and ending points. The boolean tells the type (true is start, false is end).
    let mut positions: Vec<(bool, T)> = ranges
        .iter()
        .flat_map(|r| [(true, r.start), (false, r.end)])
        .collect();
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
    let mut simplified_ranges: Vec<Range<T>> = Vec::new();
    let mut start: T = T::default();
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
                simplified_ranges.push(Range::new(start, pos));
            }
            c -= 1;
        }
    }
    simplified_ranges
}

fn build(input: &str) -> Vec<Range<u64>> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<u64> = line.split('-').map(|v| v.parse().unwrap()).collect();
            Range::<u64>::from(parts[0]..parts[1] + 1)
        })
        .collect()
}

fn lowest_allowed_ip(blocked_ips: &[Range<u64>]) -> u64 {
    let simplified = simplify_ranges(blocked_ips);
    // println!("Range count: {}", simplified.len());
    // for r in simplified {
    //     println!("{}", r);
    // }
    simplified.first().expect("No ranges found").end
}

fn part2(blocked_ips: &[Range<u64>]) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let blocked_ips = build(&input);

    println!("Part 1: {}", lowest_allowed_ip(&blocked_ips));
    println!("Part 2: {}", part2(&blocked_ips));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(lowest_allowed_ip(&build(INPUT_TEST)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
