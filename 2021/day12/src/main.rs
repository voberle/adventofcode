use std::io::{self, Read};

use fxhash::FxHashSet;

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
trait VisitTracker {
    fn visit(&mut self, i: usize);
    fn unvisit(&mut self, i: usize);
    fn can_visit(&self, i: usize) -> bool;
}

// Implementation allowing only 1 visit per small cave.
struct TrackerSingleVisit {
    cave_types: Vec<CaveType>,
    visited: Vec<bool>,
}

impl TrackerSingleVisit {
    fn new(graph: &[Cave]) -> Self {
        Self {
            cave_types: get_cave_types(graph),
            visited: vec![false; graph.len()],
        }
    }
}

impl VisitTracker for TrackerSingleVisit {
    fn visit(&mut self, i: usize) {
        match self.cave_types.get(i).unwrap() {
            CaveType::Start | CaveType::End | CaveType::Small => {
                self.visited[i] = true;
            }
            // Big caves are never marked visited.
            CaveType::Big => {}
        }
    }

    fn unvisit(&mut self, i: usize) {
        self.visited[i] = false;
    }

    fn can_visit(&self, i: usize) -> bool {
        !self.visited[i]
    }
}

// Implementation allowing two visits to one small cave and one to the others.
struct TrackerVisitOneExtra {
    cave_types: Vec<CaveType>,
    visited: Vec<bool>,
    small_cave_visited_twice: usize,
}

impl TrackerVisitOneExtra {
    fn new(graph: &[Cave]) -> Self {
        let cave_types = get_cave_types(graph);
        Self {
            cave_types,
            visited: vec![false; graph.len()],
            small_cave_visited_twice: usize::MAX,
        }
    }
}

impl VisitTracker for TrackerVisitOneExtra {
    fn visit(&mut self, i: usize) {
        match self.cave_types.get(i).unwrap() {
            CaveType::Start | CaveType::End => {
                self.visited[i] = true;
            }
            CaveType::Small => {
                // If that small cave is already visited once, but we haven't visited any small cave twice,
                // then mark this one as visited twice.
                if self.visited[i] && self.small_cave_visited_twice == usize::MAX {
                    self.small_cave_visited_twice = i;
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
                if self.small_cave_visited_twice == i {
                    self.small_cave_visited_twice = usize::MAX;
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
            self.small_cave_visited_twice == usize::MAX || !self.visited[i]
        } else {
            !self.visited[i]
        }
    }
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
    visited: &mut Box<dyn VisitTracker>,
    path: &mut Vec<usize>,
    all_paths: &mut FxHashSet<Vec<usize>>,
) {
    // Visit the current cave and add it to the path.
    visited.visit(from);
    path.push(from);

    if from == to {
        // If source and destination are the same, we found a path.
        all_paths.insert(path.clone());
    } else {
        // Otherwise recursively explore all connected nodes.
        for next in &graph[from].connections {
            if visited.can_visit(*next) {
                find_all_paths(graph, *next, to, visited, path, all_paths);
            }
        }
    }

    // Remove current cave from path, mark as unvisited.
    visited.unvisit(from);
    path.pop();
}

// Paths count that visit small caves at most once.
// ONE_SMALL_CAVE_TWICE indicates that one small cave can be visited twice.
fn path_count(graph: &[Cave], allow_one_small_cave_twice: bool) -> usize {
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

    let mut visited: Box<dyn VisitTracker> = if allow_one_small_cave_twice {
        Box::new(TrackerVisitOneExtra::new(graph))
    } else {
        Box::new(TrackerSingleVisit::new(graph))
    };
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
