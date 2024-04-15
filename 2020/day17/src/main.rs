use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::{repeat_n, Itertools};

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

fn build_neighbors_coords<const DIMS: usize>(pos: &Coord) -> Vec<Coord> {
    if DIMS == 3 {
        repeat_n([-1, 0, 1].iter(), 3)
            .multi_cartesian_product()
            .filter(|p| *p[0] != 0 || *p[1] != 0 || *p[2] != 0)
            .map(|p| Coord::new(pos.x + p[0], pos.y + p[1], pos.z + p[2], 0))
            .collect()
    } else if DIMS == 4 {
        repeat_n([-1, 0, 1].iter(), 4)
            .multi_cartesian_product()
            .filter(|p| *p[0] != 0 || *p[1] != 0 || *p[2] != 0 || *p[3] != 0)
            .map(|p| Coord::new(pos.x + p[0], pos.y + p[1], pos.z + p[2], pos.w + p[3]))
            .collect()
    } else {
        panic!("Invalid dim count")
    }
}

fn count_neighbors<const DIMS: usize>(dimension: &FxHashSet<Coord>, pos: &Coord) -> usize {
    let neighbors_coords = build_neighbors_coords::<DIMS>(pos);
    neighbors_coords
        .iter()
        .filter(|c| dimension.contains(c))
        .count()
}

fn active_cubes_count<const DIMS: usize>(dimension: &FxHashSet<Coord>) -> usize {
    const CYCLES_COUNT: usize = 6;

    let mut dimension = dimension.clone();

    for _ in 0..CYCLES_COUNT {
        // Get all coordinates to examine.
        let mut coords_to_check = dimension.clone();
        coords_to_check.extend(dimension.iter().flat_map(build_neighbors_coords::<DIMS>));

        // Generate new dimension.
        let mut next_dimension: FxHashSet<Coord> = FxHashSet::default();
        for coords in coords_to_check {
            let neighbors_active = count_neighbors::<DIMS>(&dimension, &coords);

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

    println!("Part 1: {}", active_cubes_count::<3>(&dimension));
    println!("Part 2: {}", active_cubes_count::<4>(&dimension));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(active_cubes_count::<3>(&build(INPUT_TEST)), 112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(active_cubes_count::<4>(&build(INPUT_TEST)), 848);
    }
}
