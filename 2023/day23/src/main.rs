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

fn slopes_to_dir(c: char) -> Direction {
    match c {
        '<' => West,
        '>' => East,
        // Observation: ^ is never found in the input.
        '^' => North,
        'v' => South,
        _ => panic!("Invalid char {}", c),
    }
}

// Checks if it's possible to go towards that direction from that position.
fn can_go_towards<const IGNORE_SLOPES: bool>(grid: &Grid, pos: usize, d: Direction) -> bool {
    if let Some(npos) = grid.try_next_pos(pos, d) {
        let c = grid.values[npos];
        match c {
            '#' => false, // Not allowed to go into forest
            '.' => true,  // Paths are ok
            '<' | '>' | '^' | 'v' => {
                if IGNORE_SLOPES {
                    true
                } else {
                    d == slopes_to_dir(c)
                }
            }
            _ => panic!("Invalid char in map {}", c),
        }
    } else {
        // Falling out of the grid.
        false
    }
}

// Indicates if it's an intersection, meaning 3 or 4 paths from here.
fn is_intersection(grid: &Grid, pos: usize) -> bool {
    let c = grid.values[pos];
    if c == '#' {
        return false;
    }
    ALL_DIRECTIONS
        .iter()
        .filter_map(|&d| {
            if can_go_towards::<true>(grid, pos, d) {
                Some(d)
            } else {
                None
            }
        })
        .count()
        > 2
}

// Walk until we reach an intersection, taking the specified towards direction.
// Returns the intersection position and the distance to get there.
// None means we got blocked.
// Exit of the grid is considered an intersection.
fn walk<const IGNORE_SLOPES: bool>(
    grid: &Grid,
    from: usize,
    towards: Direction,
) -> Option<(usize, u32)> {
    // First direction is forced, no choice.
    if !can_go_towards::<IGNORE_SLOPES>(grid, from, towards) {
        return None;
    }

    let mut pos = grid.next_pos(from, towards);
    let mut steps = 1;
    let mut forbidden_dir = towards.opposite();
    loop {
        // Find all the directions we are allowed to walk
        let dirs: Vec<Direction> = ALL_DIRECTIONS
            .iter()
            .filter_map(|&d| {
                if d == forbidden_dir {
                    // Not allowed to go back
                    None
                } else if grid.allowed(pos, d) {
                    // Not at the border
                    if can_go_towards::<IGNORE_SLOPES>(grid, pos, d) {
                        Some(d)
                    } else {
                        None
                    }
                } else {
                    // Start / end
                    Some(d)
                }
            })
            .collect();

        if dirs.is_empty() {
            // dead-end
            return None;
        }

        if is_intersection(grid, pos) {
            // intersection
            return Some((pos, steps));
        }

        // continue walking
        assert_eq!(dirs.len(), 1);
        if let Some(npos) = grid.try_next_pos(pos, dirs[0]) {
            pos = npos;
            forbidden_dir = dirs[0].opposite();
            steps += 1;
        } else {
            // fell out of the grid, so we are at start or end
            return Some((pos, steps));
        }
    }
}

// Nodes for a graph of intersections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Intersection(usize),
}

impl Node {
    fn new(pos: usize, start: usize, end: usize) -> Self {
        if pos == start {
            Node::Start
        } else if pos == end {
            Node::End
        } else {
            Node::Intersection(pos)
        }
    }

    fn name(&self) -> String {
        match self {
            Node::Start => "start".to_string(),
            Node::End => "end".to_string(),
            Node::Intersection(pos) => pos.to_string(),
        }
    }
}

// Values are an array corresponding to the 4 directions.
type Graph = HashMap<Node, [Option<(Node, u32)>; 4]>;

// Go through the grid and build the graph of intersections
fn build_graph<const IGNORE_SLOPES: bool>(grid: &Grid, start: usize, end: usize) -> Graph {
    let mut graph: Graph = HashMap::new();

    // Add all the nodes to the graph
    for pos in 0..grid.values.len() {
        if pos == start {
            graph.insert(Node::Start, [None; 4]);
        } else if pos == end {
            graph.insert(Node::End, [None; 4]);
        } else if is_intersection(grid, pos) {
            graph.insert(Node::Intersection(pos), [None; 4]);
        }
    }

    // Connect the nodes
    graph.iter_mut().for_each(|(key, val)| {
        // println!("Connecting nodes for {:?}", key);
        match key {
            Node::Start => {
                let dir = Direction::South;
                if let Some((pos, steps)) = walk::<IGNORE_SLOPES>(grid, start, dir) {
                    let node = Node::new(pos, start, end);
                    val[dir.index()] = Some((node, steps));
                }
            }
            Node::End => {
                let dir = Direction::North;
                if let Some((pos, steps)) = walk::<IGNORE_SLOPES>(grid, end, dir) {
                    let node = Node::new(pos, start, end);
                    val[dir.index()] = Some((node, steps));
                }
            }
            Node::Intersection(from) => ALL_DIRECTIONS.iter().for_each(|&dir| {
                if let Some((pos, steps)) = walk::<IGNORE_SLOPES>(grid, *from, dir) {
                    let node = Node::new(pos, start, end);
                    // println!(" {:?}: intersection towards {:?} => {} - {:?}", key, dir, pos, node);
                    val[dir.index()] = Some((node, steps));
                }
            }),
        }
    });
    graph
}

