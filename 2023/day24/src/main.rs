// https://adventofcode.com/2023/day/24

use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

// Position in 3D
#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: i128,
    y: i128,
    z: i128,
}

#[cfg(test)]
impl Pos {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Vel {
    x: i128,
    y: i128,
    z: i128,
}

#[derive(Debug, Clone)]
struct Hailstone {
    p: Pos,
    v: Vel,
}

impl Hailstone {
    fn new(x: i128, y: i128, z: i128, vx: i128, vy: i128, vz: i128) -> Self {
        Self {
            p: Pos { x, y, z },
            v: Vel {
                x: vx,
                y: vy,
                z: vz,
            },
        }
    }

    // Project the hailstone as one on the X-Y plane
    fn project_xy(&self) -> Hailstone2d {
        Hailstone2d {
            p: Pos2d {
                x: self.p.x,
                y: self.p.y,
            },
            v: Vel2d {
                x: self.v.x,
                y: self.v.y,
            },
        }
    }

    // Project the hailstone as one on the X-Z plane
    fn project_xz(&self) -> Hailstone2d {
        Hailstone2d {
            p: Pos2d {
                x: self.p.x,
                y: self.p.z,
            },
            v: Vel2d {
                x: self.v.x,
                y: self.v.z,
            },
        }
    }
}

// The whole thing projected on a plane

#[derive(Debug, Clone, PartialEq)]
struct Pos2d {
    x: i128,
    y: i128,
}

#[derive(Debug, Clone)]
struct Vel2d {
    x: i128,
    y: i128,
}

#[derive(Debug, Clone)]
struct Hailstone2d {
    p: Pos2d,
    v: Vel2d,
}

impl Hailstone2d {
    //  y - y1 = m(x - x1)
    // becomes
    //  m*x - y  + (y1 - m*x1) = 0
    // with m = vy / vx.
    // For equations like:
    //  a1x + b1y + c1 = 0 and a2x + b2y + c2 = 0,
    // Intersection is:
    //  (x, y) = ((b1c2-b2c1)/(a1b2-a2b1), (c1a2-c2a1)/(a1b2-a2b1))
    fn intersection(h1: &Hailstone2d, h2: &Hailstone2d) -> Option<(f64, f64)> {
        let m1: f64 = h1.v.y as f64 / h1.v.x as f64;
        let a1: f64 = m1;
        let b1: f64 = -1.0;
        let c1: f64 = h1.p.y as f64 - m1 * h1.p.x as f64;

        let m2: f64 = h2.v.y as f64 / h2.v.x as f64;
        let a2: f64 = m2;
        let b2: f64 = -1.0;
        let c2: f64 = h2.p.y as f64 - m2 * h2.p.x as f64;

        let den = a1 * b2 - a2 * b1;
        if den == 0f64 {
            None
        } else {
            Some((
                Self::round_to_3((b1 * c2 - b2 * c1) / den),
                Self::round_to_3((c1 * a2 - c2 * a1) / den),
            ))
        }
    }

    fn round_to_3(v: f64) -> f64 {
        (v * 1000.0).round() / 1000.0
    }

    #[cfg(test)]
    fn crosses(&self, b: &Hailstone2d) -> Option<(f64, f64)> {
        Self::intersection(self, b)
    }

    #[cfg(test)]
    fn crosses_in_area(
        &self,
        b: &Hailstone2d,
        x_area: &RangeInclusive<f64>,
        y_area: &RangeInclusive<f64>,
    ) -> bool {
        if let Some((x, y)) = Self::intersection(self, b) {
            x_area.contains(&x) && y_area.contains(&y)
        } else {
            false
        }
    }

    fn crosses_in_future(&self, b: &Hailstone2d, cross: &(f64, f64)) -> bool {
        (cross.0 - self.p.x as f64) * self.v.x as f64 > 0.0
            && (cross.1 - self.p.y as f64) * self.v.y as f64 > 0.0
            && (cross.0 - b.p.x as f64) * b.v.x as f64 > 0.0
            && (cross.1 - b.p.y as f64) * b.v.y as f64 > 0.0
    }

    fn crosses_in_area_and_future(
        &self,
        b: &Hailstone2d,
        x_area: &RangeInclusive<f64>,
        y_area: &RangeInclusive<f64>,
    ) -> bool {
        if let Some((x, y)) = Self::intersection(self, b) {
            x_area.contains(&x) && y_area.contains(&y) && self.crosses_in_future(b, &(x, y))
        } else {
            false
        }
    }

