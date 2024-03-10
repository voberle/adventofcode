use std::io::{self, Read};

use fxhash::FxHashSet;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

impl Coords {
    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

fn build(input: &str) -> Vec<Coords> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    input
        .lines()
        .map(|line| {
            let p = re.captures(line).unwrap();
            Coords {
                x: p[1].parse().unwrap(),
                y: p[2].parse().unwrap(),
                z: p[3].parse().unwrap(),
            }
        })
        .collect()
}

fn apply_gravity(moons: &[Coords], velocities: &mut [Coords]) {
    for i in 0..velocities.len() {
        for j in i..velocities.len() {
            macro_rules! adjust_velocity {
                ($dim:ident) => {
                    if moons[i].$dim < moons[j].$dim {
                        velocities[i].$dim += 1;
                        velocities[j].$dim -= 1;
                    } else if moons[i].$dim > moons[j].$dim {
                        velocities[i].$dim -= 1;
                        velocities[j].$dim += 1;
                    }
                };
            }

            adjust_velocity!(x);
            adjust_velocity!(y);
            adjust_velocity!(z);
        }
    }
}

fn apply_velocity(moons: &mut [Coords], velocities: &[Coords]) {
    for (m, v) in moons.iter_mut().zip(velocities.iter()) {
        m.x += v.x;
        m.y += v.y;
        m.z += v.z;
    }
}

fn total_energy(moons: &[Coords], steps: usize) -> i32 {
    let mut moons = moons.to_vec();
    let mut velocities = vec![Coords::zero(); moons.len()];
    for _ in 0..steps {
        apply_gravity(&moons, &mut velocities);
        apply_velocity(&mut moons, &velocities);

        // for i in 0..moons.len() {
        //     println!("{:?} {:?}", moons[i], velocities[i]);
        // }
    }
    moons
        .iter()
        .zip(velocities.iter())
        .map(|(m, v)| m.energy() * v.energy())
        .sum::<i32>()
}

fn steps_to_reach_previous_state(moons: &[Coords]) -> usize {
    let mut states: FxHashSet<(Vec<Coords>, Vec<Coords>)> = FxHashSet::default();

    let mut moons = moons.to_vec();
    let mut velocities = vec![Coords::zero(); moons.len()];
    for step in 0.. {
        apply_gravity(&moons, &mut velocities);
        apply_velocity(&mut moons, &velocities);

        if !states.insert((moons.clone(), velocities.clone())) {
            return step;
        }
    }
    panic!("No previous state reached");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let moons = build(&input);

    println!("Part 1: {}", total_energy(&moons, 1000));
    println!("Part 2: {}", steps_to_reach_previous_state(&moons));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(total_energy(&build(INPUT_TEST_1), 10), 179);
        assert_eq!(total_energy(&build(INPUT_TEST_2), 100), 1940);
    }

    #[test]
    fn test_part2() {
        assert_eq!(steps_to_reach_previous_state(&build(INPUT_TEST_1)), 2772);
    }
}
