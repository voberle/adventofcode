// https://adventofcode.com/2023/day/24

use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

struct Vel {
    x: i64,
    y: i64,
    z: i64,
}

struct Hailstone {
    p: Pos,
    v: Vel,
}

impl Hailstone {
    fn new(x: i64, y: i64, z: i64, vx: i64, vy: i64, vz: i64) -> Self {
        Self {
            p: Pos { x, y, z },
            v: Vel {
                x: vx,
                y: vy,
                z: vz,
            },
        }
    }

    //  y - y1 = m(x - x1)
    // becomes
    //  m*x - y  + (y1 - m*x1) = 0
    // with m = vy / vx.
    // For equations like:
    //  a1x + b1y + c1 = 0 and a2x + b2y + c2 = 0,
    // Intersection is:
    //  (x, y) = ((b1c2-b2c1)/(a1b2-a2b1), (c1a2-c2a1)/(a1b2-a2b1))
    fn intersection(h1: &Hailstone, h2: &Hailstone) -> Option<(f64, f64)> {
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

    fn crosses(&self, b: &Hailstone) -> Option<(f64, f64)> {
        Self::intersection(self, b)
    }

    fn crosses_in_area(
        &self,
        b: &Hailstone,
        x_area: &RangeInclusive<f64>,
        y_area: &RangeInclusive<f64>,
    ) -> bool {
        if let Some((x, y)) = Self::intersection(self, b) {
            x_area.contains(&x) && y_area.contains(&y)
        } else {
            false
        }
    }

    fn crosses_in_future(&self, b: &Hailstone, cross: &(f64, f64)) -> bool {
        (cross.0 - self.p.x as f64) * self.v.x as f64 > 0.0
            && (cross.1 - self.p.y as f64) * self.v.y as f64 > 0.0
            && (cross.0 - b.p.x as f64) * b.v.x as f64 > 0.0
            && (cross.1 - b.p.y as f64) * b.v.y as f64 > 0.0
    }

    fn crosses_in_area_and_future(
        &self,
        b: &Hailstone,
        x_area: &RangeInclusive<f64>,
        y_area: &RangeInclusive<f64>,
    ) -> bool {
        if let Some((x, y)) = Self::intersection(self, b) {
            x_area.contains(&x) && y_area.contains(&y) && self.crosses_in_future(b, &(x, y))
        } else {
            false
        }
    }
}

#[test]
fn test_intersection() {
    // X and Y position each at least 7 and at most 27
    let x_area = 7f64..=27f64;
    let y_area = 7f64..=27f64;

    let mut a = Hailstone::new(19, 13, 30, -2, 1, -2);
    let mut b = Hailstone::new(18, 19, 22, -1, -1, -2);
    // Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).
    assert_eq!(a.crosses(&b), Some((14.333, 15.333)));
    assert!(a.crosses_in_area(&b, &x_area, &y_area));
    assert!(a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(19, 13, 30, -2, 1, -2);
    b = Hailstone::new(20, 25, 34, -2, -2, -4);
    // Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).
    assert_eq!(a.crosses(&b), Some((11.667, 16.667)));
    assert!(a.crosses_in_area(&b, &x_area, &y_area));
    assert!(a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(19, 13, 30, -2, 1, -2);
    b = Hailstone::new(12, 31, 28, -1, -2, -1);
    // Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).
    assert_eq!(a.crosses(&b), Some((6.2, 19.4)));
    assert!(!a.crosses_in_area(&b, &x_area, &y_area));
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(19, 13, 30, -2, 1, -2);
    b = Hailstone::new(20, 19, 15, 1, -5, -3);
    // Hailstones' paths crossed in the past for hailstone A.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(18, 19, 22, -1, -1, -2);
    b = Hailstone::new(20, 25, 34, -2, -2, -4);
    // Hailstones' paths are parallel; they never intersect.
    assert_eq!(a.crosses(&b), None);
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(18, 19, 22, -1, -1, -2);
    b = Hailstone::new(12, 31, 28, -1, -2, -1);
    // Hailstones' paths will cross outside the test area (at x=-6, y=-5).
    assert_eq!(a.crosses(&b), Some((-6.0, -5.0)));
    assert!(!a.crosses_in_area(&b, &x_area, &y_area));
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(18, 19, 22, -1, -1, -2);
    b = Hailstone::new(20, 19, 15, 1, -5, -3);
    // Hailstones' paths crossed in the past for both hailstones.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(20, 25, 34, -2, -2, -4);
    b = Hailstone::new(12, 31, 28, -1, -2, -1);
    // Hailstones' paths will cross outside the test area (at x=-2, y=3).
    assert_eq!(a.crosses(&b), Some((-2.0, 3.0)));
    assert!(!a.crosses_in_area(&b, &x_area, &y_area));
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(20, 25, 34, -2, -2, -4);
    b = Hailstone::new(20, 19, 15, 1, -5, -3);
    // Hailstones' paths crossed in the past for hailstone B.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));

    a = Hailstone::new(12, 31, 28, -1, -2, -1);
    b = Hailstone::new(20, 19, 15, 1, -5, -3);
    // Hailstones' paths crossed in the past for both hailstones.
    assert!(!a.crosses_in_area_and_future(&b, &x_area, &y_area));
}

fn count_crossing_hailstones(hailstones: &Vec<Hailstone>, area: &RangeInclusive<f64>) -> i64 {
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if hailstones[i].crosses_in_area_and_future(&hailstones[j], &area, &area) {
                count += 1;
            }
        }
    }
    count
}

fn build_hailstones<R>(reader: &mut R) -> Vec<Hailstone>
where
    R: BufRead,
{
    let mut hailstones = Vec::new();
    for l in reader.lines() {
        let line = l.unwrap();
        let pv: Vec<&str> = line.split(" @ ").collect();
        let pos: Vec<i64> = pv[0]
            .split(", ")
            .map(|v| v.trim().parse().unwrap())
            .collect();
        let vel: Vec<i64> = pv[1]
            .split(", ")
            .map(|v| v.trim().parse::<i64>().unwrap())
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
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let hailstones = build_hailstones(&mut reader);
        let area = 7f64..=27f64;
        assert_eq!(count_crossing_hailstones(&hailstones, &area), 2);
    }
}
