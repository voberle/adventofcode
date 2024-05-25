use std::{
    collections::VecDeque,
    fmt,
    io::{self, Read},
};

// Result of the comparaison of two cuboid.
#[derive(Debug, PartialEq)]
enum CuboidPosition {
    IsInside,  // The other cube is fully inside this one.
    Wraps,     // The other cube contains this one.
    NoOverlap, // Both cubes have no overlap.
    Overlap,   // The cubes overlap partially.
}

// The value "1" is always smaller or equal than the value "2".
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cuboid {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl From<((i32, i32, i32), (i32, i32, i32))> for Cuboid {
    fn from(c: ((i32, i32, i32), (i32, i32, i32))) -> Self {
        Self {
            x1: c.0 .0,
            y1: c.0 .1,
            z1: c.0 .2,
            x2: c.1 .0,
            y2: c.1 .1,
            z2: c.1 .2,
        }
    }
}

impl From<((i32, i32), (i32, i32), (i32, i32))> for Cuboid {
    fn from(c: ((i32, i32), (i32, i32), (i32, i32))) -> Self {
        Self {
            x1: c.0 .0,
            x2: c.0 .1,
            y1: c.1 .0,
            y2: c.1 .1,
            z1: c.2 .0,
            z2: c.2 .1,
        }
    }
}

impl From<&str> for Cuboid {
    fn from(line: &str) -> Self {
        let p: Vec<_> = line
            .split(',')
            .flat_map(|axe| axe[2..].split("..").map(|c| c.parse().unwrap()))
            .collect();
        Self {
            x1: p[0],
            x2: p[1],
            y1: p[2],
            y2: p[3],
            z1: p[4],
            z2: p[5],
        }
    }
}

impl Cuboid {
    fn is_initialization(&self) -> bool {
        self.x1.abs() <= 50
            && self.x2.abs() <= 50
            && self.y1.abs() <= 50
            && self.y2.abs() <= 50
            && self.z1.abs() <= 50
            && self.z2.abs() <= 50
    }

    // Returns all "dots" of the cuboid.
    // Used in the brute-force version.
    fn all_cubes(&self) -> Vec<(i32, i32, i32)> {
        (self.x1..=self.x2)
            .flat_map(|x| {
                (self.y1..=self.y2).flat_map(move |y| (self.z1..=self.z2).map(move |z| (x, y, z)))
            })
            .collect()
    }

    #[allow(clippy::cast_sign_loss)]
    fn volume(&self) -> u64 {
        (self.x2 - self.x1 + 1) as u64
            * (self.y2 - self.y1 + 1) as u64
            * (self.z2 - self.z1 + 1) as u64
    }

    fn contains(&self, other: &Cuboid) -> bool {
        // Should we store the ranges in the struct as an optimization?
        let xr = self.x1..=self.x2;
        let yr = self.y1..=self.y2;
        let zr = self.z1..=self.z2;

        xr.contains(&other.x1)
            && xr.contains(&other.x2)
            && yr.contains(&other.y1)
            && yr.contains(&other.y2)
            && zr.contains(&other.z1)
            && zr.contains(&other.z2)
    }

    // Compares the position of two cuboid.
    fn cmp(&self, other: &Cuboid) -> CuboidPosition {
        if self.contains(other) {
            CuboidPosition::Wraps
        } else if other.contains(self) {
            CuboidPosition::IsInside
        } else if self.x2 < other.x1
            || other.x2 < self.x1
            || self.y2 < other.y1
            || other.y2 < self.y1
            || self.z2 < other.z1
            || other.z2 < self.z1
        {
            CuboidPosition::NoOverlap
        } else {
            CuboidPosition::Overlap
        }
    }

    // Split two overlaping cuboids into smaller ones.
    // Precisely, assuming self and other overlap, this function return cuboids
    // that compose self minus the part that overlaps with other.
    // Supports other being fully inside self, or partially overlapping it.
    fn split(&self, other: &Cuboid) -> Vec<Cuboid> {
        // Identify the overlapping part. Those coordinates are also the splitting planes.
        let x_min = self.x1.max(other.x1);
        let y_min = self.y1.max(other.y1);
        let z_min = self.z1.max(other.z1);
        let x_max = self.x2.min(other.x2);
        let y_max = self.y2.min(other.y2);
        let z_max = self.z2.min(other.z2);

        // Check there is overlap.
        assert!(x_min <= x_max);
        assert!(y_min <= y_max);
        assert!(z_min <= z_max);

        // Create the smaller cuboids by excluding the volume defined by the overlap.
        // Note that we need to take into account that the borders are part of the cuboid,
        // the -1 and +1 deal with that.
        let mut smaller = Vec::new();
        // Left
        if self.x1 < x_min {
            smaller.push(((self.x1, self.y1, self.z1), (x_min - 1, self.y2, self.z2)).into());
        }
        // Right
        if x_max < self.x2 {
            smaller.push(((x_max + 1, self.y1, self.z1), (self.x2, self.y2, self.z2)).into());
        }
        // Below
        if self.y1 < y_min {
            smaller.push(((x_min, self.y1, self.z1), (x_max, y_min - 1, self.z2)).into());
        }
        // Above
        if y_max < self.y2 {
            smaller.push(((x_min, y_max + 1, self.z1), (x_max, self.y2, self.z2)).into());
        }
        // In front
        if self.z1 < z_min {
            smaller.push(((x_min, y_min, self.z1), (x_max, y_max, z_min - 1)).into());
        }
        // Behind
        if z_max < self.z2 {
            smaller.push(((x_min, y_min, z_max + 1), (x_max, y_max, self.z2)).into());
        }

        smaller
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{}",
            self.x1, self.x2, self.y1, self.y2, self.z1, self.z2
        )
    }
}

