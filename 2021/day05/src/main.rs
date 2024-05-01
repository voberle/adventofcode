use std::io::{self, Read};

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn build(s: &str) -> Self {
        let (p1, p2) = s.split(" -> ").collect_tuple().unwrap();
        let (x1, y1) = p1
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (x2, y2) = p2
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x1, y1, x2, y2 }
    }

    fn is_horizontal(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_vertical(&self) -> bool {
        self.y1 == self.y2
    }

    fn get_points(&self) -> Vec<(u32, u32)> {
        if self.is_horizontal() {
            let (y1, y2) = if self.y1 < self.y2 {
                (self.y1, self.y2)
            } else {
                (self.y2, self.y1)
            };
            (y1..=y2).map(|y| (self.x1, y)).collect()
        } else if self.is_vertical() {
            let (x1, x2) = if self.x1 < self.x2 {
                (self.x1, self.x2)
            } else {
                (self.x2, self.x1)
            };
            (x1..=x2).map(|x| (x, self.y1)).collect()
        } else {
            panic!("Only horizontal or vertical lines supported")
        }
    }
}

fn build(input: &str) -> Vec<Line> {
    input.lines().map(Line::build).collect()
}

fn points_with_two_more_overlapping(lines: &[Line]) -> usize {
    // Brute-forcing it.
    let mut points: FxHashMap<(u32, u32), usize> = FxHashMap::default();
    for line in lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
    {
        let line_points = line.get_points();
        for p in line_points {
            points.entry(p).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    points.values().filter(|c| **c >= 2).count()
}

fn part2(lines: &[Line]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Part 1: {}", points_with_two_more_overlapping(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(points_with_two_more_overlapping(&build(INPUT_TEST)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
