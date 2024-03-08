use std::{
    cmp::Ordering,
    io::{self, Read},
};

use fxhash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    // x is the distance from the left edge and y is the distance from the top edge
    // Signed integers to make math easier.
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }

    fn distance(self, other: Coords) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

// We don't need to store all points, but just the positions of each asteroid.
fn build(input: &str) -> Vec<Coords> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                if c == '#' {
                    Some(Coords::new(col, row))
                } else {
                    None
                }
            })
        })
        .collect()
}

// Check if p is on the same line as p1-p2
fn is_same_line(p1: Coords, p2: Coords, p: Coords) -> bool {
    // The line equation
    (p.y - p1.y) * (p2.x - p1.x) == (p.x - p1.x) * (p2.y - p1.y)
}

// For each asteroid, we trace a line with all other asteroids.
// Then we look if any asteroid is on this line. If there are some, and there are further than the one we are checking,
// we remove them from our in sight list.
fn find_asteroids_in_sight(map: &[Coords], asteroid: Coords) -> Vec<Coords> {
    // A list of all other asteroids.
    // Using options so we can clear the ones we have found to be unreachable.
    let mut in_sight: Vec<Option<Coords>> = map
        .iter()
        .filter_map(|&c| if c == asteroid { None } else { Some(Some(c)) })
        .collect();

    // Check all other asteroids that may be in sight
    for i in 0..in_sight.len() {
        if let Some(ast_i) = in_sight[i] {
            // Finds the one that are on the line.
            for item_to_check in &mut in_sight {
                if let Some(ast_to_check) = item_to_check {
                    if is_same_line(asteroid, ast_i, *ast_to_check) {
                        let d_i2check = ast_i.distance(*ast_to_check);
                        let d_i = asteroid.distance(ast_i);
                        let d_check = asteroid.distance(*ast_to_check);
                        // If same side:
                        if d_i2check != d_i + d_check {
                            // Keep it only if it is closer.
                            if d_i < d_check {
                                *item_to_check = None;
                            }
                        }
                    }
                }
            }
        }
    }
    in_sight.iter().flatten().copied().collect()
}

fn best_position(map: &[Coords]) -> (Coords, usize) {
    let mut in_sight_count: FxHashMap<Coords, usize> = FxHashMap::default();
    for asteroid in map {
        let in_sight = find_asteroids_in_sight(map, *asteroid);
        in_sight_count.insert(*asteroid, in_sight.len());
    }

    let (best_coord, count) = in_sight_count.iter().max_by_key(|(_, v)| **v).unwrap();
    (*best_coord, *count)
}

// Orders the coordinates into which "squares" they are, as following:
// 3 | 0
// -------> x
// 2 | 1
//   v
//   y
#[allow(clippy::bool_to_int_with_if)]
fn which_square(x: i32, y: i32) -> u8 {
    if x >= 0 {
        if y <= 0 {
            0
        } else {
            1
        }
    } else if y >= 0 {
        2
    } else {
        3
    }
}

#[allow(clippy::cast_precision_loss)]
fn tangent(y: i32, x: i32) -> f32 {
    (y as f32 / x as f32).abs()
}

// Compares coords p1 and p2, relative to the center c.
#[allow(clippy::similar_names)]
fn cmp_coords(c: Coords, p1: Coords, p2: Coords) -> Ordering {
    let dx1 = p1.x - c.x;
    let dy1 = p1.y - c.y;
    let dx2 = p2.x - c.x;
    let dy2 = p2.y - c.y;
    let s1 = which_square(dx1, dy1);
    let s2 = which_square(dx2, dy2);
    if s1 == s2 {
        // If they are in same square, compare the tangent.
        let cmp_res = tangent(dy1, dx1).total_cmp(&tangent(dy2, dx2));
        match s1 {
            0 | 2 => cmp_res.reverse(),
            1 | 3 => cmp_res,
            _ => panic!("Invalid square"),
        }
    } else {
        // If they are in different squares, just order by squares.
        s1.cmp(&s2)
    }
}

fn vaporize_until(map: &[Coords], monitoring_location: Coords, nth: usize) -> Coords {
    let mut vaporizable_map = map.to_vec();
    loop {
        let mut round_to_vapor = find_asteroids_in_sight(&vaporizable_map, monitoring_location);
        round_to_vapor.sort_by(|a, b| cmp_coords(monitoring_location, *a, *b));
        let mut vaporized_pos = 0;
        for a in round_to_vapor {
            vaporizable_map.retain(|&x| x != a);
            vaporized_pos += 1;
            if vaporized_pos == nth {
                return a;
            }
        }
    }
}

fn asteroid_vaporized(map: &[Coords], monitoring_location: Coords, nth: usize) -> i32 {
    let last_vaporized_asteroid = vaporize_until(map, monitoring_location, nth);
    last_vaporized_asteroid.x * 100 + last_vaporized_asteroid.y
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = build(&input);

    let (best_coord, in_sight_count) = best_position(&map);
    println!("Part 1: {}", in_sight_count);
    println!("Part 2: {}", asteroid_vaporized(&map, best_coord, 200));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");
    const INPUT_TEST_5: &str = include_str!("../resources/input_test_5");

    #[test]
    fn test_best_position() {
        assert_eq!(best_position(&build(INPUT_TEST_1)), (Coords::new(3, 4), 8));
        assert_eq!(best_position(&build(INPUT_TEST_2)), (Coords::new(5, 8), 33));
        assert_eq!(best_position(&build(INPUT_TEST_3)), (Coords::new(1, 2), 35));
        assert_eq!(best_position(&build(INPUT_TEST_4)), (Coords::new(6, 3), 41));
        assert_eq!(
            best_position(&build(INPUT_TEST_5)),
            (Coords::new(11, 13), 210)
        );
    }

    const INPUT_TEST_6: &str = include_str!("../resources/input_test_6");

    #[test]
    fn test_vaporize_until() {
        let map = build(INPUT_TEST_6);
        let station = Coords::new(8, 3);
        assert_eq!(vaporize_until(&map, station, 4), Coords::new(10, 0));
    }

    #[test]
    fn test_asteroid_vaporized() {
        let map = build(INPUT_TEST_5);
        let station = best_position(&map).0;
        assert_eq!(asteroid_vaporized(&map, station, 200), 802);
    }
}
