use std::{
    collections::BinaryHeap,
    fmt,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

mod parsing;

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
pub fn find_shortest_path(graph: &[parsing::GraphNode], start: usize, end: usize) -> usize {
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
    let nodes = parsing::parse_regex(regex);
    // parsing::print_graphviz(&nodes);

    // Find all the nodes that don't have any next, meaning they are at the end.
    let ending_nodes: Vec<usize> = nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.next.is_empty())
        .map(|(i, _)| i)
        .collect();

    // Compute the shortest path to each of those ending nodes, and take the max.
    // This produces the right answer, probably because no path overlap each other?
    ending_nodes
        .iter()
        .map(|end| find_shortest_path(&nodes, 0, *end))
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
    // println!("nodes count {}", nodes.len());

    // let total_rooms_count: usize = nodes.iter().map(|n| n.value.len()).sum();
    // +1 as we need to add first room
    // println!("total_rooms_count count {}", total_rooms_count + 1);
    // number of doors is number of rooms - 1

    walk_and_mark::<1000>(&mut nodes, 0, 0);
    // parsing::print_graphviz(&nodes);

    // Unlike part 1, this doesn't work. Don't know why yet.
    nodes
        .iter()
        .map(|n| n.value.iter().filter(|d| d.is_some()).count())
        .sum()
}

fn main() {
    let mut regex = Vec::new();
    io::stdin().read_to_end(&mut regex).unwrap();

    println!("Part 1: {}", dist_to_furthest_room(&regex));
    println!("Part 2: {}", rooms_dist_over_1000_doors(&regex));
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
}
