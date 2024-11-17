use std::io::{self, Read};

use md5::Digest;
use rayon::prelude::*;

#[inline]
fn starts_with(digest: &Digest, start: &str) -> bool {
    format!("{digest:x}").starts_with(start)
}

fn find_lowest_number<const MAX: u32>(secret_key: &str, start: &str) -> u32 {
    (1..MAX)
        .into_par_iter()
        .filter_map(|n| {
            let digest = md5::compute(format!("{secret_key}{n}").as_bytes());
            if starts_with(&digest, start) {
                Some(n)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn part1(input: &str) -> u32 {
    find_lowest_number::<10_000_000>(input, "00000")
}

fn part2(input: &str) -> u32 {
    find_lowest_number::<10_000_000>(input, "000000")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
