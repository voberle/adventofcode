use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::{repeat_n, Itertools};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }
}

fn build(input: &str) -> FxHashSet<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Coord::new(
                        i32::try_from(x).unwrap(),
                        i32::try_from(y).unwrap(),
                        0,
                        0,
                    ))
                } else {
                    None
                }
            })
        })
        .collect()
}

type Diff = (i32, i32, i32, i32);

lazy_static! {
    static ref DIFFS_3D: Vec<Diff> = {
        repeat_n([-1, 0, 1].iter(), 3)
            .multi_cartesian_product()
            .filter(|p| *p[0] != 0 || *p[1] != 0 || *p[2] != 0)
            .map(|p| (*p[0], *p[1], *p[2], 0))
            .collect()
    };
    static ref DIFFS_4D: Vec<Diff> = {
        repeat_n([-1, 0, 1].iter(), 4)
            .multi_cartesian_product()
            .filter(|p| *p[0] != 0 || *p[1] != 0 || *p[2] != 0 || *p[3] != 0)
            .map(|p| (*p[0], *p[1], *p[2], *p[3]))
            .collect()
    };
}

fn build_neighbors_coords(pos: &Coord, diffs: &[Diff]) -> Vec<Coord> {
    diffs
        .iter()
        .map(|d| Coord::new(pos.x + d.0, pos.y + d.1, pos.z + d.2, pos.w + d.3))
        .collect()
}

fn count_neighbors(dimension: &FxHashSet<Coord>, pos: &Coord, diffs: &[Diff]) -> usize {
    let neighbors_coords = build_neighbors_coords(pos, diffs);
    neighbors_coords
        .iter()
        .filter(|c| dimension.contains(c))
        .count()
}

fn active_cubes_count(dimension: &FxHashSet<Coord>, diffs: &[Diff]) -> usize {
    const CYCLES_COUNT: usize = 6;

    let mut dimension = dimension.clone();

    for _ in 0..CYCLES_COUNT {
        // Get all coordinates to examine.
        let mut coords_to_check = dimension.clone();
        coords_to_check.extend(
            dimension
                .iter()
                .flat_map(|c| build_neighbors_coords(c, diffs)),
        );

        // Generate new dimension.
        let mut next_dimension: FxHashSet<Coord> = FxHashSet::default();
        for coords in coords_to_check {
            let neighbors_active = count_neighbors(&dimension, &coords, diffs);

            if dimension.contains(&coords) {
                if neighbors_active == 2 || neighbors_active == 3 {
                    next_dimension.insert(coords);
                }
            } else if neighbors_active == 3 {
                next_dimension.insert(coords);
            }
        }

        std::mem::swap(&mut dimension, &mut next_dimension);
    }

    dimension.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let dimension = build(&input);

    println!("Part 1: {}", active_cubes_count(&dimension, &DIFFS_3D));
    println!("Part 2: {}", active_cubes_count(&dimension, &DIFFS_4D));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(active_cubes_count(&build(INPUT_TEST), &DIFFS_3D), 112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(active_cubes_count(&build(INPUT_TEST), &DIFFS_4D), 848);
    }
}
