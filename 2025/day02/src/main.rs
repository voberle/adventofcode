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

    fn get_invalid_ids(&self) -> Vec<u64> {
        (self.first..=self.last)
            .filter(|n| {
                // Using a vector of chars, as sub-slices can easily be compared.
                let s: Vec<_> = n.to_string().chars().collect();
                let middle = s.len() / 2;
                s[0..middle] == s[middle..]
            })
            .collect()
    }
}

fn build(input: &str) -> Vec<Range> {
    input.split(',').map(Range::build).collect()
}

fn invalid_ids_sum(ranges: &[Range]) -> u64 {
    ranges.iter().flat_map(Range::get_invalid_ids).sum()
}

fn part2(ranges: &[Range]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ranges = build(&input);

    println!("Part 1: {}", invalid_ids_sum(&ranges));
    println!("Part 2: {}", part2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(invalid_ids_sum(&build(INPUT_TEST)), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
