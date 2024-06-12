use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u32,
    y: u32,
    z: u32,
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let (x, y, z) = value
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x, y, z }
    }
}

fn build(input: &str) -> Vec<Coord> {
    input.lines().map(Into::into).collect()
}

// We identify a side on its axe (x, y or z) and the coordinate that is just after on this axe.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Axe {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Side {
    axe: Axe,
    center: Coord,
}

impl Side {
    fn new(axe: Axe, x: u32, y: u32, z: u32) -> Self {
        Self {
            axe,
            center: Coord { x, y, z },
        }
    }
}

fn exposed_sides(cubes: &[Coord]) -> usize {
    let mut all_sides: FxHashSet<Side> = FxHashSet::default();
    // Number of sides that are covered, i.e. already oresent in the hashset.
    let mut covered_sides = 0;

    for cube in cubes {
        for side in [
            Side::new(Axe::X, cube.x, cube.y, cube.z),
            Side::new(Axe::Y, cube.x, cube.y, cube.z),
            Side::new(Axe::Z, cube.x, cube.y, cube.z),
            Side::new(Axe::X, cube.x + 1, cube.y, cube.z),
            Side::new(Axe::Y, cube.x, cube.y + 1, cube.z),
            Side::new(Axe::Z, cube.x, cube.y, cube.z + 1),
        ] {
            if !all_sides.insert(side) {
                covered_sides += 1;
            }
        }
    }

    let total_sides = cubes.len() * 6;
    total_sides - covered_sides * 2
}

fn part2(cubes: &[Coord]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cubes = build(&input);

    println!("Part 1: {}", exposed_sides(&cubes));
    println!("Part 2: {}", part2(&cubes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(exposed_sides(&build(INPUT_TEST)), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
