// https://adventofcode.com/2023/day/23

use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self, BufRead},
};

// NB: Direction and Grid code is mostly the same as Day 21.

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    fn index(&self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }

    fn opposite(&self) -> Self {
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
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build<R>(reader: &mut R) -> Self
    where
        R: BufRead,
    {
        let mut rows = 0;
        let values: Vec<_> = reader
            .lines()
            .filter_map(|result| result.ok())
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print(&self) {
        for row in 0..self.rows {
            println!(
                "{}",
                self.values[row * self.cols..(row + 1) * self.cols]
                    .iter()
                    .collect::<String>()
            );
        }
    }

    fn print_with_pos(&self, pos: usize) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if p == pos {
                    print!("\x1b[91m{}\x1b[0m", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
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
    let grid = Grid::build(&mut input.as_bytes());
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

// Walk until we reach an intersection.
// Returns the intersection position and the distance to get there.
// None means we got blocked.
// Exit of the grid is considered an intersection.
fn walk(grid: &Grid, from: usize, towards: Direction) -> Option<(usize, u32)> {
    let mut pos = from;
    let mut opp_dir = towards.opposite();
    let mut steps = 0;
    let mut exit = false;
    loop {
        // Finds the direction we are allowed to walk
        let dirs: Vec<Direction> = ALL_DIRECTIONS
            .iter()
            .filter_map(|&d| {
                if steps == 0 && d != towards {
                    // On first step, we want to go into a specific direction
                    return None;
                }

                if d == opp_dir {
                    // Not allowed to go back
                    None
                } else if let Some(npos) = grid.try_next_pos(pos, d) {
                    // println!("{pos} {:?}, c={}", d, grid.values[npos]);
                    match grid.values[npos] {
                        '#' => None,    // Not allowed to go into forest
                        '.' => Some(d), // Paths are ok
                        '<' => {
                            if d == West {
                                Some(d)
                            } else {
                                None
                            }
                        }
                        '>' => {
                            if d == East {
                                Some(d)
                            } else {
                                None
                            }
                        }
                        // Observation: ^ is never found in the input.
                        '^' => {
                            if d == North {
                                Some(d)
                            } else {
                                None
                            }
                        }
                        'v' => {
                            if d == South {
                                Some(d)
                            } else {
                                None
                            }
                        }
                        _ => panic!("Invalid char in map {}", grid.values[npos]),
                    }
                } else {
                    // Exit detection is ugly, could be improved.
                    if steps > 0 {
                        // We found the exit!
                        // println!("Exit found! {}, steps={}", pos, steps);
                        exit = true;
                        Some(d)
                    } else {
                        None
                    }
                }
            })
            .collect();

        if exit {
            return Some((pos, steps));
        }

        if dirs.is_empty() {
            // dead-end
            return None;
        } else if dirs.len() == 1 {
            // continue walking
            pos = grid.next_pos(pos, dirs[0]);
            // grid.print_with_pos(pos);
            opp_dir = dirs[0].opposite();
            steps += 1;
        } else {
            // intersection
            // println!("Intersection: steps={}, pos={}, dirs={:?}", steps, pos, dirs);
            // grid.print_with_pos(pos);
            return Some((pos, steps));
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: usize,
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

// Dijkstra's algorithm
fn longest_hike_step_count(grid: &Grid) -> u32 {
    let start = 1;
    let end = grid.pos(grid.rows - 1, grid.cols - 2);
    // println!("Start: {}; End: {}", start, end);

    let mut visited: HashSet<usize> = HashSet::new();
    let mut distance: HashMap<usize, u32> = HashMap::new();

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: start,
        cost: 0,
    });

    let mut answer = u32::MIN;

    while let Some(Node { pos, cost }) = queue.pop() {
        if pos == end {
            answer = u32::max(answer, cost);
            // println!("On End {} ({}, {}), cost {}, answer {}", pos, grid.row(pos), grid.col(pos), cost, answer);
            continue;
        }

        visited.insert(pos);
        // println!("Visiting {} ({}, {})", pos, grid.row(pos), grid.col(pos));
        // grid.print_with_pos(pos);

        queue.extend(ALL_DIRECTIONS.iter().filter_map(|&d| {
            // TODO optimization to avoid walking back
            // println!("Walking towards {:?}", d);
            if let Some((npos, steps)) = walk(grid, pos, d) {
                // println!(" reached {} after {} steps", npos, steps);
                let nkey = npos;
                let ncost = cost + steps;
                if visited.contains(&nkey) {
                    return None;
                }
                if let Some(prevcost) = distance.get(&nkey) {
                    if *prevcost > ncost {
                        return None;
                    }
                }
                distance.insert(nkey, ncost);
                Some(Node {
                    pos: npos,
                    cost: ncost,
                })
            } else {
                None
            }
        }));
    }
    // println!("{:#?}", distance);
    answer
}

fn main() {
    let stdin = io::stdin();
    let grid = Grid::build(&mut stdin.lock());
    // grid.print();

    println!("Part 1: {}", longest_hike_step_count(&grid));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    fn get_grid() -> Grid {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        Grid::build(&mut reader)
    }

    #[test]
    fn test_walk() {
        let grid = get_grid();
        assert_eq!(walk(&grid, 1, South), Some((grid.pos(5, 3), 15)));
        assert_eq!(walk(&grid, 1, North), None);
        assert_eq!(walk(&grid, 1, East), None);
        assert_eq!(walk(&grid, 1, West), None);
        // 312, goes to exit
        assert_eq!(
            walk(&grid, grid.pos(13, 13), South),
            Some((grid.pos(22, 21), 25))
        );
    }

    #[test]
    fn test_part1() {
        let grid = get_grid();
        assert_eq!(longest_hike_step_count(&grid), 94);
    }
}
