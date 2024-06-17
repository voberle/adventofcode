use std::{
    collections::BinaryHeap,
    io::{self, Read},
    usize,
};

use fxhash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

// We don't need to track all blizzards on each minute. We can calculate for each blizzard where
// they will be at each minute, and we are only interested in the state of 4 positions.
// Also only a sub-set of blizzards can reach a specific position, those on same row or column.
#[derive(Debug)]
struct Valley {
    // Each entry represents a line, and lines contains the column where the blizzard is.
    // Important: The blizzard positions start at the map position 1,1, meaning a blizzard on the top-left
    // will be at 0,0 in the vectors below, not 1,1.
    up_blizzards: Vec<Vec<isize>>,
    down_blizzards: Vec<Vec<isize>>,
    left_blizzards: Vec<Vec<isize>>,
    right_blizzards: Vec<Vec<isize>>,
    rows: usize,
    cols: usize,
    vertical_len: isize,
    horizontal_len: isize,
    entrance: Pos,
    exit: Pos,
}

macro_rules! ensure_len {
    ($blizzards:expr, $len:expr) => {
        while $blizzards.len() < $len {
            $blizzards.push(vec![]);
        }
    };
}

impl From<&str> for Valley {
    #[allow(clippy::cast_possible_wrap)]
    fn from(input: &str) -> Self {
        let mut up_blizzards: Vec<Vec<isize>> = vec![vec![]];
        let mut down_blizzards: Vec<Vec<isize>> = vec![vec![]];
        let mut left_blizzards: Vec<Vec<isize>> = vec![vec![]];
        let mut right_blizzards: Vec<Vec<isize>> = vec![vec![]];
        let mut cols = 0;
        let mut rows = 0;
        let mut entrance = Pos::new(0, 0);
        let mut exit = Pos::new(0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, e) in line.chars().enumerate() {
                if y == 0 {
                    if e == '.' {
                        entrance.x = x;
                        entrance.y = 0;
                    } else {
                        assert_eq!(e, '#');
                    }
                    cols += 1;
                } else {
                    match e {
                        '.' => {
                            // Last dot found will be the exit.
                            exit.x = x;
                            exit.y = y;
                        }
                        '^' => {
                            ensure_len!(up_blizzards, x);
                            up_blizzards[x - 1].push(y as isize - 1);
                        }
                        'v' => {
                            ensure_len!(down_blizzards, x);
                            down_blizzards[x - 1].push(y as isize - 1);
                        }
                        '<' => {
                            ensure_len!(left_blizzards, y);
                            left_blizzards[y - 1].push(x as isize - 1);
                        }
                        '>' => {
                            ensure_len!(right_blizzards, y);
                            right_blizzards[y - 1].push(x as isize - 1);
                        }
                        '#' => {}
                        _ => panic!("Invalid map item"),
                    }
                }
            }
            rows += 1;
        }
        let vertical_len = rows - 2;
        let horizontal_len = cols - 2;

        // Make sure blizzard vectors are big enough, to avoid having to do boundary checks after.
        ensure_len!(up_blizzards, horizontal_len);
        ensure_len!(down_blizzards, horizontal_len);
        ensure_len!(left_blizzards, vertical_len);
        ensure_len!(right_blizzards, vertical_len);

        Self {
            up_blizzards,
            down_blizzards,
            left_blizzards,
            right_blizzards,
            rows,
            cols,
            vertical_len: vertical_len as isize,
            horizontal_len: horizontal_len as isize,
            entrance,
            exit,
        }
    }
}

impl Valley {
    // Checks if the position is where blizzards can be.
    fn is_in_blizzard_zone(&self, p: Pos) -> bool {
        p.x > 0 && p.x < self.cols - 1 && p.y > 0 && p.y < self.rows - 1
    }

