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

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

#[derive(Debug)]
struct Nanobot {
    pos: Pos,
    range: u32,
}

fn build(input: &str) -> Vec<Nanobot> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let p = re.captures(line).unwrap();
            Nanobot {
                pos: Pos {
                    x: int(&p[1]),
                    y: int(&p[2]),
                    z: int(&p[3]),
                },
                range: int(&p[4]),
            }
        })
        .collect()
}

fn bots_in_range_of_strongest(bots: &[Nanobot]) -> usize {
    let strongest = bots.iter().max_by_key(|b| b.range).unwrap();
    bots.iter()
        .filter(|b| b.pos.distance(&strongest.pos) <= strongest.range)
        .count()
}

fn closest_to_most(bots: &[Nanobot]) -> Pos {
    Pos::new(0, 0, 0)
}

fn dist_to_closest_to_most(bots: &[Nanobot]) -> u32 {
    let closest = closest_to_most(bots);
    closest.distance(&Pos::new(0, 0, 0))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bots = build(&input);

    println!("Part 1: {}", bots_in_range_of_strongest(&bots));
    println!("Part 2: {}", dist_to_closest_to_most(&bots));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(bots_in_range_of_strongest(&build(INPUT_TEST_1)), 7);
    }

    #[test]
    fn test_part2() {
        let bots = build(INPUT_TEST_2);
        assert_eq!(closest_to_most(&bots), Pos::new(12, 12, 12));
        assert_eq!(dist_to_closest_to_most(&bots), 36);
    }
}
