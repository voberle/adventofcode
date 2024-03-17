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

    #[allow(dead_code)]
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
            // Door is allowed, shortest path method filters them separately
            Entrance | Open | Key(_) | Door(_) => true,
            Wall => false,
        }
    }

    fn get_entrance_pos(&self) -> usize {
        self.values
            .iter()
            .position(|&e| matches!(e, Entrance))
            .unwrap()
    }

    fn get_keys_positions(&self) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(pos, e)| if matches!(e, Key(_)) { Some(pos) } else { None })
            .collect()
    }

    fn get_doors_positions(&self) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(pos, e)| {
                if matches!(e, Door(_)) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect()
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
fn find_shortest_path(
    map: &Map,
    doors_positions: &[usize],
    start: usize,
    end: usize,
) -> Option<usize> {
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
            if doors_positions.contains(&pos) {
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
    keys_positions: &[usize],
    doors_positions: &[usize],
    distance_so_far: usize,
    shortest_distance: &mut usize,
    cache: &mut FxHashSet<(usize, usize, Vec<usize>)>,
    path_cache: &mut FxHashMap<(usize, usize, Vec<usize>), Option<usize>>,
) {
    // keys_positions was sorted when created
    if !cache.insert((distance_so_far, from, keys_positions.to_vec())) {
        return;
    }

    let mut reachable_keys: Vec<(usize, usize)> = keys_positions
        .iter()
        .filter_map(|&pos| {
            if let Some(opt_d) = path_cache.get(&(from, pos, doors_positions.to_vec())) {
                opt_d.map(|d| (pos, d))
            } else if let Some(dist_to_key) = find_shortest_path(map, doors_positions, from, pos) {
                path_cache.insert((from, pos, doors_positions.to_vec()), Some(dist_to_key));
                Some((pos, dist_to_key))
            } else {
                path_cache.insert((from, pos, doors_positions.to_vec()), None);
                None
            }
        })
        .collect();

    // Sort by distance, to explore the closest ones first.
    reachable_keys.sort_unstable_by_key(|e| e.1);

    for (key_pos, dist_to_key) in reachable_keys {
        let key = map.values[key_pos];
        assert!(matches!(key, Key(_)));
        let new_dist = distance_so_far + dist_to_key;

        if new_dist >= *shortest_distance {
            // We have better already
            continue;
        }

        if keys_positions.len() == 1 {
            // Last key we needed to find
            *shortest_distance = new_dist.min(*shortest_distance);
            println!(
                "Last key, dist to it {}, dist {}, shortest {}",
                dist_to_key, new_dist, shortest_distance
            );
            continue;
        }

        let new_doors_positions: Vec<usize> =
            if let Some(door_pos) = map.values.iter().position(|&e| e.is_door_for_key(key)) {
                // Some keys don't have a corresponding door.
                doors_positions
                    .iter()
                    .filter(|&&p| p != door_pos)
                    .copied()
                    .collect()
            } else {
                // we could maybe save this clone
                doors_positions.to_vec()
            };

        let mut new_keys_positions: Vec<usize> = keys_positions
            .iter()
            .filter(|&&p| p != key_pos)
            .copied()
            .collect();
        new_keys_positions.sort_unstable(); // for adding to cache later

        find_keys(
            map,
            key_pos,
            &new_keys_positions,
            &new_doors_positions,
            new_dist,
            shortest_distance,
            cache,
            path_cache,
        );
    }
}

fn shortest_path_all_keys(map: &Map) -> usize {
    // Find which keys are reachable and their distance.
    // Recursively:
    // - Unlock each.
    // - Start over: Find keys etc
    // Stop when we have key count.

    let mut shortest_distance = usize::MAX;

    let entrance_pos = map.get_entrance_pos();
    let doors_positions = map.get_doors_positions();
    let keys_positions = map.get_keys_positions();

    // Cache of "distance so far" + "from where we are searching" + "positions of remaining keys to find".
    let mut cache: FxHashSet<(usize, usize, Vec<usize>)> = FxHashSet::default();
    // Cache for Dijkstra shortest path function.
    let mut path_cache: FxHashMap<(usize, usize, Vec<usize>), Option<usize>> = FxHashMap::default();

    find_keys(
        map,
        entrance_pos,
        &keys_positions,
        &doors_positions,
        0,
        &mut shortest_distance,
        &mut cache,
        &mut path_cache,
    );

    shortest_distance
}

fn update_map(map: &Map) -> Map {
    let mut map = map.clone();
    let entrance_pos = map.get_entrance_pos();
    // North
    map.values[entrance_pos - map.cols - 1] = Entrance;
    map.values[entrance_pos - map.cols] = Wall;
    map.values[entrance_pos - map.cols + 1] = Entrance;
    // Middle
    map.values[entrance_pos + 1] = Wall;
    map.values[entrance_pos] = Wall;
    map.values[entrance_pos - 1] = Wall;
    // South
    map.values[entrance_pos + map.cols - 1] = Entrance;
    map.values[entrance_pos + map.cols] = Wall;
    map.values[entrance_pos + map.cols + 1] = Entrance;
    map
}

fn shortest_path_4_robots(map: &Map) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Map::build(&input);
    // map.print();

    // println!("Part 1: {}", shortest_path_all_keys(&map));

    let updated_map = update_map(&map);
    // updated_map.print();

    println!("Part 2: {}", shortest_path_4_robots(&updated_map));
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

    const INPUT_TEST_6: &str = include_str!("../resources/input_test_6");
    const INPUT_TEST_7: &str = include_str!("../resources/input_test_7");
    const INPUT_TEST_8: &str = include_str!("../resources/input_test_8");
    const INPUT_TEST_9: &str = include_str!("../resources/input_test_9");

    #[test]
    fn test_part2() {
        assert_eq!(shortest_path_4_robots(&Map::build(INPUT_TEST_6)), 8);
        assert_eq!(shortest_path_4_robots(&Map::build(INPUT_TEST_7)), 24);
        assert_eq!(shortest_path_4_robots(&Map::build(INPUT_TEST_8)), 32);
        assert_eq!(shortest_path_4_robots(&Map::build(INPUT_TEST_9)), 72);
    }
}
