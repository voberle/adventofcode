use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
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

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
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
    fn new(axe: Axe, x: i32, y: i32, z: i32) -> Self {
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

fn big_cube(cubes: &[Coord]) -> (Coord, Coord) {
    let mut min = Coord::new(i32::MAX, i32::MAX, i32::MAX);
    let mut max = Coord::new(i32::MIN, i32::MIN, i32::MIN);
    for c in cubes {
        min.x = min.x.min(c.x);
        min.y = min.y.min(c.y);
        min.z = min.z.min(c.z);
        max.x = max.x.max(c.x);
        max.y = max.y.max(c.y);
        max.z = max.z.max(c.z);
    }
    // Make the big cube big enough so it fully wraps the small cubes.
    min.x -= 1;
    min.y -= 1;
    min.z -= 1;
    max.x += 1;
    max.y += 1;
    max.z += 1;
    (min, max)
}

fn create_exterior_list(
    lava_cubes: &FxHashSet<Coord>,
    big_cube_min: &Coord,
    big_cube_max: &Coord,
) -> FxHashSet<Coord> {
    let mut queue: Vec<Coord> = Vec::new();
    // Add corner.
    queue.push(*big_cube_min);

    // It's also our "visited" set.
    let mut exterior: FxHashSet<Coord> = FxHashSet::default();

    while let Some(cube) = queue.pop() {
        exterior.insert(cube);

        queue.extend(
            [
                Coord::new(cube.x - 1, cube.y, cube.z),
                Coord::new(cube.x, cube.y - 1, cube.z),
                Coord::new(cube.x, cube.y, cube.z - 1),
                Coord::new(cube.x + 1, cube.y, cube.z),
                Coord::new(cube.x, cube.y + 1, cube.z),
                Coord::new(cube.x, cube.y, cube.z + 1),
            ]
            .iter()
            .filter(|next| {
                if next.x < big_cube_min.x
                    || next.y < big_cube_min.y
                    || next.z < big_cube_min.z
                    || next.x > big_cube_max.x
                    || next.y > big_cube_max.y
                    || next.z > big_cube_max.z
                {
                    // Outside the big cube.
                    return false;
                }
                if exterior.contains(next) {
                    // Already visited.
                    return false;
                }
                if lava_cubes.contains(next) {
                    // Inside, part of the lava.
                    return false;
                }
                true
            }),
        );
    }
    exterior
}

fn create_total_list(big_cube_min: &Coord, big_cube_max: &Coord) -> FxHashSet<Coord> {
    let mut total: FxHashSet<Coord> = FxHashSet::default();
    for x in big_cube_min.x..=big_cube_max.x {
        for y in big_cube_min.y..=big_cube_max.y {
            for z in big_cube_min.z..=big_cube_max.z {
                total.insert(Coord::new(x, y, z));
            }
        }
    }
    total
}

fn exposed_sides_minus_droplets(cubes: &[Coord], lava_exposed_sides: usize) -> usize {
    // Let's say we have a big cube that contains all our small cubes. All together it's "total" cubes.
    // - All the cubes around our small cubes are "exterior" cubes.
    // - Our small cubes are "lava" cubes.
    // - Pockets inside the input cubes are "droplet" cubes.
    // So we have:
    //   "total" = "exterior" + "lava" + "droplet"
    // Lava is our input. Total is trivial to calculate. If we can get exterior, we can get the list of droplet cubes.
    // Then we can get the exposed sides of droplet cubes and remove that from the part 1 result.

    let (big_cube_min, big_cube_max) = big_cube(cubes);

    let mut lava_cubes: FxHashSet<Coord> = FxHashSet::default();
    lava_cubes.extend(cubes);

    let total_cubes = create_total_list(&big_cube_min, &big_cube_max);

    let exterior_cubes = create_exterior_list(&lava_cubes, &big_cube_min, &big_cube_max);

    let mut droplets_cubes: FxHashSet<Coord> =
        total_cubes.difference(&exterior_cubes).copied().collect();
    droplets_cubes = droplets_cubes.difference(&lava_cubes).copied().collect();

    // Get exposed sides of droplets.
    let droplet_exposed = exposed_sides(&droplets_cubes.iter().copied().collect::<Vec<Coord>>());

    lava_exposed_sides - droplet_exposed
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cubes = build(&input);

    let lava_exposed_sides = exposed_sides(&cubes);
    println!("Part 1: {lava_exposed_sides}");
    println!(
        "Part 2: {}",
        exposed_sides_minus_droplets(&cubes, lava_exposed_sides)
    );
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
        let cubes = build(INPUT_TEST);
        let lava_exposed_sides = exposed_sides(&cubes);
        assert_eq!(exposed_sides_minus_droplets(&cubes, lava_exposed_sides), 58);
    }
}
