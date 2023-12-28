// https://adventofcode.com/2023/day/24

use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}
#[derive(Debug, Clone)]
struct Vel {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
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

    // Returns the position of the hailstone at the specified time
    fn pos_at(&self, at: i64) -> Pos {
        Pos {
            x: self.p.x + at * self.v.x,
            y: self.p.y + at * self.v.y,
            z: self.p.z + at * self.v.z,
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

fn count_crossing_hailstones(hailstones: &[Hailstone], area: &RangeInclusive<f64>) -> i64 {
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

// Returns the vector for these two points.
// https://math.stackexchange.com/questions/947555/how-to-determine-if-3-points-on-a-3-d-graph-are-collinear
fn vector_for(a: &Pos, b: &Pos) -> Pos {
    Pos {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    }
}

// Check if the cross-product of the two vectors is 0. This means the vectors are on the same line.
// https://en.wikipedia.org/wiki/Cross_product#Coordinate_notation
fn is_vector_cross_product_zero(ab: &Pos, ac: &Pos) -> bool {
    let ab_x = ab.x as i128;
    let ab_y = ab.y as i128;
    let ab_z = ab.z as i128;
    let ac_x = ac.x as i128;
    let ac_y = ac.y as i128;
    let ac_z = ac.z as i128;
    let (a1, a2, a3) = (ab_x, ab_y, ab_z);
    let (b1, b2, b3) = (ac_x, ac_y, ac_z);
    let s1 = a2.clone() * b3.clone() - a3.clone() * b2.clone();
    let s2 = a3.clone() * b1.clone() - a1.clone() * b3.clone();
    let s3 = a1.clone() * b2.clone() - a2.clone() * b1.clone();
    s1 == 0 && s2 == 0 && s3 == 0
}

// Check if the list of points are on the same line.
fn are_points_aligned(points: &[Pos]) -> bool {
    assert!(points.len() > 2);
    let a = &points[0];
    let b = &points[1];
    let ab = vector_for(&a, &b);
    for _ in 2..points.len() {
        let c = &points[2];
        let ac = vector_for(&a, &c);
        if !is_vector_cross_product_zero(&ab, &ac) {
            return false;
        }
    }
    true
}

fn are_points_ref_aligned(points: &[&Pos]) -> bool {
    assert!(points.len() > 2);
    let a = points[0];
    let b = points[1];
    let ab = vector_for(&a, &b);
    for _ in 2..points.len() {
        let c = points[2];
        let ac = vector_for(&a, &c);
        if !is_vector_cross_product_zero(&ab, &ac) {
            return false;
        }
    }
    true
}

// Theory: We only need to find when 3 hailstones would be aligned
fn find_alignment<const MAX: i64>(hailstones: &[Hailstone]) -> Option<([Pos; 3], [i64; 3])> {
    assert_eq!(hailstones.len(), 3);
    for ta in 0..MAX {
        println!("Progress {ta}");
        let a = hailstones[0].pos_at(ta as i64);
        for tb in 0..MAX {
            let b = hailstones[1].pos_at(tb as i64);
            for tc in 0..MAX {
                let c = hailstones[2].pos_at(tc as i64);
                let p = [&a, &b, &c];
                // println!("Checking {:?} at times {:?}", p, (ta, tb, tc));
                if are_points_ref_aligned(&p) {
                    println!("Found aligned {:?} at times {:?}", p, (ta, tb, tc));
                    return Some(([a, b, c], [ta, tb, tc]));
                }
            }
        }
    }
    None
}

fn perfect_colision_initial_pos(hailstones: &[Hailstone]) -> i64 {
    // let a = Hailstone::new(81036585584730, 57155609328680, 97064111364402, 148, 211, 304);
    // let b = Hailstone::new(87348042064677, 28402136204210, 217221456953281, 105, 205, 165);
    // let c = Hailstone::new(279724917845922, 250956079438084, 6340365342173, 220, 327, 891);
    // let subset = [a, b, c];
    let subset = &hailstones[0..3];
    println!("{:?}", subset);

    if let Some(result) = find_alignment::<2000>(&subset) {
        println!("{:?}", result);
        return 0;
    }
    0
}

#[test]
fn test_find_alignment() {
    let a = Hailstone::new(19, 13, 30, -2, 1, -2);
    let b = Hailstone::new(18, 19, 22, -1, -1, -2);
    let c = Hailstone::new(20, 25, 34, -2, -2, -4);

    let a_f = Pos::new(9, 18, 20);
    let b_f = Pos::new(15, 16, 16);
    let c_f = Pos::new(12, 17, 18);

    assert_eq!(find_alignment::<20>(&[a, b, c]), Some(([a_f, b_f, c_f], [5, 3, 4])));
}

#[test]
fn test_collision() {
    let a = Hailstone::new(19, 13, 30, -2, 1, -2);
    let a_f = a.pos_at(5);
    assert_eq!(a_f, Pos::new(9, 18, 20));

    let b = Hailstone::new(18, 19, 22, -1, -1, -2);
    let b_f = b.pos_at(3);
    assert_eq!(b_f, Pos::new(15, 16, 16));

    let c = Hailstone::new(20, 25, 34, -2, -2, -4);
    let c_f = c.pos_at(4);
    assert_eq!(c_f, Pos::new(12, 17, 18));

    let d = Hailstone::new(12, 31, 28, -1, -2, -1);
    let d_f = d.pos_at(6);
    assert_eq!(d_f, Pos::new(6, 19, 22));

    let e = Hailstone::new(20, 19, 15, 1, -5, -3);
    let e_f = e.pos_at(1);
    assert_eq!(e_f, Pos::new(21, 14, 12));

    let ab = vector_for(&a_f, &b_f);
    let ac = vector_for(&a_f, &c_f);
    assert!(is_vector_cross_product_zero(&ab, &ac));

    let points = [a_f, b_f, c_f, d_f, e_f];
    assert!(are_points_aligned(&points));
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

    // hailstones.sort_by_key(|h| h.p.x);

    println!("Part 1: {}", count_crossing_hailstones(&hailstones, &area));

    println!("Part 2: {}", perfect_colision_initial_pos(&hailstones));
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
