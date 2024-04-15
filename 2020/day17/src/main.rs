use std::io::{self, Read};

use fxhash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
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
                    ))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn build_neighbors_coords(pos: &Coord) -> Vec<Coord> {
    vec![
        Coord::new(pos.x, pos.y, pos.z - 1),
        Coord::new(pos.x, pos.y, pos.z + 1),
        Coord::new(pos.x, pos.y - 1, pos.z - 1),
        Coord::new(pos.x, pos.y - 1, pos.z),
        Coord::new(pos.x, pos.y - 1, pos.z + 1),
        Coord::new(pos.x, pos.y + 1, pos.z - 1),
        Coord::new(pos.x, pos.y + 1, pos.z),
        Coord::new(pos.x, pos.y + 1, pos.z + 1),
        Coord::new(pos.x - 1, pos.y, pos.z),
        Coord::new(pos.x - 1, pos.y, pos.z - 1),
        Coord::new(pos.x - 1, pos.y, pos.z + 1),
        Coord::new(pos.x - 1, pos.y - 1, pos.z - 1),
        Coord::new(pos.x - 1, pos.y - 1, pos.z),
        Coord::new(pos.x - 1, pos.y - 1, pos.z + 1),
        Coord::new(pos.x - 1, pos.y + 1, pos.z - 1),
        Coord::new(pos.x - 1, pos.y + 1, pos.z),
        Coord::new(pos.x - 1, pos.y + 1, pos.z + 1),
        Coord::new(pos.x + 1, pos.y, pos.z),
        Coord::new(pos.x + 1, pos.y, pos.z - 1),
        Coord::new(pos.x + 1, pos.y, pos.z + 1),
        Coord::new(pos.x + 1, pos.y - 1, pos.z - 1),
        Coord::new(pos.x + 1, pos.y - 1, pos.z),
        Coord::new(pos.x + 1, pos.y - 1, pos.z + 1),
        Coord::new(pos.x + 1, pos.y + 1, pos.z - 1),
        Coord::new(pos.x + 1, pos.y + 1, pos.z),
        Coord::new(pos.x + 1, pos.y + 1, pos.z + 1),
    ]
}

fn count_neighbors(dimension: &FxHashSet<Coord>, pos: &Coord) -> usize {
    let neighbors_coords = build_neighbors_coords(pos);
    neighbors_coords
        .iter()
        .filter(|c| dimension.contains(c))
        .count()
}

fn active_cubes_count(dimension: &FxHashSet<Coord>, cycles: usize) -> usize {
    let mut dimension = dimension.clone();

    for _ in 0..cycles {
        let mut next_dimension: FxHashSet<Coord> = FxHashSet::default();
        // Get all coordinates to examine
        let mut coords_to_check = dimension.clone();
        coords_to_check.extend(dimension.iter().flat_map(build_neighbors_coords));

        for coords in coords_to_check {
            let neighbors_active = count_neighbors(&dimension, &coords);
            if dimension.contains(&coords) {
                // Cube active
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

fn part2(dimension: &FxHashSet<Coord>) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let dimension = build(&input);

    println!("Part 1: {}", active_cubes_count(&dimension, 6));
    println!("Part 2: {}", part2(&dimension));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(active_cubes_count(&build(INPUT_TEST), 6), 112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
