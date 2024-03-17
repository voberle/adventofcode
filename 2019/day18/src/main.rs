use fxhash::{FxHashMap, FxHashSet};
use std::{
    collections::BinaryHeap,
    fmt,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Entrance,
    Open,
    Wall,
    Key(char),
    Door(char),
}
use Element::{Door, Entrance, Key, Open, Wall};

impl Element {
    fn new(c: char) -> Self {
        match c {
            '@' => Entrance,
            '.' => Open,
            '#' => Wall,
            'a'..='z' => Key(c),
            'A'..='Z' => Door(c.to_ascii_lowercase()),
            _ => panic!("Unrecognized char {}", c),
        }
    }

    fn is_door_for_key(self, key: Self) -> bool {
        if let Door(door_val) = self {
            if let Key(key_val) = key {
                return key_val == door_val;
            }
        }
        false
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Entrance => '@',
                Open => '.',
                Wall => '#',
                Key(c) => c,
                Door(c) => c.to_ascii_uppercase(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    values: Vec<Element>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().map(Element::new).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print(&self) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                print!("{}", c);
            }
            println!();
        }
    }

    fn is_dir_on_map(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    // Are we allowed to walk on that position?
    fn is_pos_allowed(&self, pos: usize) -> bool {
        let elt = self.values[pos];
        match elt {
            Entrance | Open | Key(_) => true,
            Wall | Door(_) => false,
        }
    }

    fn get_entrance_pos(&self) -> usize {
        self.values
            .iter()
            .position(|&e| matches!(e, Entrance))
            .unwrap()
    }

    fn count_keys(&self) -> usize {
        self.values.iter().filter(|&&e| matches!(e, Key(_))).count()
    }

    fn get_keys_positions(&self) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(pos, e)| if matches!(e, Key(_)) { Some(pos) } else { None })
            .collect()
    }

    // Remove the key and corresponding door from the map.
    // Finds the door corresponding to the key and replaces it with an open space.
    fn collect_key_and_open_door(&mut self, key_pos: usize) {
        let key = self.values[key_pos];
        assert!(matches!(key, Key(_)));

        self.values[key_pos] = Open;

        // println!("For key {} we have door {}", key, door);
        if let Some(door_pos) = self.values.iter().position(|&e| e.is_door_for_key(key)) {
            // Some keys don't have a corresponding door.
            self.values[door_pos] = Open;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
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
fn find_shortest_path(map: &Map, start: usize, end: usize) -> Option<usize> {
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

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|d| {
            if !map.is_dir_on_map(pos, *d) {
                return None;
            }
            let next_pos = map.next_pos(pos, *d);
            if !map.is_pos_allowed(pos) {
                return None;
            }

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
        }));
    }

    if shortest_distance == usize::MAX {
        None
    } else {
        Some(shortest_distance)
    }
}

// Recursive function.
fn find_keys(
    map: &Map,
    from: usize,
    keys_to_find_count: usize,
    distance_so_far: usize,
    shortest_distance: &mut usize,
) {
    let all_keys_pos = map.get_keys_positions();
    for key_pos in all_keys_pos {
        if let Some(dist_to_key) = find_shortest_path(map, from, key_pos) {
            // This key is reachable
            let key = map.values[key_pos];
            assert!(matches!(key, Key(_)));
            let new_dist = distance_so_far + dist_to_key;

            if new_dist >= *shortest_distance {
                // We have better already
                continue;
            }

            // println!("Key reachable: {}, dist so far {}", key, new_dist);

            if keys_to_find_count == 1 {
                // Last key we needed to find
                *shortest_distance = new_dist.min(*shortest_distance);
                continue;
            }

            let mut map_copy = map.clone();
            map_copy.collect_key_and_open_door(key_pos);

            // map_copy.print();

            find_keys(
                &map_copy,
                key_pos,
                keys_to_find_count - 1,
                new_dist,
                shortest_distance,
            );
        }
    }
}

fn shortest_path_all_keys(map: &Map) -> usize {
    // Find which keys are reachable and their distance.
    // Recursively:
    // - Unlock each.
    // - Start over: Find keys etc
    // Stop when we have key count.

    let mut shortest_distance = usize::MAX;

    let keys_to_find_count = map.count_keys();
    let entrance_pos = map.get_entrance_pos();

    println!("Need to find {} keys", keys_to_find_count);

    find_keys(
        map,
        entrance_pos,
        keys_to_find_count,
        0,
        &mut shortest_distance,
    );

    shortest_distance
}

fn part2(map: &Map) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Map::build(&input);
    map.print();

    println!("Part 1: {}", shortest_path_all_keys(&map));
    println!("Part 2: {}", part2(&map));
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
    fn test_part1() {
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_1)), 8);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_2)), 86);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_3)), 132);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_4)), 136);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_5)), 81);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
