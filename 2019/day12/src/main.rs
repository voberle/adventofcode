use std::{
    io::{self, Read},
    ops::{Index, IndexMut},
};

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

// We allow to access x,y,z by index, making it easier to manipulate them.
impl Index<usize> for Coords {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl IndexMut<usize> for Coords {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
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

#[allow(clippy::comparison_chain)]
fn apply_gravity(moons: &[Coords], velocities: &mut [Coords]) {
    for i in 0..velocities.len() {
        for j in i..velocities.len() {
            for index in 0..3 {
                if moons[i][index] < moons[j][index] {
                    velocities[i][index] += 1;
                    velocities[j][index] -= 1;
                } else if moons[i][index] > moons[j][index] {
                    velocities[i][index] -= 1;
                    velocities[j][index] += 1;
                }
            }
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
    }

    moons
        .iter()
        .zip(velocities.iter())
        .map(|(m, v)| m.energy() * v.energy())
        .sum::<i32>()
}

fn steps_to_reach_previous_state(moons: &[Coords]) -> usize {
    // The x of all moons positions and velocities depend on each others, same with y and z.
    // So we find when the x repeat, when the y repeat and when the z repeat.
    // And then we just take the Least Common Multiple of the 3.
    let mut states: [FxHashSet<Vec<i32>>; 3] = [
        FxHashSet::default(),
        FxHashSet::default(),
        FxHashSet::default(),
    ];
    let mut steps: [usize; 3] = [0; 3];

    let mut moons = moons.to_vec();
    let mut velocities = vec![Coords::zero(); moons.len()];
    for step in 0.. {
        apply_gravity(&moons, &mut velocities);
        apply_velocity(&mut moons, &velocities);

        for index in 0..3 {
            if steps[index] == 0 {
                // If we haven't found a period when they repeat.
                let mut key: Vec<i32> = moons.iter().map(|v| v[index]).collect();
                key.extend(velocities.iter().map(|v| v[index]));

                if !states[index].insert(key) {
                    steps[index] = step;
                }
            }
        }

        if steps.iter().all(|v| *v != 0) {
            // Stop once we have the 3 periods.
            break;
        }
    }

    // LCM.
    steps.iter().fold(1, |n, i| num_integer::lcm(n, *i))
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
        assert_eq!(
            steps_to_reach_previous_state(&build(INPUT_TEST_2)),
            4686774924
        );
    }
}
