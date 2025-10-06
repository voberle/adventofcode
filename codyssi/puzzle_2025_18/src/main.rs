use std::{
    fmt::Display,
    io::{self, Read},
};

use once_cell::sync::Lazy;
use regex::Regex;

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
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.a)
    }
}

struct Rule {
    number: usize,
    x: i64,
    y: i64,
    z: i64,
    a: i64,
    divide: i64,
    remainder: i64,
    vel_x: i64,
    vel_y: i64,
    vel_z: i64,
    vel_a: i64,
}

impl Rule {
    fn build(line: &str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"RULE (\d+): (\d+)x\+(\d+)y\+(\d+)z\+(\d+)a DIVIDE (\d+) HAS REMAINDER (\d+) \| DEBRIS VELOCITY \((-?\d+), (-?\d+), (-?\d+), (-?\d+)\)")
                .unwrap()
        });

        let p = RE.captures(line).unwrap();
        Self {
            number: p[1].parse().unwrap(),
            x: p[2].parse().unwrap(),
            y: p[3].parse().unwrap(),
            z: p[4].parse().unwrap(),
            a: p[5].parse().unwrap(),
            divide: p[6].parse().unwrap(),
            remainder: p[7].parse().unwrap(),
            vel_x: p[8].parse().unwrap(),
            vel_y: p[9].parse().unwrap(),
            vel_z: p[10].parse().unwrap(),
            vel_a: p[11].parse().unwrap(),
        }
    }

    fn check_coords(&self, coords: &Coords) -> bool {
        let t = self.x * coords.x + self.y * coords.y + self.z * coords.z + self.a * coords.a;
        // For negative numbers we need to use rem_euclid, as % gives negative numbers, problem expects positive ones.
        self.remainder == t.rem_euclid(self.divide)
    }

    fn total_debris(&self, space_dims: &Coords) -> i64 {
        // Space dimensions for a always go from -1 to 1
        assert_eq!(space_dims.a, 3);

        (0..space_dims.x)
            .map(|x| {
                (0..space_dims.y)
                    .map(|y| {
                        (0..space_dims.z)
                            .map(|z| {
                                (-1..=1)
                                    .map(|a| {
                                        let c = Coords::new(x, y, z, a);
                                        // println!("{}", c);
                                        i64::from(self.check_coords(&c))
                                    })
                                    .sum::<i64>()
                            })
                            .sum::<i64>()
                    })
                    .sum::<i64>()
            })
            .sum::<i64>()
    }
}

fn build(input: &str) -> Vec<Rule> {
    input.lines().map(Rule::build).collect()
}

fn total_debris(rules: &[Rule], space_dims: &Coords) -> i64 {
    // Space dimensions for a always go from -1 to 1
    assert_eq!(space_dims.a, 3);

    rules.iter().map(|rule| rule.total_debris(space_dims)).sum()
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
    fn test_rule_build() {
        let rule = Rule::build(
            "RULE 1: 8x+10y+3z+5a DIVIDE 9 HAS REMAINDER 4 | DEBRIS VELOCITY (0, -1, 0, 1)",
        );
        assert_eq!(rule.number, 1);
        assert_eq!(rule.y, 10);
        assert_eq!(rule.remainder, 4);
        assert_eq!(rule.vel_x, 0);
        assert_eq!(rule.vel_y, -1);

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
        assert_eq!(rule.total_debris(&SPACE_DIMS), 14);
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
