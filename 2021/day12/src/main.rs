use std::{
    hash::{Hash, Hasher},
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHasher};

#[derive(Debug, Clone, Copy)]
enum CaveType {
    Start,
    End,
    Small,
    Big,
}

#[derive(Debug)]
struct Cave {
    name: String,
    ctype: CaveType,
    connections: Vec<usize>,
}

impl Cave {
    fn new(name: &str) -> Self {
        let cave_type = if name == "start" {
            CaveType::Start
        } else if name == "end" {
            CaveType::End
        } else if name.chars().all(|c| c.is_ascii_uppercase()) {
            CaveType::Big
        } else {
            CaveType::Small
        };

        Self {
            name: name.to_string(),
            ctype: cave_type,
            connections: Vec::new(),
        }
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

fn get_cave_types(graph: &[Cave]) -> Vec<CaveType> {
    graph.iter().map(|cave| cave.ctype).collect()
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

// Helps tracking if a cave is allowed to be visited.
struct VisitTracker {
    cave_types: Vec<CaveType>,
    visited: Vec<bool>,
    small_cave_visited_twice: Option<usize>,
}

impl VisitTracker {
    fn new(graph: &[Cave], allow_one_small_cave_twice: bool) -> Self {
        let cave_types = get_cave_types(graph);
        Self {
            cave_types,
            visited: vec![false; graph.len()],
            small_cave_visited_twice: if allow_one_small_cave_twice {
                None
            } else {
                // We set the value to an ID that doesn't exist.
                Some(usize::MAX)
            },
        }
    }

    fn visit(&mut self, i: usize) {
        match self.cave_types.get(i).unwrap() {
            CaveType::Start | CaveType::End => {
                self.visited[i] = true;
            }
            CaveType::Small => {
                // If that small cave is already visited once, but we haven't visited any small cave twice,
                // then mark this one as visited twice.
                if self.visited[i] && self.small_cave_visited_twice.is_none() {
                    self.small_cave_visited_twice = Some(i);
                } else {
                    self.visited[i] = true;
                }
            }
            // Big caves are never marked visited.
            CaveType::Big => {}
        }
    }

    fn unvisit(&mut self, i: usize) {
        match self.cave_types.get(i).unwrap() {
            CaveType::Start | CaveType::End => self.visited[i] = false,
            CaveType::Small => {
                if self.small_cave_visited_twice.is_some_and(|v| v == i) {
                    self.small_cave_visited_twice = None;
                    assert!(self.visited[i]);
                } else {
                    self.visited[i] = false;
                }
            }
            CaveType::Big => {}
        }
    }

    fn can_visit(&self, i: usize) -> bool {
        if matches!(self.cave_types.get(i), Some(CaveType::Small)) {
            self.small_cave_visited_twice.is_none() || !self.visited[i]
        } else {
            !self.visited[i]
        }
    }
}

fn hash(from: usize, visited: &VisitTracker) -> u64 {
    let mut hasher = FxHasher::default();
    from.hash(&mut hasher);
    // Not including to, it never changes (always end).
    visited.visited.hash(&mut hasher);
    visited.small_cave_visited_twice.hash(&mut hasher);
    hasher.finish()
}

// Finds all paths between `from` and `to`.
fn find_all_paths(
    graph: &[Cave],
    from: usize,
    to: usize,
    visited: &mut VisitTracker,
    cache: &mut FxHashMap<u64, usize>,
) -> usize {
    // Visit the current cave.
    visited.visit(from);

    let path_count = if from == to {
        // If source and destination are the same, we found a path.
        1
    } else {
        // Otherwise recursively explore all connected nodes.
        graph[from]
            .connections
            .iter()
            .map(|next| {
                if let Some(v) = cache.get(&hash(*next, visited)) {
                    return *v;
                }

                if visited.can_visit(*next) {
                    find_all_paths(graph, *next, to, visited, cache)
                } else {
                    0
                }
            })
            .sum()
    };

    // Mark current cave as unvisited.
    visited.unvisit(from);

    cache.insert(hash(from, visited), path_count);

    path_count
}

// Paths count that visit small caves at most once.
fn path_count(graph: &[Cave], allow_one_small_cave_twice: bool) -> usize {
    // We are doing a Depth First Traversal (DFS) of the graph.
    // Each cave we visit is added to the path.
    // Each cave visited that we are not allowed to go back to (small ones and start) is marked as visited.
    // Then for each cave, we explore recursively all connections.
    // When we reach the end, we increase the count of paths found.
    //
    // Idea from https://www.geeksforgeeks.org/find-paths-given-source-destination/

    let start_idx = find_cave(graph, "start").unwrap();
    let end_idx = find_cave(graph, "end").unwrap();

    let mut visited = VisitTracker::new(graph, allow_one_small_cave_twice);

    let mut cache: FxHashMap<u64, usize> = FxHashMap::default();

    find_all_paths(graph, start_idx, end_idx, &mut visited, &mut cache)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let graph = build(&input);

    // print_graphviz(&graph);

    println!("Part 1: {}", path_count(&graph, false));
    println!("Part 2: {}", path_count(&graph, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");

    #[test]
    fn test_part1() {
        assert_eq!(path_count(&build(INPUT_TEST_1), false), 10);
        assert_eq!(path_count(&build(INPUT_TEST_2), false), 19);
        assert_eq!(path_count(&build(INPUT_TEST_3), false), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(path_count(&build(INPUT_TEST_1), true), 36);
        assert_eq!(path_count(&build(INPUT_TEST_2), true), 103);
        assert_eq!(path_count(&build(INPUT_TEST_3), true), 3509);
    }
}
