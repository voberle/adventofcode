use std::{
    fmt::Display,
    io::{self, Read},
};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
    z: i64,
    a: i64,
}

impl Coords {
    const fn new(x: i64, y: i64, z: i64, a: i64) -> Self {
        Self { x, y, z, a }
    }

    fn get_at(&self, time: i64, velocity: &Coords, space_dims: &Coords) -> Self {
        Self {
            x: (self.x + velocity.x * time).rem_euclid(space_dims.x),
            y: (self.y + velocity.y * time).rem_euclid(space_dims.y),
            z: (self.z + velocity.z * time).rem_euclid(space_dims.z),
            a: (self.a + 1 + velocity.a * time).rem_euclid(space_dims.a) - 1,
        }
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.a)
    }
}

struct Rule {
    _number: usize,
    x: i64,
    y: i64,
    z: i64,
    a: i64,
    divide: i64,
    remainder: i64,
    velocity: Coords,
}

impl Rule {
    fn build(line: &str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"RULE (\d+): (\d+)x\+(\d+)y\+(\d+)z\+(\d+)a DIVIDE (\d+) HAS REMAINDER (\d+) \| DEBRIS VELOCITY \((-?\d+), (-?\d+), (-?\d+), (-?\d+)\)")
                .unwrap()
        });

        let p = RE.captures(line).unwrap();
        Self {
            _number: p[1].parse().unwrap(),
            x: p[2].parse().unwrap(),
            y: p[3].parse().unwrap(),
            z: p[4].parse().unwrap(),
            a: p[5].parse().unwrap(),
            divide: p[6].parse().unwrap(),
            remainder: p[7].parse().unwrap(),
            velocity: Coords {
                x: p[8].parse().unwrap(),
                y: p[9].parse().unwrap(),
                z: p[10].parse().unwrap(),
                a: p[11].parse().unwrap(),
            },
        }
    }

    fn check_coords(&self, coords: &Coords) -> bool {
        let t = self.x * coords.x + self.y * coords.y + self.z * coords.z + self.a * coords.a;
        // For negative numbers we need to use rem_euclid, as % gives negative numbers, problem expects positive ones.
        self.remainder == t.rem_euclid(self.divide)
    }

    fn get_all_debris(&self, space_dims: &Coords) -> Vec<Coords> {
        // Space dimensions for a always go from -1 to 1
        assert_eq!(space_dims.a, 3);

        (0..space_dims.x)
            .flat_map(|x| {
                (0..space_dims.y).flat_map(move |y| {
                    (0..space_dims.z).flat_map(move |z| {
                        (-1..=1).filter_map(move |a| {
                            let c = Coords::new(x, y, z, a);
                            // println!("{}", c);
                            if self.check_coords(&c) { Some(c) } else { None }
                        })
                    })
                })
            })
            .collect()
    }
}

fn build(input: &str) -> Vec<Rule> {
    input.lines().map(Rule::build).collect()
}

fn total_debris(rules: &[Rule], space_dims: &Coords) -> usize {
    // Space dimensions for a always go from -1 to 1
    assert_eq!(space_dims.a, 3);

    rules
        .iter()
        .map(|rule| rule.get_all_debris(space_dims).len())
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rules = build(&input);

    println!(
        "Part 1: {}",
        total_debris(&rules, &Coords::new(10, 15, 60, 3))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_remainder() {
        let a: i32 = -5;
        let b: i32 = 9;

        assert_eq!(a % b, -5);
        assert_eq!(a.rem_euclid(b), 4);
    }

    #[test]
    fn test_coords_at() {
        let coords = Coords::new(3, 9, 1, -1);
        let velocity = Coords::new(1, -1, 0, 1);
        let space_dims = Coords::new(10, 15, 60, 3);
        assert_eq!(
            coords.get_at(3, &velocity, &space_dims),
            Coords::new(6, 6, 1, -1)
        );
    }

    #[test]
    fn test_rule_build() {
        let rule = Rule::build(
            "RULE 1: 8x+10y+3z+5a DIVIDE 9 HAS REMAINDER 4 | DEBRIS VELOCITY (0, -1, 0, 1)",
        );
        assert_eq!(rule._number, 1);
        assert_eq!(rule.y, 10);
        assert_eq!(rule.remainder, 4);
        assert_eq!(rule.velocity.x, 0);
        assert_eq!(rule.velocity.y, -1);

        assert!(rule.check_coords(&Coords::new(3, 4, 1, 0)));
        assert!(!rule.check_coords(&Coords::new(4, 4, 1, 0)));
    }

    #[test]
    fn test_rule_1() {
        let rule = Rule::build(
            "RULE 1: 8x+2y+3z+5a DIVIDE 9 HAS REMAINDER 4 | DEBRIS VELOCITY (0, -1, 0, 1)",
        );
        assert_eq!(rule.y, 2);

        const SPACE_DIMS: Coords = Coords::new(3, 3, 5, 3);
        assert_eq!(rule.get_all_debris(&SPACE_DIMS).len(), 14);
    }

    #[test]
    fn test_part1_1() {
        let rules = build(&INPUT_TEST_1);
        const SPACE_DIMS: Coords = Coords::new(3, 3, 5, 3);
        assert_eq!(total_debris(&rules, &SPACE_DIMS), 146);
    }

    #[test]
    fn test_part1_2() {
        let rules = build(&INPUT_TEST_2);
        const SPACE_DIMS: Coords = Coords::new(10, 15, 60, 3);
        assert_eq!(total_debris(&rules, &SPACE_DIMS), 32545);
    }
}
