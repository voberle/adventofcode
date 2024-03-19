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
    Portal(String),
    Space,
}
use Element::{OpenPassage, Portal, Space, Wall};

impl Element {}

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
            if let Some(portal) = portals.get(&p) {
                assert_eq!(*e, OpenPassage);
                *e = Portal(portal.clone());
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
                    Some(Portal(_)) => 'O',
                    Some(Space) => ' ',
                    None => panic!("Bug in print()"),
                };
                print!("{}", c);
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

    // Use only for AA or ZZ, when there is only one portal.
    fn get_portal_pos(&self, portal: &str) -> usize {
        let elt = Portal(portal.to_string());
        self.values.iter().position(|e| *e == elt).unwrap()
    }

    fn get_other_portal_pos(&self, portal: &str, current_pos: usize) -> Option<usize> {
        let elt = Portal(portal.to_string());
        if let Some((pos, _)) = self
            .values
            .iter()
            .enumerate()
            .find(|(i, e)| *i != current_pos && **e == elt)
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
fn find_shortest_path(maze: &Maze, start: usize, end: usize) -> usize {
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
            shortest_distance = shortest_distance.min(cost);
            continue;
        }

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|d| {
            // We don't need to check if next_pos is on grid, as it's take care of by the None arm of following match.
            let mut next_pos = maze.next_pos(pos, *d);
            let mut next_cost = cost + 1;

            match maze.values.get(next_pos) {
                Some(Portal(name)) => {
                    if let Some(other_portal_pos) = maze.get_other_portal_pos(name, next_pos) {
                        // Teleporting!
                        next_pos = other_portal_pos;
                        // Teleporting costs a step.
                        next_cost += 1;
                    }
                }
                Some(OpenPassage) => {}
                Some(Wall | Space) | None => return None,
            }

            if visited.contains(&next_pos) {
                return None;
            }
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
    shortest_distance
}

fn aa_to_zz_path_length(maze: &Maze) -> usize {
    let start = maze.get_portal_pos("AA");
    let end = maze.get_portal_pos("ZZ");
    find_shortest_path(maze, start, end)
}

fn part2(maze: &Maze) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let maze = Maze::build(&input);
    // maze.print();

    println!("Part 1: {}", aa_to_zz_path_length(&maze));
    println!("Part 2: {}", part2(&maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(aa_to_zz_path_length(&Maze::build(INPUT_TEST_1)), 23);
        assert_eq!(aa_to_zz_path_length(&Maze::build(INPUT_TEST_2)), 58);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
