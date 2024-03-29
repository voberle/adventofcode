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

#[derive(Debug, Clone)]
struct Particule {
    nb: usize,
    p: (i32, i32, i32),
    v: (i32, i32, i32),
    a: (i32, i32, i32),
}

impl Particule {
    fn distance_to_zero(&self) -> i32 {
        self.p.0.abs() + self.p.1.abs() + self.p.2.abs()
    }

    fn tick(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }
}

impl PartialEq for Particule {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Eq for Particule {}

impl Ord for Particule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.p.cmp(&self.p)
    }
}

impl PartialOrd for Particule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn build(input: &str) -> Vec<Particule> {
    let re = Regex::new(r"p=<(\-?\d+),(\-?\d+),(\-?\d+)>, v=<(\-?\d+),(\-?\d+),(\-?\d+)>, a=<(\-?\d+),(\-?\d+),(\-?\d+)>").unwrap();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let p = re.captures(line).unwrap();
            Particule {
                nb: i,
                p: (int(&p[1]), int(&p[2]), int(&p[3])),
                v: (int(&p[4]), int(&p[5]), int(&p[6])),
                a: (int(&p[7]), int(&p[8]), int(&p[9])),
            }
        })
        .collect()
}

fn particule_closest_to_zero(particules: &[Particule]) -> usize {
    let mut particules = particules.to_vec();
    // things seem to stabilize above 400
    for _ in 0..1000 {
        particules.iter_mut().for_each(Particule::tick);
    }
    particules
        .iter()
        .min_by_key(|p| p.distance_to_zero())
        .unwrap()
        .nb
}

fn particules_left_after_collisions(particules: &[Particule]) -> usize {
    let mut particules = particules.to_vec();
    for _ in 0..1000 {
        particules.iter_mut().for_each(Particule::tick);

        // Remove the ones that collide i.e. are on the same position.
        particules.sort();
        let colliding_particules_nb: Vec<usize> = particules
            .windows(2)
            .flat_map(|p| {
                if p[0] == p[1] {
                    vec![p[0].nb, p[1].nb]
                } else {
                    vec![]
                }
            })
            .collect();
        particules.retain(|p| !colliding_particules_nb.contains(&p.nb));
    }
    particules.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let particules = build(&input);

    println!("Part 1: {}", particule_closest_to_zero(&particules));
    println!("Part 2: {}", particules_left_after_collisions(&particules));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(particule_closest_to_zero(&build(INPUT_TEST_1)), 0);
    }

    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part2() {
        assert_eq!(particules_left_after_collisions(&build(INPUT_TEST_2)), 1);
    }
}
