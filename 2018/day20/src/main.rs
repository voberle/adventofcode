use std::{
    collections::BinaryHeap,
    fmt,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

mod parsing;

use parsing::parse_regex;
use parsing::GraphNode;

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

#[allow(dead_code)]
fn build_map(graph: &[GraphNode]) -> Map {
    let mut map: Map = Map::new();

    let pos = Pos::new(0, 0);
    // While building, "false" in the allowed array actually means "maybe",
    // but at the end it means "wall".
    map.0.insert(pos, [false, false, false, false]);

    walk(graph, 0, pos, &mut map);

    map
}

fn walk(graph: &[GraphNode], node_idx: usize, pos: Pos, map: &mut Map) {
    // println!("Walking {}", node_idx);

    let mut pos = pos;
    for dir in &graph[node_idx].value {
        let dir = dir.unwrap();
        // We can go in that direction from current position.
        map.update(pos, dir);

        // From next position, we can go back.
        pos = pos.next(dir);
        map.update(pos, dir.opposite());
    }

    // A clone() is needed for the borrow checker
    let next_nodes = graph[node_idx].next.clone();
    for n in next_nodes {
        walk(graph, n, pos, map);
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.borders();
        let width = (((max_x - min_x) + 1) * 2 + 1) as usize;
        // println!("borders = {:?}, width={}", (min_x, max_x, min_y, max_y), width);

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

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize, // Index in the parsing::Node vector
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
pub fn find_shortest_path(graph: &[GraphNode], start: usize, end: usize) -> usize {
    let mut visited: FxHashSet<usize> = FxHashSet::default();
    let mut distance: FxHashMap<usize, usize> = FxHashMap::default();
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

        queue.extend(graph[pos].next.iter().filter_map(|next_pos| {
            if visited.contains(next_pos) {
                return None;
            }
            let next_cost = cost + graph[pos].value.len();
            if let Some(prevcost) = distance.get(next_pos) {
                if *prevcost <= next_cost {
                    return None;
                }
            }
            distance.insert(*next_pos, next_cost);
            Some(Node {
                pos: *next_pos,
                cost: next_cost,
            })
        }));
    }
    // Need to add the last node len, as it's not been added before.
    shortest_distance + graph[end].value.len()
}

// Largest number of doors required to pass through to reach a room.
fn dist_to_furthest_room(regex: &[u8]) -> usize {
    let graph = parse_regex(regex);
    // parsing::print_graphviz(&nodes);

    // let map = build_map(&graph);
    // println!("{}", regex_to_string(regex));
    // println!("{}", map);

    // Find all the nodes that don't have any next, meaning they are at the end.
    let ending_nodes: Vec<usize> = graph
        .iter()
        .enumerate()
        .filter(|(_, n)| n.next.is_empty())
        .map(|(i, _)| i)
        .collect();

    // Compute the shortest path to each of those ending nodes, and take the max.
    // This produces the right answer, probably because no path overlap each other?
    ending_nodes
        .iter()
        .map(|end| find_shortest_path(&graph, 0, *end))
        .max()
        .unwrap()
}

// Walk through the nodes and mark all rooms less than 1000 doors away
fn walk_and_mark<const LIMIT: usize>(
    graph: &mut Vec<parsing::GraphNode>,
    node: usize,
    steps: usize,
) {
    if steps >= LIMIT - 1 {
        return;
    }

    let mut steps = steps;
    for i in 0..graph[node].value.len() {
        graph[node].value[i] = None;
        steps += 1;
        if steps >= LIMIT - 1 {
            break;
        }
    }

    // A clone() is needed for the borrow checker
    let next_nodes = graph[node].next.clone();
    for n in next_nodes {
        walk_and_mark::<LIMIT>(graph, n, steps);
    }
}

fn rooms_dist_over_1000_doors(regex: &[u8]) -> usize {
    let mut nodes = parsing::parse_regex(regex);

    // Parsing and ignore empty nodes is possibly working for some inputs,
    // but it doesn't for mine.
    // let mut nodes = parsing::parse_regex_with(regex, true);

    walk_and_mark::<1000>(&mut nodes, 0, 0);

    // Unlike part 1, this doesn't work. Don't know why yet.
    nodes
        .iter()
        .map(|n| n.value.iter().filter(|d| d.is_some()).count())
        .sum()
}

// Very simple and working version adapted from a Reddit solution.
// Works on specific input only.
#[allow(dead_code)]
fn trivial_version(regex: &[u8]) {
    let mut grid: FxHashMap<(i32, i32), usize> = FxHashMap::default();
    let (mut dist, mut x, mut y) = (0, 0, 0);
    let mut stack: Vec<(usize, i32, i32)> = Vec::new();

    for c in &regex[1..regex.len() - 1] {
        match c {
            b'(' => stack.push((dist, x, y)),
            b')' => (dist, x, y) = stack.pop().unwrap(),
            b'|' => (dist, x, y) = *stack.last().unwrap(),
            _ => {
                x += i32::from(*c == b'E') - i32::from(*c == b'W');
                y += i32::from(*c == b'S') - i32::from(*c == b'N');
                dist += 1;
                grid.entry((x, y))
                    .and_modify(|e| {
                        if dist < *e {
                            *e = dist;
                        }
                    })
                    .or_insert(dist);
            }
        }
    }
    // println!("Part 1: {}", grid.values().max().unwrap());
    println!("Part 2: {}", grid.values().filter(|v| **v >= 1000).count());
}

fn main() {
    let mut regex = Vec::new();
    io::stdin().read_to_end(&mut regex).unwrap();

    
    println!("Part 1: {}", dist_to_furthest_room(&regex));
    // println!("Part 2: {}", rooms_dist_over_1000_doors(&regex));
    trivial_version(&regex);
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
    fn test_map_generation() {
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_1)).to_string().trim(),
            include_str!("../resources/input_test_1.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_2)).to_string().trim(),
            include_str!("../resources/input_test_2.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_3)).to_string().trim(),
            include_str!("../resources/input_test_3.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_4)).to_string().trim(),
            include_str!("../resources/input_test_4.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_5)).to_string().trim(),
            include_str!("../resources/input_test_5.map")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(dist_to_furthest_room(INPUT_TEST_1), 3);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_2), 10);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_3), 18);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_4), 23);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_5), 31);
    }
}