fn build(input: &str) -> Vec<(bool, Cuboid)> {
    input
        .lines()
        .map(|line| {
            if let Some(coords) = line.strip_prefix("on ") {
                (true, coords.into())
            } else if let Some(coords) = line.strip_prefix("off ") {
                (false, coords.into())
            } else {
                panic!("Invalid input")
            }
        })
        .collect()
}

fn get_init_steps(reboot_steps: &[(bool, Cuboid)]) -> Vec<(bool, Cuboid)> {
    reboot_steps
        .iter()
        .filter(|(_, s)| s.is_initialization())
        .copied()
        .collect()
}

// Brute-force version, which finds the volume by counting each "cube" of each cuboid.
// Gets too slow when dimensions are bigger (or even runs out of memory).
#[allow(dead_code, clippy::cast_sign_loss)]
fn cubes_on_brute_force(reboot_steps: &[(bool, Cuboid)]) -> u64 {
    let mut reactor = vec![vec![vec![false; 101]; 101]; 101];
    for (set_on, cuboid) in reboot_steps {
        for c in cuboid.all_cubes() {
            reactor[(c.0 + 50) as usize][(c.1 + 50) as usize][(c.2 + 50) as usize] = *set_on;
        }
    }
    reactor
        .iter()
        .flatten()
        .flatten()
        .filter(|v| **v)
        .count()
        .try_into()
        .unwrap()
}

macro_rules! remove_indexes {
    ($vector:expr, $indexes:expr) => {
        for i in $indexes.iter().rev() {
            $vector.remove(*i);
        }
    };
}

