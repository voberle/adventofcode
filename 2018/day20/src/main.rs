use std::{
    collections::BinaryHeap,
    fmt,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

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

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

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

    fn next_pos(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        if let Some(allowed_dir) = self.0.get(&pos) {
            if allowed_dir[dir.index()] {
                return Some(pos.next(dir));
            }
        }
        None
    }
}

// Map building directly from the regex.
fn build_map_from_regex(regex: &[u8]) -> Map {
    let mut map = Map::new();
    // Passing 1 as first index to skip first ^.
    explore_map_from_regex(regex, &mut 1, &mut map, Pos::new(0, 0));
    map
}

fn explore_map_from_regex(regex: &[u8], index: &mut usize, map: &mut Map, mut pos: Pos) {
    // The next few lines are the key at exploring the regex.. so simple.
    // Inspired from the idea at
    // https://www.reddit.com/r/adventofcode/comments/a7uk3f/comment/ec6fv6r/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
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
    #[allow(clippy::cast_sign_loss)]
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

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Pos,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra shortest path.
fn find_shortest_path(map: &Map, start: Pos, end: Pos) -> usize {
    let mut visited: FxHashSet<Pos> = FxHashSet::default();
    let mut distance: FxHashMap<Pos, usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    while let Some(Node { pos, cost }) = queue.pop() {
        visited.insert(pos);
        if pos == end {
            shortest_distance = usize::min(shortest_distance, cost);
            continue;
        }
        queue.extend(ALL_DIRECTIONS.iter().filter_map(|d| {
            if let Some(next_pos) = map.next_pos(pos, *d) {
                if visited.contains(&next_pos) {
                    return None;
                }
                let next_cost = cost + 1;
                if let Some(prevcost) = distance.get(&next_pos) {
                    if *prevcost <= next_cost {
                        return None;
                    }
                }
                distance.insert(next_pos, next_cost);
                Some(Node {
                    pos: next_pos,
                    cost: next_cost,
                })
            } else {
                None
            }
        }));
    }
    shortest_distance
}

fn get_all_distances(regex: &[u8]) -> Vec<usize> {
    let map = build_map_from_regex(regex);
    let start = Pos::new(0, 0);
    // Compute all shortest distances
    map.0
        .keys()
        .map(|end| find_shortest_path(&map, start, *end))
        .collect()
}

fn dist_to_furthest_room(all_dist: &[usize]) -> usize {
    *all_dist.iter().max().unwrap()
}

fn rooms_dist_over_1000_doors(all_dist: &[usize]) -> usize {
    all_dist.iter().filter(|&&d| d >= 1000).count()
}

fn main() {
    let mut regex = Vec::new();
    io::stdin().read_to_end(&mut regex).unwrap();

    // Version I inially wrote, but doesn't work for part 2.
    println!("Part 1: {}", graphnode::dist_to_furthest_room(&regex));
    println!("Part 2: {}", graphnode::rooms_dist_over_1000_doors(&regex));

    // Very simple version I re-implemented from a Reddit idea.
    trivial::run_both_parts(&regex);

    // My final version, that doesn't attempt to build a graph from the regex, and that works.
    let all_dist = get_all_distances(&regex);
    println!("Part 1: {}", dist_to_furthest_room(&all_dist));
    println!("Part 2: {}", rooms_dist_over_1000_doors(&all_dist));
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
        assert_eq!(dist_to_furthest_room(&get_all_distances(INPUT_TEST_1)), 3);
        assert_eq!(dist_to_furthest_room(&get_all_distances(INPUT_TEST_2)), 10);
        assert_eq!(dist_to_furthest_room(&get_all_distances(INPUT_TEST_3)), 18);
        assert_eq!(dist_to_furthest_room(&get_all_distances(INPUT_TEST_4)), 23);
        assert_eq!(dist_to_furthest_room(&get_all_distances(INPUT_TEST_5)), 31);
    }
}
