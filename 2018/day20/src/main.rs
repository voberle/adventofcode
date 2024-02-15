use std::io::{self, Read};

use fxhash::FxHashMap;

fn regex_to_string(regex: &[u8]) -> String {
    regex.iter().map(|c| *c as char).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
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

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

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
fn build_map(regex: &[u8]) -> FxHashMap<Pos, [bool; 4]> {
    // While building, "false" in the allowed array actually means "maybe"
    let mut map: FxHashMap<Pos, [bool; 4]> = FxHashMap::default();

    // Ignoring first ^ and last $
    let regex = &regex[1..regex.len() - 1];

    let mut pos = Pos::new(0, 0);
    map.insert(pos, [false, false, false, false]);

    for c in regex {
        if let Some(dir) = Direction::new(*c) {
            // We can go in that direction from current position.
            map.get_mut(&pos).unwrap()[dir.index()] = true;
            // From next position, we can go back.
            pos = pos.next(dir);
            let mut allowed_dir = [false, false, false, false];
            allowed_dir[dir.opposite().index()] = true;
            map.insert(pos, allowed_dir);
        }
    }

    map
}

// Returns min x, max x, min y, max y.
fn map_borders(map: &FxHashMap<Pos, [bool; 4]>) -> (i32, i32, i32, i32) {
    // Not using iterator min / max to keep only one loop.
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for Pos { x, y } in map.keys() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    (min_x, max_x, min_y, max_y)
}

fn print_map(map: &FxHashMap<Pos, [bool; 4]>) {
    let (min_x, max_x, min_y, max_y) = map_borders(map);
    println!("{:#<1$}", "", (max_x - min_y + 4) as usize);
    for y in min_y..=max_y {
        print!("#");
        for x in min_x..=max_x {
            let val = map.get(&Pos::new(x, y)).unwrap();
            print!("{}", if x == 0 && y == 0 { 'X' } else { '.' });
            if val[East.index()] {
                print!("|");
            } else {
                print!("#");
            }
        }
        println!();

        print!("#");
        for x in min_x..=max_x {
            let val = map.get(&Pos::new(x, y)).unwrap();
            if val[South.index()] {
                print!("-");
            } else {
                print!("#");
            }
            print!("#");
        }
        println!();
    }
    // println!("{:#<1$}", "", (max_x - min_y + 3) as usize);
}

// Largest number of doors required to pass through to reach a room.
fn dist_to_furthest_room(regex: &[u8]) -> usize {
    let map = build_map(regex);
    print_map(&map);

    0
}

fn part2(regex: &[u8]) -> usize {
    0
}

fn main() {
    let mut regex = Vec::new();
    io::stdin().read_to_end(&mut regex).unwrap();

    println!("Part 1: {}", dist_to_furthest_room(&regex));
    println!("Part 2: {}", part2(&regex));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_test_1 = include_bytes!("../resources/input_test_1");
        let input_test_2 = include_bytes!("../resources/input_test_2");
        let input_test_3 = include_bytes!("../resources/input_test_3");
        let input_test_4 = include_bytes!("../resources/input_test_4");
        let input_test_5 = include_bytes!("../resources/input_test_5");

        assert_eq!(dist_to_furthest_room(input_test_1), 3);
        assert_eq!(dist_to_furthest_room(input_test_2), 10);
        assert_eq!(dist_to_furthest_room(input_test_3), 18);
        assert_eq!(dist_to_furthest_room(input_test_4), 23);
        assert_eq!(dist_to_furthest_room(input_test_5), 31);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