    fn next_positions_iter(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [
            Pos::new(pos.x, pos.y), // Current position aka not moving is also possible.
            Pos::new(pos.x - 1, pos.y),
            Pos::new(pos.x + 1, pos.y),
            Pos::new(
                pos.x,
                if pos.y == 0 {
                    // Ugly hack to handle the case when we are on entrance still.
                    pos.y
                } else {
                    pos.y - 1
                },
            ),
            Pos::new(pos.x, pos.y + 1),
        ]
        .into_iter()
        .filter(|&p| p == self.entrance || p == self.exit || self.is_in_blizzard_zone(p))
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn is_blizzard_at(&self, pos: Pos, minute: usize) -> bool {
        if !self.is_in_blizzard_zone(pos) {
            return false;
        }

        // Blizzard positions start at 0,0.
        let x = pos.x - 1;
        let y = pos.y - 1;
        let minute = minute as isize;

        for initial_x in &self.left_blizzards[y] {
            let current_x = (initial_x - minute).rem_euclid(self.horizontal_len);
            if x == current_x as usize {
                return true;
            }
        }
        for initial_x in &self.right_blizzards[y] {
            let current_x = (initial_x + minute).rem_euclid(self.horizontal_len);
            if x == current_x as usize {
                return true;
            }
        }
        for initial_y in &self.up_blizzards[x] {
            let current_y = (initial_y - minute).rem_euclid(self.vertical_len);
            if y == current_y as usize {
                return true;
            }
        }
        for initial_y in &self.down_blizzards[x] {
            let current_y = (initial_y + minute).rem_euclid(self.vertical_len);
            if y == current_y as usize {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Pos,
    minute: usize,
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
fn find_shortest_path(valley: &Valley, start: Pos, end: Pos) -> usize {
    let mut visited: FxHashSet<(Pos, usize)> = FxHashSet::default();
    let mut distance: FxHashMap<(Pos, usize), usize> = FxHashMap::default();

    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        minute: 0,
        cost: 0,
    });

    while let Some(Node { pos, minute, cost }) = queue.pop() {
        visited.insert((pos, minute));

        if pos == end {
            shortest_distance = shortest_distance.min(cost + 1);
            continue;
        }

        queue.extend(valley.next_positions_iter(pos).filter_map(|next_pos| {
            // Check if next pos is valid.
            if valley.is_blizzard_at(pos, minute + 1) {
                return None;
            }

            if visited.contains(&(next_pos, minute + 1)) {
                return None;
            }

            let next_cost = cost + 1;

            if let Some(prevcost) = distance.get(&(next_pos, minute + 1)) {
                if *prevcost <= next_cost {
                    return None;
                }
            }

            // Avoid going too far.
            if next_cost >= shortest_distance {
                return None;
            }

            distance.insert((next_pos, minute + 1), next_cost);
            Some(Node {
                pos: next_pos,
                minute: minute + 1,
                cost: next_cost,
            })
        }));
    }
    shortest_distance
}

fn time_to_reach_goal(valley: &Valley) -> usize {
    find_shortest_path(valley, valley.entrance, valley.exit)
}

fn part2(valley: &Valley) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let valley = input.as_str().into();
    // println!("{:?}", valley);

    println!("Part 1: {}", time_to_reach_goal(&valley));
    println!("Part 2: {}", part2(&valley));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_0: &str = include_str!("../resources/input_test_0");
    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_blizzard_at() {
        let valley: Valley = INPUT_TEST_0.into();
        let mut minute = 0;
        assert!(valley.is_blizzard_at(Pos::new(1, 2), minute));
        assert!(valley.is_blizzard_at(Pos::new(4, 4), minute));
        minute += 1;
        assert!(valley.is_blizzard_at(Pos::new(2, 2), minute));
        assert!(valley.is_blizzard_at(Pos::new(4, 5), minute));
    }

    #[test]
    fn test_part1() {
        assert_eq!(time_to_reach_goal(&INPUT_TEST_1.into()), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT_TEST_1.into()), 0);
    }
}
