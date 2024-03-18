use fxhash::{FxHashMap, FxHashSet};
use std::{collections::BinaryHeap, fmt};

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

    // Does does specified key open this door?
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
pub struct Map {
    values: Vec<Element>,
    rows: usize,
    cols: usize,
}

impl Map {
    pub fn build(input: &str) -> Self {
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

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    // Does going into this direction keep us on the map?
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
            // Door is allowed, shortest path method filters them separately.
            Entrance | Open | Key(_) | Door(_) => true,
            Wall => false,
        }
    }

    fn get_positions_of(&self, match_fn: fn(Element) -> bool) -> Vec<usize> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(pos, e)| if match_fn(*e) { Some(pos) } else { None })
            .collect()
    }

    pub fn get_entrance_positions(&self) -> Vec<usize> {
        self.get_positions_of(|e| matches!(e, Entrance))
    }

    pub fn get_keys_positions(&self) -> Vec<usize> {
        self.get_positions_of(|e| matches!(e, Key(_)))
    }

    pub fn get_doors_positions(&self) -> Vec<usize> {
        self.get_positions_of(|e| matches!(e, Door(_)))
    }

    pub fn get_door_position_for_key(&self, key_pos: usize) -> Option<usize> {
        let key = self.values[key_pos];
        assert!(matches!(key, Key(_)));

        // Some keys don't have a corresponding door.
        self.values.iter().position(|&e| e.is_door_for_key(key))
    }

    // Updates the map for part 2.
    pub fn update_map(&self) -> Map {
        let mut map = self.clone();
        let entrance_positions = map.get_entrance_positions();
        assert_eq!(entrance_positions.len(), 1);
        let entrance_pos = entrance_positions[0];
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

    // Quadrants are numbered 0 to 3
    pub fn is_in_quadrant(&self, pos: usize, entrance_pos: usize, quadrant: usize) -> bool {
        let row = self.row(pos);
        let col = self.col(pos);
        let e_row = self.row(entrance_pos);
        let e_col = self.col(entrance_pos);
        match quadrant {
            0 => row <= e_row && col <= e_col,
            1 => row <= e_row && col >= e_col,
            2 => row >= e_row && col <= e_col,
            3 => row >= e_row && col >= e_col,
            _ => panic!("Invalid quadrant {}", quadrant)
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
pub fn find_shortest_path(
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
            // Don't walk into closed doors.
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