fn get_intersections(graph: &Graph) -> Vec<usize> {
    graph
        .keys()
        .filter_map(|n| match n {
            Node::Intersection(pos) => Some(*pos),
            _ => None,
        })
        .collect()
}

// Prints the graph in Graphviz Dot format
// dot -Tpdf -Ksfdp resources/input_test.graph > resources/input_test.pdf
fn print_graph_as_graphviz<const IGNORE_SLOPES: bool>(graph: &Graph) {
    let edgeop = if IGNORE_SLOPES { "--" } else { "->" };
    println!("digraph Maze {{");
    for (key, val) in graph.iter() {
        for node in val.iter().flatten() {
            println!(
                "    {} {} {} [label=\"{}\"];",
                key.name(),
                edgeop,
                node.0.name(),
                node.1
            );
        }
    }
    println!("}}");
}

// Graph traversal, as example how to do DFS.
fn traverse_graph(graph: &Graph) {
    let start = Node::Start;

    // DFS (Depth First Search)
    let mut discovered: HashSet<Node> = HashSet::new();
    let mut stack: Vec<Node> = Vec::new();
    stack.push(start);
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        if !discovered.contains(&node) {
            println!("-> {:?}", node);
            discovered.insert(node);

            graph
                .get(&node)
                .unwrap()
                .iter()
                .filter_map(|node_len| {
                    if let Some(n) = node_len {
                        Some(n.0)
                    } else {
                        None
                    }
                })
                .for_each(|next| {
                    stack.push(next);
                });
        }
    }
}

// Recursive traversal of the graph to find the longest path.
fn longest_path_rec(
    graph: &Graph,
    longest_path: &mut HashMap<Node, u32>, // longest path from start node
    discovered: &mut HashSet<Node>,
    node: &Node,
    curr_sum: u32,
) {
    if discovered.contains(&node) {
        return;
    }
    discovered.insert(*node);

    // if the found path is longer, save it
    longest_path
        .entry(*node)
        .and_modify(|e| {
            if *e < curr_sum {
                *e = curr_sum
            }
        })
        .or_insert(0);

    // call the method on the neighbour nodes
    graph.get(node).unwrap().iter().for_each(|val| {
        if let Some(node_len) = val {
            let length = curr_sum + node_len.1;
            longest_path_rec(graph, longest_path, discovered, &node_len.0, length);
        }
    });

    discovered.remove(node);
}

fn longest_hike_step_count<const IGNORE_SLOPES: bool>(grid: &Grid) -> u32 {
    let start = 1;
    let end = grid.pos(grid.rows - 1, grid.cols - 2);

    let graph = build_graph::<IGNORE_SLOPES>(&grid, start, end);

    let mut discovered: HashSet<Node> = HashSet::new();
    let mut longest_path: HashMap<Node, u32> = HashMap::new();
    longest_path_rec(&graph, &mut longest_path, &mut discovered, &Node::Start, 0);

    *longest_path.get(&Node::End).unwrap()
}

fn debug_graph(grid: &Grid) {
    let start = 1;
    let end = grid.pos(grid.rows - 1, grid.cols - 2);

    let graph = build_graph::<false>(&grid, start, end);
    let intersections = get_intersections(&graph);
    println!("{:?}", intersections);
    grid.print_with_pos(&intersections);
    print_graph_as_graphviz::<false>(&graph);
}

fn main() {
    let stdin = io::stdin();
    let grid = Grid::build(&mut stdin.lock());

    println!("Part 1: {}", longest_hike_step_count::<false>(&grid));
    println!("Part 2: {}", longest_hike_step_count::<true>(&grid));
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
    fn test_is_intersection() {
        let grid = get_grid();
        assert!(!is_intersection(&grid, 1));
        assert!(is_intersection(&grid, 312));
        assert!(!is_intersection(&grid, 313));
    }

    #[test]
    fn test_walk() {
        let g = get_grid();
        assert_eq!(walk::<false>(&g, 1, South), Some((g.pos(5, 3), 15)));
        assert_eq!(walk::<false>(&g, 1, North), None);
        assert_eq!(walk::<false>(&g, 1, East), None);
        assert_eq!(walk::<false>(&g, 1, West), None);
        let p = g.pos(13, 13); // 312
        assert_eq!(walk::<false>(&g, p, South), Some((g.pos(19, 13), 10)));
        // 456, goes to exit
        let p1 = g.pos(19, 19);
        assert_eq!(walk::<false>(&g, p1, South), Some((g.pos(22, 21), 5)));
    }

    #[test]
    fn test_part1_2() {
        let grid = get_grid();
        assert_eq!(longest_hike_step_count::<false>(&grid), 94);

        assert_eq!(longest_hike_step_count::<true>(&grid), 154);
    }
}
