use std::io::{self, Read};

use itertools::Itertools;

struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn build(s: &str) -> Self {
        let (x, y) = s
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x, y }
    }

    fn rect_area(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn build(input: &str) -> Vec<Position> {
    input.lines().map(Position::build).collect()
}

fn largest_rect_area(positions: &[Position]) -> u64 {
    positions
        .iter()
        .combinations(2)
        .map(|p| p[0].rect_area(p[1]))
        .max()
        .unwrap()
}

fn part2(positions: &[Position]) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let positions = build(&input);

    println!("Part 1: {}", largest_rect_area(&positions));
    println!("Part 2: {}", part2(&positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_rect_area() {
        let p1 = Position::build("2,5");
        let p2 = Position::build("11,1");
        assert_eq!(p1.rect_area(&p2), 50);
    }

    #[test]
    fn test_part1() {
        assert_eq!(largest_rect_area(&build(INPUT_TEST)), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
