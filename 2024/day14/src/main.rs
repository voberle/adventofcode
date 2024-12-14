use std::{
    cmp::Ordering,
    io::{self, Read},
};

use itertools::Itertools;

struct Robot {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Robot {
    fn build(line: &str) -> Self {
        fn parse_coords(s: &str) -> (i32, i32) {
            s[2..]
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap()
        }

        let (p, v) = line.split_whitespace().collect_tuple().unwrap();
        let (pos_x, pos_y) = parse_coords(p);
        let (vel_x, vel_y) = parse_coords(v);
        Self {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
        }
    }

    fn position_after(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        (
            (self.pos_x + self.vel_x * seconds).rem_euclid(width),
            (self.pos_y + self.vel_y * seconds).rem_euclid(height),
        )
    }
}

fn build(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::build).collect()
}

const WIDTH_REAL: i32 = 101;
const HEIGHT_REAL: i32 = 103;

const TIME: i32 = 100;

fn safety_factor(robots: &[Robot], width: i32, height: i32) -> i32 {
    let mut quadrant_top_left = 0;
    let mut quadrant_top_right = 0;
    let mut quadrant_bottom_left = 0;
    let mut quadrant_bottom_right = 0;
    for (x, y) in robots
        .iter()
        .map(|robot| robot.position_after(TIME, width, height))
    {
        match x.cmp(&(width / 2)) {
            Ordering::Less => match y.cmp(&(height / 2)) {
                Ordering::Less => quadrant_top_left += 1,
                Ordering::Greater => quadrant_bottom_left += 1,
                Ordering::Equal => {}
            },
            Ordering::Greater => match y.cmp(&(height / 2)) {
                Ordering::Less => quadrant_top_right += 1,
                Ordering::Greater => quadrant_bottom_right += 1,
                Ordering::Equal => {}
            },
            Ordering::Equal => {}
        }
    }
    quadrant_top_left * quadrant_top_right * quadrant_bottom_left * quadrant_bottom_right
}

fn part2(robots: &[Robot]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let robots = build(&input);

    println!(
        "Part 1: {}",
        safety_factor(&robots, WIDTH_REAL, HEIGHT_REAL)
    );
    println!("Part 2: {}", part2(&robots));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");
    const WIDTH_TEST: i32 = 11;
    const HEIGHT_TEST: i32 = 7;

    #[test]
    fn test_part1() {
        assert_eq!(
            safety_factor(&build(INPUT_TEST), WIDTH_TEST, HEIGHT_TEST),
            12
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
