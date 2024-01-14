use std::{
    cmp::{Ordering, PartialOrd},
    io::{self, Read},
};

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

fn build(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line.split('-').map(|v| v.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn lowest_allowed_ip(blocked_ips: &[(u32, u32)]) -> u32 {
    // We need exclusive ranges for the simplify method, so we need to use u64 as u32 + 1 may overflow.
    let ranges_excl: Vec<(u64, u64)> = blocked_ips
        .iter()
        .map(|r| (r.0 as u64, r.1 as u64 + 1))
        .collect();

    let simplified = simplify_ranges(&ranges_excl);
    simplified.first().expect("No ranges found").1 as u32
}

fn part2(blocked_ips: &[(u32, u32)]) -> u32 {
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
    fn test_simplify_ranges() {
        let ranges = [(5, 9), (0, 3), (4, 8)];
        assert_eq!(simplify_ranges(&ranges), &[(0, 3), (4, 9)]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(lowest_allowed_ip(&build(INPUT_TEST)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
