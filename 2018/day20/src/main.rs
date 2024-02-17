use std::{
    fmt,
    io::{self, Read},
};

use fxhash::FxHashMap;

mod graphnode;
mod trivial;

// In this exercise, we try reading the input file as bytes instead of a string.
// This functions helps with debugging.
#[allow(dead_code)]
fn regex_to_string(regex: &[u8]) -> String {
    regex.iter().map(|c| *c as char).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

impl Direction {
    fn new(c: u8) -> Option<Self> {
        match c {
            b'N' => Some(North),
            b'E' => Some(East),
            b'S' => Some(South),
            b'W' => Some(West),
            _ => None,
        }
    }

    fn index(self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }

    fn opposite(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                North => 'N',
                East => 'E',
                South => 'S',
                West => 'W',
            }
        )
    }
}

// A position on the map.
// x represents the columns (east means positive x).
// y represents the rows (south means positive y).
// x=0 y=0 is the starting position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next(self, dir: Direction) -> Self {
        match dir {
            North => Self {
                x: self.x,
                y: self.y - 1,
            },
            East => Self {
                x: self.x + 1,
                y: self.y,
            },
            South => Self {
                x: self.x,
                y: self.y + 1,
            },
            West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

// The map is represented by a HashMap of positions to where we can go from these positions.
// Indexes are the Direction `index()` values.
struct Map(FxHashMap<Pos, [bool; 4]>);

impl Map {
    fn new() -> Self {
        Self(FxHashMap::default())
    }

    // Returns min x, max x, min y, max y.
    fn borders(&self) -> (i32, i32, i32, i32) {
        // Not using iterator min / max to keep only one loop.
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for Pos { x, y } in self.0.keys() {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
        (min_x, max_x, min_y, max_y)
    }

    fn update(&mut self, pos: Pos, dir: Direction) {
        self.0
            .entry(pos)
            .and_modify(|e| e[dir.index()] = true)
            .or_insert({
                let mut allowed_dir = [false, false, false, false];
                allowed_dir[dir.index()] = true;
                allowed_dir
            });
    }
}

// Map building directly from the regex.
fn build_map_from_regex(regex: &[u8]) -> Map {
    let mut map = Map::new();
    explore_map_from_regex(regex, &mut 1, &mut map, Pos::new(0, 0));
    map
}

fn explore_map_from_regex(regex: &[u8], index: &mut usize, map: &mut Map, mut pos: Pos) {
    loop {
        match regex[*index] {
            b'|' | b')' | b'$' => break,
            b'(' => {
                while regex[*index] != b')' {
                    *index += 1;
                    explore_map_from_regex(regex, index, map, pos);
                }
            }
            dir => {
                let dir = Direction::new(dir).unwrap();
                // We can go in that direction from current position.
                map.update(pos, dir);
                // From next position, we can go back.
                pos = pos.next(dir);
                map.update(pos, dir.opposite());
            }
        }
        *index += 1;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.borders();
        let width = (((max_x - min_x) + 1) * 2 + 1) as usize;

        writeln!(f, "{:#<1$}", "", width)?;
        for y in min_y..=max_y {
            write!(f, "#")?;
            for x in min_x..=max_x {
                if let Some(val) = self.0.get(&Pos::new(x, y)) {
                    write!(f, "{}", if x == 0 && y == 0 { 'X' } else { '.' })?;
                    write!(f, "{}", if val[East.index()] { "|" } else { "#" })?;
                } else {
                    // Doesn't happen on a "pure" map, that is a nice rectangle
                    write!(f, "  ")?;
                }
            }
            writeln!(f)?;

            write!(f, "#")?;
            for x in min_x..=max_x {
                if let Some(val) = self.0.get(&Pos::new(x, y)) {
                    write!(f, "{}", if val[South.index()] { "-" } else { "#" })?;

                    write!(f, "#")?;
                } else {
                    write!(f, "  ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn dist_to_furthest_room(regex: &[u8]) -> usize {
    0
}

fn rooms_dist_over_1000_doors(regex: &[u8]) -> usize {
    0
}

fn main() {
    let mut regex = Vec::new();
    io::stdin().read_to_end(&mut regex).unwrap();

    println!("Part 1: {}", graphnode::dist_to_furthest_room(&regex));
    println!("Part 2: {}", graphnode::rooms_dist_over_1000_doors(&regex));
    // trivial_version(&regex);
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const INPUT_TEST_1: &[u8; 5] = include_bytes!("../resources/input_test_1");
    pub const INPUT_TEST_2: &[u8; 23] = include_bytes!("../resources/input_test_2");
    pub const INPUT_TEST_3: &[u8; 41] = include_bytes!("../resources/input_test_3");
    pub const INPUT_TEST_4: &[u8; 51] = include_bytes!("../resources/input_test_4");
    pub const INPUT_TEST_5: &[u8; 65] = include_bytes!("../resources/input_test_5");
    pub const INPUT_TEST_6: &[u8; 23] = include_bytes!("../resources/input_test_6");

    #[test]
    fn test_part1() {
        assert_eq!(dist_to_furthest_room(INPUT_TEST_1), 3);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_2), 10);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_3), 18);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_4), 23);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_5), 31);
    }
}
