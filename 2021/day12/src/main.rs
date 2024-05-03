use std::io::{self, Read};

use fxhash::FxHashSet;

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_uppercase())
}

#[derive(Debug)]
struct Cave {
    name: String,
    is_big: bool,
    is_start: bool,
    is_end: bool,
    connections: Vec<usize>,
}

impl Cave {
    fn new(name: &str) -> Self {
        let is_start = name == "start";
        let is_end = name == "end";
        let is_big = !is_start && !is_end && is_uppercase(name);
        Self {
            name: name.to_string(),
            is_big,
            is_start,
            is_end,
            connections: Vec::new(),
        }
    }

    fn is_small_cave(&self) -> bool {
        !self.is_start && !self.is_end && !self.is_big
    }
}

fn find_cave(graph: &[Cave], name: &str) -> Option<usize> {
    graph.iter().position(|c| c.name == name)
}

fn build(input: &str) -> Vec<Cave> {
    let mut graph = Vec::new();
    for line in input.lines() {
        let indexes: Vec<usize> = line
            .split('-')
            .map(|name| {
                if let Some(cave_idx) = find_cave(&graph, name) {
                    cave_idx
                } else {
                    graph.push(Cave::new(name));
                    graph.len() - 1
                }
            })
            .collect();
        // Connect the nodes.
        graph[indexes[0]].connections.push(indexes[1]);
        graph[indexes[1]].connections.push(indexes[0]);
    }
    graph
}

// dot -Tpdf -Ksfdp resources/input_test_1.gv > resources/input.pdf
#[allow(dead_code)]
fn print_graphviz(graph: &[Cave]) {
    // strict is to remove duplicate edges.
    println!("strict graph {{");
    for cave in graph {
        for conn in &cave.connections {
            println!("\t{} -- {};", cave.name, graph[*conn].name);
        }
    }
    println!("}}");
}

// Finds all paths between `from` and `to`.
//
// `visited` is a vector of boolean indicating if the cave with that index has been visited so far.
// visited should be set only for small caves, as big ones are allowed to be visited multuple times.
// `path` is the path currently being built.
// `all_paths` contains all unique paths found so far.
fn find_all_paths(
    graph: &[Cave],
    from: usize,
    to: usize,
    visited: &mut Vec<bool>,
    path: &mut Vec<usize>,
    all_paths: &mut FxHashSet<Vec<usize>>,
) {
    // Mark the current cave as visited (if it's a small one) and add it to the path.
    if graph[from].is_small_cave() || graph[from].is_start {
        visited[from] = true;
    }
    path.push(from);

    if from == to {
        // If source and destination are the same, we found a path.
        all_paths.insert(path.clone());
    } else {
        // Otherwise recursively explore all connected nodes.
        for next in &graph[from].connections {
            if !visited[*next] {
                find_all_paths(graph, *next, to, visited, path, all_paths);
            }
        }
    }

    // Remove current cave from path, mark as unvisited.
    visited[from] = false;
    path.pop();
}

// Paths count that visit small caves at most once.
fn unique_path_count(graph: &[Cave]) -> usize {
    // We are doing a Depth First Traversal (DFS) of the graph.
    // Each cave we visit is added to the path.
    // Each cave visited that we are not allowed to go back to (small ones and start) is marked as visited.
    // Then for each cave, we explore recursively all connections.
    // When we reach the end, we add the path to the set of path found.
    //
    // Idea from https://www.geeksforgeeks.org/find-paths-given-source-destination/

    let mut all_paths: FxHashSet<Vec<usize>> = FxHashSet::default();
    let start_idx = find_cave(graph, "start").unwrap();
    let end_idx = find_cave(graph, "end").unwrap();

    let mut visited = vec![false; graph.len()];
    let mut path: Vec<usize> = Vec::new();

    find_all_paths(
        graph,
        start_idx,
        end_idx,
        &mut visited,
        &mut path,
        &mut all_paths,
    );

    all_paths.len()
}

fn part2(graph: &[Cave]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let graph = build(&input);

    // print_graphviz(&graph);

    println!("Part 1: {}", unique_path_count(&graph));
    println!("Part 2: {}", part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part1() {
        assert_eq!(unique_path_count(&build(INPUT_TEST_1)), 10);
        assert_eq!(unique_path_count(&build(INPUT_TEST_2)), 19);
        assert_eq!(unique_path_count(&build(INPUT_TEST_3)), 226);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
