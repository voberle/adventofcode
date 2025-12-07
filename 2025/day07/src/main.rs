use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};

struct Grid {
    values: Vec<char>,
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
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn find_start(&self) -> usize {
        self.values.iter().position(|c| *c == 'S').unwrap()
    }

    #[allow(dead_code)]
    fn print_with_beam(&self, positions: &[usize]) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("|");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }
}

// Part 1
fn bean_split_count(manifold: &Grid) -> usize {
    let mut split_count = 0;

    let mut beam_columns: FxHashSet<usize> = FxHashSet::default();
    // let mut beam_pos: Vec<usize> = Vec::new();

    let start_pos = manifold.find_start();
    beam_columns.insert(manifold.col(start_pos));

    // Go down row by row.
    for row in 1..manifold.rows {
        let mut next_beam_columns: FxHashSet<usize> = FxHashSet::default();
        for col in beam_columns {
            let pos = manifold.pos(row, col);
            match manifold.values[pos] {
                '.' => {
                    next_beam_columns.insert(col);
                }
                '^' => {
                    next_beam_columns.insert(col - 1);
                    next_beam_columns.insert(col + 1);
                    split_count += 1;
                }
                _ => panic!("Invalid manifold char"),
            }
        }

        beam_columns = next_beam_columns;

        // beam_pos.extend(beam_columns.iter().map(|c| manifold.pos(row, *c)));
        // manifold.print_with_beam(&beam_pos);
        // println!();
    }

    split_count
}

// Go down from the position until a splitter is found or we reach the end of the manifold.
// Returns the splitter position or None if we are at the end.
fn go_down(manifold: &Grid, start_pos: usize) -> Option<usize> {
    let mut p = start_pos;
    while p < manifold.values.len() {
        if manifold.values[p] == '^' {
            return Some(p);
        }
        p += manifold.cols;
    }
    None
}

// Each splitter is represented by a node.
// Each splitter has two next nodes. That can be another splitter or None (end of the manifold).
struct Node {
    manifold_position: usize,
    // The value here is the index in the graph, not the manifold position.
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new_unconnected(manifold_position: usize) -> Self {
        Self {
            manifold_position,
            left: None,
            right: None,
        }
    }
}

fn find_node_by_manifold_position(graph: &[Node], manifold_position: usize) -> usize {
    graph
        .iter()
        .position(
            |Node {
                 manifold_position: p,
                 left: _,
                 right: _,
             }| *p == manifold_position,
        )
        .unwrap()
}

// Convert the manifold into a graph.
fn make_graph(manifold: &Grid) -> Vec<Node> {
    let mut graph: Vec<Node> = Vec::new();

    // Create all the nodes of the graph, unconnected.
    for (pos, _) in manifold
        .values
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '^')
    {
        graph.push(Node::new_unconnected(pos));
    }

    // Connect the nodes.
    // We take each node and go down in the manifold to find the node to connect to.
    for i in 0..graph.len() {
        let splitter_pos = graph[i].manifold_position;

        if let Some(splitter_left) = go_down(manifold, splitter_pos - 1) {
            graph[i].left = Some(find_node_by_manifold_position(&graph, splitter_left));
        } else {
            graph[i].left = None;
        }

        if let Some(splitter_right) = go_down(manifold, splitter_pos + 1) {
            graph[i].right = Some(find_node_by_manifold_position(&graph, splitter_right));
        } else {
            graph[i].right = None;
        }
    }

    graph
}

fn timelines_count(manifold: &Grid) -> usize {
    let graph = make_graph(manifold);

    let mut cache: FxHashMap<usize, usize> = FxHashMap::default();

    // Walk the graph.
    let mut timelines = 1;
    timelines += walk(&graph, 0, &mut cache);
    timelines
}

// Recursive function.
fn walk(graph: &[Node], index: usize, cache: &mut FxHashMap<usize, usize>) -> usize {
    if let Some(cached) = cache.get(&index) {
        return *cached;
    }

    let mut timelines = 1;

    if let Some(next_splitter) = graph[index].left {
        timelines += walk(graph, next_splitter, cache);
    }
    if let Some(next_splitter) = graph[index].right {
        timelines += walk(graph, next_splitter, cache);
    }

    cache.insert(index, timelines);

    timelines
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let manifold = Grid::build(&input);

    println!("Part 1: {}", bean_split_count(&manifold));
    println!("Part 2: {}", timelines_count(&manifold));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(bean_split_count(&Grid::build(INPUT_TEST)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(timelines_count(&Grid::build(INPUT_TEST)), 40);
    }
}
