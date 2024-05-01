use std::io::{self, Read};

use fxhash::FxHashMap;
use itertools::Itertools;

type Point = (i32, i32);

#[derive(Debug, Clone)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn build(s: &str) -> Self {
        let (x1, y1, x2, y2) = s
            .split(" -> ")
            .flat_map(|p| p.split(',').map(|v| v.parse().unwrap()))
            .collect_tuple()
            .unwrap();
        Self { x1, y1, x2, y2 }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn get_points(&self) -> Vec<Point> {
        let x_inc = i32::signum(self.x2 - self.x1);
        let y_inc = i32::signum(self.y2 - self.y1);
        let points_count = i32::abs(self.x1 - self.x2).max(i32::abs(self.y1 - self.y2));

        (0..=points_count)
            .map(|n| (self.x1 + n * x_inc, self.y1 + n * y_inc))
            .collect()
    }
}

fn build(input: &str) -> Vec<Line> {
    input.lines().map(Line::build).collect()
}

fn overlapping_points(lines: &[Line], filter_fn: fn(&Line) -> bool) -> usize {
    // Brute-forcing it.
    let mut points: FxHashMap<Point, usize> = FxHashMap::default();
    lines
        .iter()
        .filter(|line| filter_fn(line))
        .flat_map(Line::get_points)
        .for_each(|p| {
            points.entry(p).and_modify(|e| *e += 1).or_insert(1);
        });
    points.values().filter(|c| **c >= 2).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!(
        "Part 1: {}",
        overlapping_points(&lines, Line::is_horizontal_or_vertical)
    );
    println!("Part 2: {}", overlapping_points(&lines, |_| true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_get_points() {
        assert_eq!(
            Line::build("1,1 -> 1,3").get_points(),
            &[(1, 1), (1, 2), (1, 3)]
        );
        assert_eq!(
            Line::build("9,7 -> 7,7").get_points(),
            &[(9, 7), (8, 7), (7, 7)]
        );
        assert_eq!(
            Line::build("1,1 -> 3,3").get_points(),
            &[(1, 1), (2, 2), (3, 3)]
        );
        assert_eq!(
            Line::build("9,7 -> 7,9").get_points(),
            &[(9, 7), (8, 8), (7, 9)]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            overlapping_points(&build(INPUT_TEST), Line::is_horizontal_or_vertical),
            5
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(overlapping_points(&build(INPUT_TEST), |_| true), 12);
    }
}