// We maintain a list of non-overlapping cuboids.
// We go through each cuboid C in the steps list, and we check each against each cuboid E of the list.
//
// If cube C is marked ON:
// - If C is fully inside E, we drop C from the steps list.
// - If E is fully inside C, we remove E from the list, and keep checking C against the remaining steps.
// - If C and E overlap, we split C into smaller cuboids that do not overlap,
//   and continue testing these smaller ones against the remaining elements of the list.
// - If there is no overlap, do nothing (keep C).
// At the end of testing C or its pieces against all Es, add those pieces to list and continue with the next
// step.
//
// If cube C is marked OFF:
// - If C is fully inside E, split E into smaller cuboids and all them to the list, minus the part covered by C.
// - If E is fully inside C, remove E from the list.
// - If C and E overlap, split E into smaller cuboids that do not overlap and add them to the list (like C inside E case).
// - If there is no overlap, do nothing.
// At the end of testing C, nothing extra is done.
//
// Once all done, add the volumes of all cuboids in the list.
fn cubes_on(reboot_steps: &[(bool, Cuboid)]) -> u64 {
    let mut non_overlap_cuboids: Vec<Cuboid> = Vec::new();

    // Use a VecDeque for the steps, and consume them by popping,
    // since this allows to modify it much better than if we would just loop on it.
    let mut steps: VecDeque<(bool, Cuboid)> = reboot_steps.iter().copied().collect();

    // Add first one, make sure it's set on.
    let first = steps.pop_front().unwrap();
    assert!(first.0);
    non_overlap_cuboids.push(first.1);

    'outer: while let Some((set_on, cuboid_to_check)) = steps.pop_front() {
        if set_on {
            let mut elements_to_remove: Vec<usize> = Vec::new();
            remove_indexes!(non_overlap_cuboids, elements_to_remove);

            for (elt_index, elt) in non_overlap_cuboids.iter().enumerate() {
                match cuboid_to_check.cmp(elt) {
                    CuboidPosition::IsInside => {
                        // This cuboid can be dropped.
                        continue 'outer;
                    }
                    CuboidPosition::Wraps => {
                        // That element is dropped, to be replaced by the cuboid we are checking.
                        elements_to_remove.push(elt_index);
                    }
                    CuboidPosition::Overlap => {
                        let smaller_cuboids = cuboid_to_check.split(elt);
                        // Replace the cuboid with the smaller ones.
                        for smaller in smaller_cuboids {
                            steps.push_front((true, smaller));
                        }
                        // Restart, check the smaller cuboids instead.
                        continue 'outer;
                    }
                    CuboidPosition::NoOverlap => {}
                }
            }

            remove_indexes!(non_overlap_cuboids, elements_to_remove);

            // If we got there, add the cuboid we are checking to the list.
            non_overlap_cuboids.push(cuboid_to_check);
        } else {
            let mut elements_to_add: Vec<Cuboid> = Vec::new();
            let mut elements_to_remove: Vec<usize> = Vec::new();

            remove_indexes!(non_overlap_cuboids, elements_to_remove);
            non_overlap_cuboids.extend(elements_to_add.iter());

            for (elt_index, elt) in non_overlap_cuboids.iter().enumerate() {
                match cuboid_to_check.cmp(elt) {
                    CuboidPosition::IsInside => {
                        let smaller_cuboids = elt.split(&cuboid_to_check);
                        // Replace the cuboid with the smaller ones.
                        elements_to_remove.push(elt_index);
                        for smaller in smaller_cuboids {
                            elements_to_add.push(smaller);
                        }
                        continue 'outer;
                    }
                    CuboidPosition::Wraps => {
                        elements_to_remove.push(elt_index);
                    }
                    CuboidPosition::Overlap => {
                        let smaller_cuboids = elt.split(&cuboid_to_check);
                        // Replace the cuboid with the smaller ones.
                        elements_to_remove.push(elt_index);
                        for smaller in smaller_cuboids {
                            elements_to_add.push(smaller);
                        }
                    }
                    CuboidPosition::NoOverlap => {}
                }
            }

            remove_indexes!(non_overlap_cuboids, elements_to_remove);
            non_overlap_cuboids.extend(elements_to_add.iter());
        }
    }

    non_overlap_cuboids.iter().map(Cuboid::volume).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reboot_steps = build(&input);

    // println!("Part 1: {}", cubes_on_brute_force(&get_init_steps(&reboot_steps)));

    println!("Part 1: {}", cubes_on(&get_init_steps(&reboot_steps)));
    println!("Part 2: {}", cubes_on(&reboot_steps));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_volume() {
        let a = ((2, 2, 2), (10, 5, 3)).into();
        let vol_bf = cubes_on_brute_force(&[(true, a)]);
        assert_eq!(a.volume(), vol_bf);
    }

    #[test]
    fn test_cuboid_contains() {
        // Checking in 2D for simplicity.
        let a: Cuboid = ((5, 20), (2, 30), (1, 1)).into();
        let b: Cuboid = ((7, 19), (10, 12), (1, 1)).into();
        assert!(a.contains(&b));
        assert!(!b.contains(&a));
    }

    #[test]
    fn test_cuboid_compare() {
        let a: Cuboid = ((5, 20), (2, 30), (1, 1)).into();
        let b: Cuboid = ((7, 19), (10, 12), (1, 1)).into();
        assert_eq!(a.cmp(&b), CuboidPosition::Wraps);
        assert_eq!(b.cmp(&a), CuboidPosition::IsInside);

        let c = ((5, 20), (42, 100), (1, 1)).into();
        assert_eq!(a.cmp(&c), CuboidPosition::NoOverlap);
        assert_eq!(c.cmp(&a), CuboidPosition::NoOverlap);

        let d = ((7, 19), (10, 42), (1, 1)).into();
        assert_eq!(a.cmp(&d), CuboidPosition::Overlap);
        assert_eq!(d.cmp(&a), CuboidPosition::Overlap);
    }

    #[test]
    fn test_split() {
        // Test cases generated with the help of ChatGPT.

        // b1 fully inside a1
        let a1: Cuboid = ((0, 0, 0), (10, 10, 10)).into();
        let b1: Cuboid = ((5, 5, 5), (8, 8, 8)).into();
        assert_eq!(
            a1.split(&b1),
            &[
                ((0, 0, 0), (4, 10, 10)).into(),
                ((9, 0, 0), (10, 10, 10)).into(),
                ((5, 0, 0), (8, 4, 10)).into(),
                ((5, 9, 0), (8, 10, 10)).into(),
                ((5, 5, 0), (8, 8, 4)).into(),
                ((5, 5, 9), (8, 8, 10)).into(),
            ]
        );

        // Partial overlap
        let a3: Cuboid = ((0, 0, 0), (10, 10, 10)).into();
        let b3: Cuboid = ((8, 8, 8), (12, 12, 12)).into();
        assert_eq!(
            a3.split(&b3),
            &[
                ((0, 0, 0), (7, 10, 10)).into(),
                ((8, 0, 0), (10, 7, 10)).into(),
                ((8, 8, 0), (10, 10, 7)).into(),
            ]
        );

        // Edge Touching
        let a4: Cuboid = ((0, 0, 0), (10, 10, 10)).into();
        let b4: Cuboid = ((10, 0, 0), (15, 5, 5)).into();
        assert_eq!(
            a4.split(&b4),
            &[
                ((0, 0, 0), (9, 10, 10)).into(),
                ((10, 6, 0), (10, 10, 10)).into(),
                ((10, 0, 6), (10, 5, 10)).into(),
            ]
        );
    }

    #[test]
    fn test_edge_touching() {
        let a: Cuboid = ((0, 0, 0), (10, 10, 10)).into();
        let b: Cuboid = ((10, 0, 0), (15, 5, 5)).into();
        let vol_bf = cubes_on_brute_force(&[(true, a), (true, b)]);
        assert_eq!(vol_bf, 1511);
        assert_eq!(a.volume(), 1331);
        assert_eq!(b.volume(), 216);
        let a_minus_b_vol: u64 = a.split(&b).iter().map(|c| c.volume()).sum();
        assert_eq!(a_minus_b_vol + b.volume(), vol_bf);
    }

    #[test]
    fn test_part1_brute_force() {
        assert_eq!(
            cubes_on_brute_force(&get_init_steps(&build(INPUT_TEST_1))),
            39
        );
        assert_eq!(
            cubes_on_brute_force(&get_init_steps(&build(INPUT_TEST_2))),
            590784
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(cubes_on(&get_init_steps(&build(INPUT_TEST_1))), 39);
        assert_eq!(cubes_on(&get_init_steps(&build(INPUT_TEST_2))), 590784);
        assert_eq!(cubes_on(&get_init_steps(&build(INPUT_TEST_3))), 474140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(cubes_on(&build(INPUT_TEST_3)), 2758514936282235);
    }
}
