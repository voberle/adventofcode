use fxhash::{FxHashMap, FxHashSet};
use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{North, East, South, West};

impl Direction {
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

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<u32>,
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print_with_pos(&self, positions: &[usize]) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("\x1b[91m{}\x1b[0m", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }

    fn print(&self) {
        self.print_with_pos(&[]);
    }

    const fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    // Check we don't go outside grid.
    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `can_go`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    fn try_next_pos(&self, pos: usize, direction: Direction) -> Option<usize> {
        if self.allowed(pos, direction) {
            Some(self.next_pos(pos, direction))
        } else {
            None
        }
    }
}

#[test]
fn test_grid() {
    let input = "123\n456";
    let grid = Grid::build(&input);
    assert_eq!(grid.cols, 3);
    assert_eq!(grid.rows, 2);
    assert_eq!(grid.pos(0, 1), 1);
    assert_eq!(grid.pos(1, 2), 5);
    assert_eq!(grid.row(5), 1);
    assert_eq!(grid.col(5), 2);
    assert_eq!(grid.row(1), 0);
    assert_eq!(grid.col(1), 1);

    assert!(grid.allowed(5, North));
    assert_eq!(grid.next_pos(5, North), 2);
    assert!(grid.allowed(5, West));
    assert_eq!(grid.next_pos(5, West), 4);
    assert!(!grid.allowed(5, East));
    assert!(!grid.allowed(5, South));
}

// Node we are exploring with Dijkstra.
// It's an "extended coordinates" model: In addition to the position `pos` on the grid
// and the cost (aka heat loss) we had so far, we also have:
// 1) From where we can to this position (`direction`);
// 2) How many steps we did in a straight line in this direction (`line_len`).
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    // Direction we came from on this position. Will only be Node for the start.
    direction: Option<Direction>,
    // How many steps we did in a straight line in this direction.
    line_len: usize,
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

// Key for the sets/maps: Position, Direction, Line ength
type HashKey = (usize, Option<Direction>, usize);

// Dijkstra shortest path
fn find_shortest_path<const ULTRA_CRUCIBLE: bool>(grid: &Grid, start: usize, end: usize) -> u32 {
    let mut visited: FxHashSet<HashKey> = FxHashSet::default();
    let mut distance: FxHashMap<HashKey, u32> = FxHashMap::default();
    let mut previous: FxHashMap<HashKey, HashKey> = FxHashMap::default();
    let mut shortest_distance = u32::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        direction: None,
        line_len: 0,
        cost: 0,
    });

    while let Some(Node {
        pos,
        direction,
        line_len,
        cost,
    }) = queue.pop()
    {
        // Mark node as visited
        match direction {
            Some(d) => {
                visited.insert((pos, Some(d), line_len));
            }
            None => {
                for d in ALL_DIRECTIONS {
                    visited.insert((pos, Some(d), line_len));
                }
            }
        };

        if pos == end {
            shortest_distance = u32::min(shortest_distance, cost);
            continue;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&d| {
            let (is_same_direction, is_opposite_direction) = match direction {
                Some(dir) => (dir == d, dir == d.opposite()),
                None => (true, false), // For starting position
            };

            // Not allowed: Going outside grid or going back
            if !grid.allowed(pos, d) || is_opposite_direction {
                return None;
            }

            #[allow(clippy::if_not_else)]
            if !ULTRA_CRUCIBLE {
                // Not allowed: Going too long straight
                if is_same_direction && line_len > 2 {
                    return None;
                }
            } else {
                // Must move a min of 4 blocks straight, before turning, or even before it can stop at the end.
                if !is_same_direction && line_len < 4 {
                    return None;
                }
                // And max of 10 before having to turn.
                if is_same_direction && line_len > 9 {
                    return None;
                }
            }

            let next_pos = grid.next_pos(pos, d);
            let next_line_len = if is_same_direction { line_len + 1 } else { 1 };
            if ULTRA_CRUCIBLE {
                // Cannot stop at the end if has not moved a min of 4 blocks straight.
                if next_pos == end && next_line_len < 4 {
                    return None;
                }
            }

            let next_key = (next_pos, Some(d), next_line_len);
            if visited.contains(&next_key) {
                return None;
            }

            let next_cost = cost + grid.values[next_pos];
            if let Some(prevcost) = distance.get(&next_key) {
                if *prevcost <= next_cost {
                    return None;
                }
            }

            distance.insert(next_key, next_cost);
            previous.insert(next_key, (pos, direction, line_len));

            Some(Node {
                pos: next_pos,
                direction: Some(d),
                line_len: next_line_len,
                cost: next_cost,
            })
        }));
    }

    let end_key = distance
        .iter()
        .filter(|(k, _)| k.0 == end)
        .min_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap();

    // grid.print_with_pos(&path_back(&previous, end_key, start));

    assert_eq!(shortest_distance, *distance.get(end_key).unwrap());
    shortest_distance
}

#[allow(dead_code)]
fn path_back(previous: &FxHashMap<HashKey, HashKey>, from: &HashKey, to: usize) -> Vec<usize> {
    let mut path_back: Vec<usize> = Vec::new();
    let mut p = *from;
    while p != (to, None, 0) {
        // println!(" {:?}", p);
        path_back.push(p.0);
        if let Some(val) = previous.get(&p) {
            p = *val;
        } else {
            break;
        }
    }
    path_back
}

fn minimal_heat_loss<const ULTRA_CRUCIBLE: bool>(grid: &Grid) -> u32 {
    let start = 0;
    let end = grid.pos(grid.rows - 1, grid.cols - 1);
    // println!("Start: {}; End: {}", start, end);
    // grid.print_with_pos(&[start, end]);

    find_shortest_path::<ULTRA_CRUCIBLE>(grid, start, end)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", minimal_heat_loss::<false>(&grid));
    println!("Part 2: {}", minimal_heat_loss::<true>(&grid));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        let grid = Grid::build(INPUT_TEST_1);
        assert_eq!(minimal_heat_loss::<false>(&grid), 102);
    }

    #[test]
    fn test_part2() {
        let grid1 = Grid::build(INPUT_TEST_1);
        assert_eq!(minimal_heat_loss::<true>(&grid1), 94);

        let grid2 = Grid::build(INPUT_TEST_2);
        assert_eq!(minimal_heat_loss::<true>(&grid2), 71);
    }
}
