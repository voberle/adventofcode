use std::{
    io::{self, Read},
    usize,
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
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(self, other: Coords) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

// We don't need to store all points, but just the positions of each asteroid.
struct Map(Vec<Coords>);

impl Map {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().map(|c| c == '#').collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;

        let pos: Vec<Coords> = values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if *v {
                    let row = i32::try_from(i / cols).unwrap();
                    let col = i32::try_from(i % cols).unwrap();
                    Some(Coords::new(col, row))
                } else {
                    None
                }
            })
            .collect();
        Self(pos)
    }
}

// Check if p is on the same line as p1-p2
fn is_same_line(p1: Coords, p2: Coords, p: Coords) -> bool {
    // The line equation
    (p.y - p1.y) * (p2.x - p1.x) == (p.x - p1.x) * (p2.y - p1.y)
}

// For each asteroid, we trace a line with all other asteroids.
// Then we look if any asteroid is on this line. If there are some, and there are further than the one we are checking,
// we remove them from our in sight list.
fn find_asteroids_in_sight(map: &Map, asteroid: Coords) -> Vec<Option<Coords>> {
    // A list of all other asteroids.
    // Using options so we can clear the ones we have found to be unreachable.
    let mut in_sight: Vec<Option<Coords>> = map
        .0
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
    // println!("{:?} => {:?}", asteroid, in_sight);
    in_sight
}

fn best_position(map: &Map) -> (Coords, usize) {
    let mut in_sight_count: FxHashMap<Coords, usize> = FxHashMap::default();
    for asteroid in &map.0 {
        let in_sight = find_asteroids_in_sight(map, *asteroid);
        in_sight_count.insert(*asteroid, in_sight.iter().filter(|v| v.is_some()).count());
    }

    let (best_coord, count) = in_sight_count.iter().max_by_key(|(_, v)| **v).unwrap();
    (*best_coord, *count)
}

fn asteroid_vaporized(map: &Map, monitoring_loc: Coords, nth: usize) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Map::build(&input);

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
        assert_eq!(
            best_position(&Map::build(INPUT_TEST_1)),
            (Coords::new(3, 4), 8)
        );
        assert_eq!(
            best_position(&Map::build(INPUT_TEST_2)),
            (Coords::new(5, 8), 33)
        );
        assert_eq!(
            best_position(&Map::build(INPUT_TEST_3)),
            (Coords::new(1, 2), 35)
        );
        assert_eq!(
            best_position(&Map::build(INPUT_TEST_4)),
            (Coords::new(6, 3), 41)
        );
        assert_eq!(
            best_position(&Map::build(INPUT_TEST_5)),
            (Coords::new(11, 13), 210)
        );
    }

    #[test]
    fn test_asteroid_vaporized() {
        let map = Map::build(INPUT_TEST_5);
        let best_coord = best_position(&map).0;
        assert_eq!(asteroid_vaporized(&map, best_coord, 200), 802);
    }
}
