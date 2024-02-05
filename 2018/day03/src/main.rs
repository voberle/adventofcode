use std::io::{self, Read};

use regex::Regex;

#[inline]
fn int<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}

struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn build(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let p = re.captures(line).unwrap();
            Claim {
                id: int(&p[1]),
                left: int(&p[2]),
                top: int(&p[3]),
                width: int(&p[4]),
                height: int(&p[5]),
            }
        })
        .collect()
}

fn overlaping_fabric<const SQUARE_SIDE: usize>(claims: &[Claim]) -> usize {
    let pos = |x, y| x * SQUARE_SIDE + y;
    let mut big_square: Vec<usize> = vec![0; SQUARE_SIDE * SQUARE_SIDE];
    for claim in claims {
        for x in claim.top..claim.top + claim.height {
            for y in claim.left..claim.left + claim.width {
                big_square[pos(x, y)] += 1;
            }
        }
    }
    big_square.iter().filter(|v| **v > 1).count()
}

fn part2(claims: &[Claim]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let claims = build(&input);

    println!("Part 1: {}", overlaping_fabric::<1000>(&claims));
    println!("Part 2: {}", part2(&claims));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(overlaping_fabric::<10>(&build(INPUT_TEST)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
