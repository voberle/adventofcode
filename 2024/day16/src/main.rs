use fxhash::{FxHashMap, FxHashSet};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
    io::{self, Read},
};

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Free,
    Start,
    End,
}
use Tile::{End, Free, Start, Wall};

impl Tile {
    fn build(c: char) -> Self {
        match c {
            '#' => Wall,
            '.' => Free,
            'S' => Start,
            'E' => End,
            _ => panic!("Invalid tile char"),
        }
    }

    fn is_wall(self) -> bool {
        matches!(self, Wall)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

impl Direction {
    fn is_rotated(self, other: Direction) -> bool {
        match self {
            North | South => [East, West].contains(&other),
            East | West => [North, South].contains(&other),
        }
    }

    // Cost in case of direction change
    fn cost_change(self, other: Direction) -> u32 {
        if self == other {
            0
        } else if self.is_rotated(other) {
            1000
        } else {
            // opposite
            2000
        }
    }
}

struct Grid {
    values: Vec<Tile>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().map(Tile::build).collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos.is_multiple_of(self.cols),
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

    fn find_start(&self) -> usize {
        self.values.iter().position(|t| matches!(t, Start)).unwrap()
    }

    fn find_end(&self) -> usize {
        self.values.iter().position(|t| matches!(t, End)).unwrap()
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    dir: Direction,
    cost: u32,
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

// Dijkstra shortest path, version which finds only the best cost.
fn find_smallest_cost(map: &Grid, start: usize, start_direction: Direction, end: usize) -> u32 {
    let mut visited: FxHashSet<(usize, Direction)> = FxHashSet::default();
    let mut distance: FxHashMap<(usize, Direction), u32> = FxHashMap::default();
    let mut smallest_cost = u32::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        dir: start_direction,
        cost: 0,
    });

    while let Some(Node { pos, dir, cost }) = queue.pop() {
        visited.insert((pos, dir));

        if pos == end {
            smallest_cost = smallest_cost.min(cost);
            continue;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&d| {
            if !map.allowed(pos, d) {
                // Cannot go outside the map.
                return None;
            }
            // We could exclude going backwards, but on the start position it might make sense.

            let next_pos = map.next_pos(pos, d);
            if visited.contains(&(next_pos, d)) {
                return None;
            }

            if map.values[next_pos].is_wall() {
                return None;
            }

            let next_cost = cost + 1 + dir.cost_change(d);

            if let Some(prevcost) = distance.get(&(next_pos, d))
                && *prevcost <= next_cost
            {
                // We have visited this place at cheaper
                return None;
            }

            if next_cost >= smallest_cost {
                return None;
            }

            distance.insert((next_pos, d), next_cost);
            Some(Node {
                pos: next_pos,
                dir: d,
                cost: next_cost,
            })
        }));
    }

    smallest_cost
}

#[allow(dead_code)]
fn lowest_score(map: &Grid) -> u32 {
    let start = map.find_start();
    let end = map.find_end();
    let dir = East;

    find_smallest_cost(map, start, dir, end)
}

// Structure to record the nodes part of the path.
#[derive(PartialEq, Eq, Clone, Debug)]
struct PathNode {
    pos: usize,
    dir: Direction,
}

// Modified Dijkstra shortest path, which finds all the best paths.
fn find_all_best_paths(
    map: &Grid,
    start: usize,
    start_direction: Direction,
    end: usize,
) -> (u32, Vec<Vec<PathNode>>) {
    // No visited set, we use distance to track where we have been.
    let mut distance: FxHashMap<(usize, Direction), u32> = FxHashMap::default();
    let mut predecessors: FxHashMap<(usize, Direction), Vec<(usize, Direction)>> =
        FxHashMap::default();
    let mut smallest_cost = u32::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        dir: start_direction,
        cost: 0,
    });

    // Initialize start distance
    distance.insert((start, start_direction), 0);

    while let Some(Node { pos, dir, cost }) = queue.pop() {
        // Skip if the cost of this node is bigger than what we have found previously.
        if cost > *distance.get(&(pos, dir)).unwrap_or(&u32::MAX) {
            continue;
        }

        if pos == end {
            smallest_cost = smallest_cost.min(cost);
            // Don't continue; we need to explore all paths to the end
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&d| {
            // Exclude going out of the map and walls.
            if !map.allowed(pos, d) || map.values[map.next_pos(pos, d)].is_wall() {
                return None;
            }

            let next_pos = map.next_pos(pos, d);
            let next_cost = cost + 1 + dir.cost_change(d);

            // Ignore any paths with cost too big.
            if next_cost > smallest_cost && pos != end {
                return None;
            }

            // Is the path we are exploring now better than the
            // one we already have here?
            if match distance.get(&(next_pos, d)) {
                Some(&existing_cost) => match next_cost.cmp(&existing_cost) {
                    Ordering::Less => true,
                    Ordering::Equal => false,
                    Ordering::Greater => return None,
                },
                None => true,
            } {
                // Better path.
                distance.insert((next_pos, d), next_cost);
                predecessors.insert((next_pos, d), vec![(pos, dir)]);
            } else {
                // Same path, just update predecessors.
                if let Some(pred_list) = predecessors.get_mut(&(next_pos, d))
                    && !pred_list.contains(&(pos, dir))
                {
                    pred_list.push((pos, dir));
                }
            }

            Some(Node {
                pos: next_pos,
                dir: d,
                cost: next_cost,
            })
        }));
    }

    // Now that we have explored the whole map, create the paths.
    // Iterative Backtracking.
    let mut all_shortest_paths: Vec<Vec<PathNode>> = Vec::new();
    // A stack to keep all the in-progress paths we are building.
    let mut stack: VecDeque<(Vec<PathNode>, (usize, Direction))> = VecDeque::new();

    // Initialize the stack with the end position / direction pair.
    for end_direction in ALL_DIRECTIONS
        .iter()
        .filter(|&&d| distance.contains_key(&(end, d)))
    {
        stack.push_back((
            vec![PathNode {
                pos: end,
                dir: *end_direction,
            }],
            (end, *end_direction),
        ));
    }

    while let Some((current_path, current_node)) = stack.pop_back() {
        if current_node == (start, start_direction) {
            // A complete path is found, adding it to the list.
            let mut complete_path = current_path.clone();
            // Path was build from end, so reverse it.
            complete_path.reverse();
            all_shortest_paths.push(complete_path);
        } else if let Some(prev_nodes) = predecessors.get(&current_node) {
            // We got all the predecessors of the current node.
            for &(prev_pos, prev_dir) in prev_nodes {
                // For each predecessor, we create a new path and add it
                // to the stack, to be checked.
                let mut new_path = current_path.clone();
                new_path.push(PathNode {
                    pos: prev_pos,
                    dir: prev_dir,
                });
                stack.push_back((new_path, (prev_pos, prev_dir)));
            }
        }
    }

    (smallest_cost, all_shortest_paths)
}

fn lowest_score_and_all_tiles(map: &Grid) -> (u32, usize) {
    let start = map.find_start();
    let end = map.find_end();
    let dir = East;

    let (best_score, all_paths) = find_all_best_paths(map, start, dir, end);

    let set: FxHashSet<usize> = all_paths.iter().flatten().map(|p| p.pos).collect();
    (best_score, set.len())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    // println!("Part 1: {}", lowest_score(&map));

    let (lowest_score, tiles_count) = lowest_score_and_all_tiles(&map);
    println!("Part 1: {lowest_score}");
    println!("Part 2: {tiles_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(lowest_score(&Grid::build(INPUT_TEST_1)), 7036);
        assert_eq!(lowest_score(&Grid::build(INPUT_TEST_2)), 11048);
    }

    #[test]
    fn test_part2() {
        let (lowest_score, tiles_count) = lowest_score_and_all_tiles(&Grid::build(INPUT_TEST_1));
        assert_eq!(lowest_score, 7036);
        assert_eq!(tiles_count, 45);

        let (lowest_score, tiles_count) = lowest_score_and_all_tiles(&Grid::build(INPUT_TEST_2));
        assert_eq!(lowest_score, 11048);
        assert_eq!(tiles_count, 64);
    }
}
