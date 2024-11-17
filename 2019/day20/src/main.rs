use fxhash::{FxHashMap, FxHashSet};
use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

mod portals;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, PartialEq)]
enum Element {
    Wall,
    OpenPassage,
    Entry, // AA
    Exit,  // BB
    InnerPortal(String),
    OuterPortal(String),
    Space,
}
use Element::{Entry, Exit, InnerPortal, OpenPassage, OuterPortal, Space, Wall};

impl Element {
    fn is_portal_for(&self, name: &str) -> bool {
        match self {
            InnerPortal(n) | OuterPortal(n) => name == n,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Maze {
    values: Vec<Element>,
    rows: usize,
    cols: usize,
}

impl Maze {
    fn build(input: &str) -> Self {
        // First we get all the portal names and positions.
        let portals = portals::get_portals_from_input(input);
        // println!("{:#?}", portals);

        // Then create the map.
        let mut rows = 0;
        let mut values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    .map(|c| match c {
                        '.' => OpenPassage,
                        '#' => Wall,
                        _ => Space, // Labels or empty space are treated like walls
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;

        // Finally inject the portals
        values.iter_mut().enumerate().for_each(|(p, e)| {
            if let Some(name) = portals.get(&p) {
                assert_eq!(*e, OpenPassage);
                if name == "AA" {
                    *e = Entry;
                } else if name == "ZZ" {
                    *e = Exit;
                } else {
                    // We identify outer portals by the fact that their position is close to the border.
                    let col = p % cols;
                    let row = p / cols;
                    if col < 3 || col > cols - 4 || row < 3 || row > rows - 4 {
                        *e = OuterPortal(name.clone());
                    } else {
                        *e = InnerPortal(name.clone());
                    }
                }
            }
        });

        Self { values, rows, cols }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = match self.values.get(p) {
                    Some(Wall) => '#',
                    Some(OpenPassage) => '.',
                    Some(Entry) => 'A',
                    Some(Exit) => 'Z',
                    Some(OuterPortal(_)) => 'O',
                    Some(InnerPortal(_)) => 'X',
                    Some(Space) => ' ',
                    None => panic!("Bug in print()"),
                };
                print!("{c}");
            }
            println!();
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

    fn get_entry(&self) -> usize {
        self.values.iter().position(|e| matches!(e, Entry)).unwrap()
    }

    fn get_exit(&self) -> usize {
        self.values.iter().position(|e| matches!(e, Exit)).unwrap()
    }

    fn get_other_portal_pos(&self, portal_name: &str, current_pos: usize) -> Option<usize> {
        if let Some((pos, _)) = self
            .values
            .iter()
            .enumerate()
            .find(|(i, e)| *i != current_pos && e.is_portal_for(portal_name))
        {
            Some(pos)
        } else {
            None
        }
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    level: usize,
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
fn find_shortest_path<const RECURSIVE: bool>(maze: &Maze) -> usize {
    let start = maze.get_entry();
    let end = maze.get_exit();

    let mut visited: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut distance: FxHashMap<(usize, usize), usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        level: 0,
        cost: 0,
    });

    while let Some(Node { pos, level, cost }) = queue.pop() {
        visited.insert((pos, level));

        if pos == end && level == 0 {
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|d| {
            // We don't need to check if next_pos is on grid, as it's take care of by the None arm of following match.
            let mut next_pos = maze.next_pos(pos, *d);
            let mut next_level = level;
            let mut next_cost = cost + 1;

            match maze.values.get(next_pos) {
                Some(InnerPortal(name)) => {
                    next_pos = maze.get_other_portal_pos(name, next_pos).unwrap();
                    next_cost += 1; // Teleporting costs a step.
                    if RECURSIVE {
                        next_level += 1;
                    }
                }
                Some(OuterPortal(name)) => {
                    if RECURSIVE && level == 0 {
                        // At top level, outer portals don't work.
                        return None;
                    }
                    next_pos = maze.get_other_portal_pos(name, next_pos).unwrap();
                    next_cost += 1;
                    if RECURSIVE {
                        next_level -= 1;
                    }
                }
                Some(OpenPassage) => {}
                Some(Entry | Exit) => {
                    // Entry and exits are walls at lower levels.
                    if level > 0 {
                        return None;
                    }
                }
                Some(Wall | Space) | None => return None,
            }

            if visited.contains(&(next_pos, next_level)) {
                return None;
            }

            // Avoid going too far
            if next_cost >= shortest_distance {
                return None;
            }

            if let Some(prevcost) = distance.get(&(next_pos, next_level)) {
                if *prevcost <= next_cost {
                    return None;
                }
            }
            distance.insert((next_pos, next_level), next_cost);
            Some(Node {
                pos: next_pos,
                level: next_level,
                cost: next_cost,
            })
        }));
    }
    shortest_distance
}

fn path_length(maze: &Maze) -> usize {
    find_shortest_path::<false>(maze)
}

fn path_length_recursive(maze: &Maze) -> usize {
    find_shortest_path::<true>(maze)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let maze = Maze::build(&input);
    // maze.print();

    println!("Part 1: {}", path_length(&maze));
    println!("Part 2: {}", path_length_recursive(&maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(path_length(&Maze::build(INPUT_TEST_1)), 23);
        assert_eq!(path_length(&Maze::build(INPUT_TEST_2)), 58);
    }

    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part2() {
        assert_eq!(path_length_recursive(&Maze::build(INPUT_TEST_3)), 396);
    }
}
