use std::io::{self, Read};

use md5::Digest;

fn starts_5_zeroes(digest: &Digest) -> bool {
    format!("{:x}", digest).starts_with("00000")
}

fn part1(secret_key: &str) -> u32 {
    let mut lowest = u32::MAX;
    for n in 1..10_000_000 {
        let s = format!("{}{}", secret_key, n);
        let digest = md5::compute(s.as_bytes());
        if starts_5_zeroes(&digest) {
            // println!("{:x}", digest);
            if n < lowest {
                lowest = n;
            }
        }
    }
    lowest
}

fn part2(input: &str) -> i64 {
    0
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
