use std::{
    cmp::Ordering,
    io::{self, Read},
};

use regex::Regex;

#[inline]
fn int<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    const ORIGIN: Pos = Pos { x: 0, y: 0, z: 0 };

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

impl Nanobot {
    fn contains(&self, p: &Pos) -> bool {
        self.pos.distance(p) <= self.range
    }
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
    bots.iter().filter(|b| strongest.contains(&b.pos)).count()
}

fn borders(bots: &[Nanobot]) -> (Pos, Pos) {
    // Not using iterator min / max to keep only one loop.
    let mut min_pos = Pos::new(i32::MAX, i32::MAX, i32::MAX);
    let mut max_pos = Pos::new(i32::MIN, i32::MIN, i32::MIN);
    for Nanobot { pos, range: _ } in bots {
        min_pos.x = min_pos.x.min(pos.x);
        max_pos.x = max_pos.x.max(pos.x);
        min_pos.y = min_pos.y.min(pos.y);
        max_pos.y = max_pos.y.max(pos.y);
        min_pos.z = min_pos.z.min(pos.z);
        max_pos.z = max_pos.z.max(pos.z);
    }
    (min_pos, max_pos)
}

// Inspired from
// https://www.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecddus1/
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn dist_to_closest_to_most(bots: &[Nanobot]) -> u32 {
    // We take a big cube that covers the whole area. We divide that cube in 8 smaller cubes,
    // and find the cube that has the best candidate. We select that cube and do the process again,
    // dividing it into 8 etc.

    // This is our search area, covering the whole space to start.
    let (mut search_min, mut search_max) = borders(bots);

    // This is our search resolution, big enough to cover the whole cube.
    // It needs to be a power of 2, so that we can divided it cleanly by 2, getting one at the end.
    let mut resolution: u32 = 1;
    while resolution < (search_max.x - search_min.x) as u32
        || resolution < (search_max.y - search_min.y) as u32
        || resolution < (search_max.z - search_min.z) as u32
    {
        resolution *= 2;
    }

    loop {
        let mut best_count = 0;
        let mut best_pos = Pos::ORIGIN;

        for x in (search_min.x..=search_max.x).step_by(resolution as usize) {
            for y in (search_min.y..=search_max.y).step_by(resolution as usize) {
                for z in (search_min.z..=search_max.z).step_by(resolution as usize) {
                    let pos = Pos::new(x, y, z);
                    let bots_count = bots
                        .iter()
                        .filter(|b| b.pos.distance(&pos) / resolution <= b.range / resolution)
                        .count();
                    match bots_count.cmp(&best_count) {
                        Ordering::Greater => {
                            // bigger than what we had
                            best_count = bots_count;
                            best_pos = pos;
                        }
                        Ordering::Equal => {
                            // equal than what we had, pick closest to 0
                            if Pos::ORIGIN.distance(&pos) < Pos::ORIGIN.distance(&best_pos) {
                                best_pos = pos;
                            }
                        }
                        Ordering::Less => {}
                    }
                }
            }
        }

        // When resolution is 1, our cube is as small as possible and we found the answer.
        if resolution == 1 {
            return Pos::ORIGIN.distance(&best_pos);
        }

        // Create a new cube based on the best position found so far.
        search_min.x = best_pos.x - resolution as i32;
        search_max.x = best_pos.x + resolution as i32;
        search_min.y = best_pos.y - resolution as i32;
        search_max.y = best_pos.y + resolution as i32;
        search_min.z = best_pos.z - resolution as i32;
        search_max.z = best_pos.z + resolution as i32;

        resolution /= 2;
    }
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
        assert_eq!(dist_to_closest_to_most(&bots), 36);
    }
}