    // Does this hailstone pass by the specified position
    fn crosses_pos(&self, pos: &Pos2d) -> bool {
        let diff = (pos.x - self.p.x) * self.v.y - (pos.y - self.p.y) * self.v.x;
        diff == 0
    }
}

#[test]
fn test_intersection() {
    // X and Y position each at least 7 and at most 27
    let x_area = 7f64..=27f64;
    let y_area = 7f64..=27f64;

    let mut a = Hailstone::new(19, 13, 30, -2, 1, -2).project_xy();
    let mut b = Hailstone::new(18, 19, 22, -1, -1, -2).project_xy();
    // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
    assert_eq!(a.crosses(&b), Some((14.333, 15.333)));
    assert!(a.crosses_in_area(&b, &x_area, &y_area));
    assert!(a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(19, 13, 30, -2, 1, -2).project_xy();
    b = Hailstone::new(20, 25, 34, -2, -2, -4).project_xy();
    // Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).
    assert_eq!(a.crosses(&b), Some((11.667, 16.667)));
    assert!(a.crosses_in_area(&b, &x_area, &y_area));
    assert!(a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(19, 13, 30, -2, 1, -2).project_xy();
    b = Hailstone::new(12, 31, 28, -1, -2, -1).project_xy();
    // Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).
    assert_eq!(a.crosses(&b), Some((6.2, 19.4)));
    assert!(!a.crosses_in_area(&b, &x_area, &y_area));
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(19, 13, 30, -2, 1, -2).project_xy();
    b = Hailstone::new(20, 19, 15, 1, -5, -3).project_xy();
    // Hailstones' paths crossed in the past for hailstone A.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(18, 19, 22, -1, -1, -2).project_xy();
    b = Hailstone::new(20, 25, 34, -2, -2, -4).project_xy();
    // Hailstones' paths are parallel; they never intersect.
    assert_eq!(a.crosses(&b), None);
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(18, 19, 22, -1, -1, -2).project_xy();
    b = Hailstone::new(12, 31, 28, -1, -2, -1).project_xy();
    // Hailstones' paths will cross outside the test area (at x=-6, y=-5).
    assert_eq!(a.crosses(&b), Some((-6.0, -5.0)));
    assert!(!a.crosses_in_area(&b, &x_area, &y_area));
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(18, 19, 22, -1, -1, -2).project_xy();
    b = Hailstone::new(20, 19, 15, 1, -5, -3).project_xy();
    // Hailstones' paths crossed in the past for both hailstones.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(20, 25, 34, -2, -2, -4).project_xy();
    b = Hailstone::new(12, 31, 28, -1, -2, -1).project_xy();
    // Hailstones' paths will cross outside the test area (at x=-2, y=3).
    assert_eq!(a.crosses(&b), Some((-2.0, 3.0)));
    assert!(!a.crosses_in_area(&b, &x_area, &y_area));
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(20, 25, 34, -2, -2, -4).project_xy();
    b = Hailstone::new(20, 19, 15, 1, -5, -3).project_xy();
    // Hailstones' paths crossed in the past for hailstone B.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(12, 31, 28, -1, -2, -1).project_xy();
    b = Hailstone::new(20, 19, 15, 1, -5, -3).project_xy();
    // Hailstones' paths crossed in the past for both hailstones.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));
}

fn project_xy(hailstones: &[Hailstone]) -> Vec<Hailstone2d> {
    hailstones.iter().map(Hailstone::project_xy).collect()
}

fn project_xz(hailstones: &[Hailstone]) -> Vec<Hailstone2d> {
    hailstones.iter().map(Hailstone::project_xz).collect()
}

// Part 1
fn count_crossing_hailstones(hailstones: &[Hailstone], area: &RangeInclusive<f64>) -> i128 {
    let hailstones2d = project_xy(hailstones);
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if hailstones2d[i].crosses_in_area_and_future(&hailstones2d[j], area, area) {
                count += 1;
            }
        }
    }
    count
}

fn change_hailstone_to_rock_still_reference(
    hailstone: &Hailstone2d,
    velocity: &Vel2d,
) -> Hailstone2d {
    Hailstone2d {
        p: hailstone.p.clone(),
        v: Vel2d {
            x: hailstone.v.x - velocity.x,
            y: hailstone.v.y - velocity.y,
        },
    }
}

fn find_collision_in_2d<const RANGE: i128>(hailstones: &[Hailstone2d]) -> Pos2d {
    for v1 in -RANGE..RANGE {
        for v2 in -RANGE..RANGE {
            let vel = Vel2d { x: v1, y: v2 };

            if let Some(inter) = Hailstone2d::intersection(
                &change_hailstone_to_rock_still_reference(&hailstones[0], &vel),
                &change_hailstone_to_rock_still_reference(&hailstones[1], &vel),
            ) {
                let rock = Pos2d {
                    x: inter.0 as i128,
                    y: inter.1 as i128,
                };

                if hailstones.iter().all(|h| {
                    let h_rock_ref = change_hailstone_to_rock_still_reference(h, &vel);
                    h_rock_ref.crosses_pos(&rock)
                }) {
                    return rock;
                }
            }
        }
    }
    panic!("Didn't work :-(");
}

fn perfect_collision_initial_pos(hailstones: &[Hailstone]) -> i128 {
    let hailstones_on_xy = project_xy(hailstones);
    let rock_xy = find_collision_in_2d::<500>(&hailstones_on_xy);

    let hailstones_on_xz = project_xz(hailstones);
    let rock_xz = find_collision_in_2d::<500>(&hailstones_on_xz);

    rock_xy.x + rock_xy.y + rock_xz.y
}

fn build_hailstones<R>(reader: &mut R) -> Vec<Hailstone>
where
    R: BufRead,
{
    let mut hailstones = Vec::new();
    for l in reader.lines() {
        let line = l.unwrap();
        let pv: Vec<&str> = line.split(" @ ").collect();
        let pos: Vec<i128> = pv[0]
            .split(", ")
            .map(|v| v.trim().parse().unwrap())
            .collect();
        let vel: Vec<i128> = pv[1]
            .split(", ")
            .map(|v| v.trim().parse::<i128>().unwrap())
            .collect();
        hailstones.push(Hailstone::new(
            pos[0], pos[1], pos[2], vel[0], vel[1], vel[2],
        ));
    }
    hailstones
}

fn main() {
    let stdin = io::stdin();
    let hailstones = build_hailstones(&mut stdin.lock());
    let area = 200000000000000f64..=400000000000000f64;

    println!("Part 1: {}", count_crossing_hailstones(&hailstones, &area));

    println!("Part 2: {}", perfect_collision_initial_pos(&hailstones));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1_2() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let hailstones = build_hailstones(&mut reader);
        let area = 7f64..=27f64;
        assert_eq!(count_crossing_hailstones(&hailstones, &area), 2);

        assert_eq!(perfect_collision_initial_pos(&hailstones), 47);
    }
}
