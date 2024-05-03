use std::io::{self, Read};

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_uppercase())
}

#[derive(Debug)]
struct Cave {
    name: String,
    big: bool,
    start: bool,
    end: bool,
    connections: Vec<usize>,
}

impl Cave {
    fn new(name: &str) -> Self {
        let start = name == "start";
        let end = name == "end";
        let big = !start && !end && is_uppercase(name);
        Self {
            name: name.to_string(),
            big,
            start,
            end,
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

// Paths count that visit small caves at most once.
fn unique_path_count(graph: &[Cave]) -> i64 {
    0
}

fn part2(graph: &[Cave]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let graph = build(&input);

    print_graphviz(&graph);

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
