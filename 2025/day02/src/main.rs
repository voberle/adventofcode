use std::io::{self, Read};

struct Range {
    first: u64,
    last: u64,
}

impl Range {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split('-').map(|n| n.parse().unwrap()).collect();
        Self {
            first: parts[0],
            last: parts[1],
        }
    }

    fn rule1(n: u64) -> bool {
        // Using a vector of chars, as sub-slices can easily be compared.
        let s: Vec<_> = n.to_string().chars().collect();

        let middle = s.len() / 2;
        s[0..middle] == s[middle..]
    }

    fn rule2(n: u64) -> bool {
        let s: Vec<_> = n.to_string().chars().collect();

        // An ID is invalid if it is made only of some sequence of digits repeated at least twice.
        // So we take all slices of size 2 to half the sequence len, and check if they repeat.
        // If one of them repeats until the end, we have an invalid ID.
        (1..=s.len() / 2).any(|len| {
            // Pattern to check.
            let pattern = &s[0..len];

            (len..s.len()).step_by(len).all(|start| {
                // Handles the case when last slice we compare to is smaller.
                let end = (start + len).min(s.len());
                let check_against = &s[start..end];

                pattern == check_against
            })
        })
    }

    fn get_invalid_ids(&self, rule: fn(u64) -> bool) -> Vec<u64> {
        (self.first..=self.last).filter(|n| rule(*n)).collect()
    }
}

fn build(input: &str) -> Vec<Range> {
    input.split(',').map(Range::build).collect()
}

fn invalid_ids_sum_rule1(ranges: &[Range]) -> u64 {
    ranges
        .iter()
        .flat_map(|r| r.get_invalid_ids(Range::rule1))
        .sum()
}

fn invalid_ids_sum_rule2(ranges: &[Range]) -> u64 {
    // for r in ranges {
    //     let invalid_ids = r.get_invalid_ids(Range::rule2);
    //     println!("{}-{}: {:?}", r.first, r.last, invalid_ids);
    // }

    ranges
        .iter()
        .flat_map(|r| r.get_invalid_ids(Range::rule2))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ranges = build(&input);

    println!("Part 1: {}", invalid_ids_sum_rule1(&ranges));
    println!("Part 2: {}", invalid_ids_sum_rule2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_rule2() {
        assert!(Range::rule2(111));
        assert!(Range::rule2(824824824));
        assert!(!Range::rule2(2121212118));
    }

    #[test]
    fn test_get_invalid_ids() {
        let r = Range {
            first: 95,
            last: 115,
        };
        assert_eq!(r.get_invalid_ids(Range::rule2), [99, 111]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(invalid_ids_sum_rule1(&build(INPUT_TEST)), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(invalid_ids_sum_rule2(&build(INPUT_TEST)), 4174379265);
    }
}
