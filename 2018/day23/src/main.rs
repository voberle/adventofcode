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

#[derive(Debug)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    range: u32,
}

impl Nanobot {
    fn distance(&self, other: &Nanobot) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

fn build(input: &str) -> Vec<Nanobot> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let p = re.captures(line).unwrap();
            Nanobot {
                x: int(&p[1]),
                y: int(&p[2]),
                z: int(&p[3]),
                range: int(&p[4]),
            }
        })
        .collect()
}

fn bots_in_range_of_strongest(bots: &[Nanobot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.range).unwrap();
    bots.iter()
        .filter(|b| b.distance(strongest) <= strongest.range)
        .count()
}

fn part2(bots: &[Nanobot]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bots = build(&input);

    println!("Part 1: {}", bots_in_range_of_strongest(&bots));
    println!("Part 2: {}", part2(&bots));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(bots_in_range_of_strongest(&build(INPUT_TEST)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
