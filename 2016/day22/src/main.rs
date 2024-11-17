use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    size: u32,
    used: u32,
    avail: u32,
    use_perc: u32,
}

#[inline]
fn int<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}

fn build(input: &str) -> Vec<Node> {
    // Filesystem              Size  Used  Avail  Use%
    let re =
        Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
    input
        .lines()
        .skip(2)
        .map(|line| {
            if let Some(parts) = re.captures(line) {
                Node {
                    x: int(&parts[1]),
                    y: int(&parts[2]),
                    size: int(&parts[3]),
                    used: int(&parts[4]),
                    avail: int(&parts[5]),
                    use_perc: int(&parts[6]),
                }
            } else {
                panic!("Invalid input {line}")
            }
        })
        .collect()
}

// True here means that a could be moved to b.
fn is_viable_pair(a: &Node, b: &Node) -> bool {
    a.used != 0 && a != b && a.used <= b.avail
}

fn viable_pairs_count(nodes: &[Node]) -> usize {
    nodes
        .iter()
        .map(|a| nodes.iter().filter(move |b| is_viable_pair(a, b)).count())
        .sum()
}

// Converts the list of nodes in an easier to manipulate 2-dimensional array.
// We return an array of references, so the original array cannot be dropped.
fn convert_graph(nodes: &[Node]) -> Vec<Vec<&Node>> {
    let mut graph: Vec<Vec<&Node>> = Vec::new();
    for y in 0.. {
        let mut line = Vec::new();
        for x in 0.. {
            if let Some(node) = nodes.iter().find(|n| n.x == x && n.y == y) {
                line.push(node);
            } else {
                break;
            }
        }
        graph.push(line);
        if graph.len() * graph[0].len() == nodes.len() {
            break;
        }
    }
    graph
}

// There is one and only one node empty in test and real input.
fn find_empty_node(nodes: &[Node]) -> &Node {
    let empty_node = nodes
        .iter()
        .find(|n| n.used == 0)
        .expect("Didn't find an empty node");
    empty_node
}

// All nodes that could pair with the empty one.
fn viable_nodes<'a>(nodes: &'a [Node], empty_node: &Node) -> Vec<&'a Node> {
    nodes
        .iter()
        .filter(|a| is_viable_pair(a, empty_node))
        .collect()
}

// Prints the graph so it can be analyzed.
// We highlight the empty node [______] and the nodes that are not viable and cannot be used [######].
fn print_graph(graph: &[Vec<&Node>], empty_node: &Node, viable_nodes: &[&Node]) {
    fn print_node(n: &Node, empty_node: &Node, viable_nodes: &[&Node]) {
        if n == empty_node {
            print!(" [______] ");
        } else if !viable_nodes.contains(&n) {
            print!(" [######] ");
        } else {
            print!("{:3}T/{:3}T ", n.used, n.size);
            // print!("x{:<3}/y{:<3} ", n.x, n.y);
        }
    }

    for y in 0..graph.len() {
        for x in 0..graph[y].len() - 1 {
            let n = graph[y][x];
            print_node(n, empty_node, viable_nodes);
            let next = graph[y][x + 1];
            if is_viable_pair(n, next) {
                print!("->");
            } else if is_viable_pair(next, n) {
                print!("<-");
            } else {
                print!("  ");
            }
        }
        let n = graph[y][graph[y].len() - 1];
        print_node(n, empty_node, viable_nodes);
        println!();

        if y < graph.len() - 1 {
            for x in 0..graph[y].len() - 1 {
                let a = graph[y][x];
                let b = graph[y + 1][x];
                if is_viable_pair(a, b) {
                    print!("    vv      ");
                } else if is_viable_pair(b, a) {
                    print!("    ^^      ");
                } else {
                    print!("            ");
                }
            }
            println!();
        }
    }
}

// Helper function to calculate the minimal number of steps between two nodes (in "staircase" path).
fn steps_between(a: &Node, b: &Node) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

// We find the answer by applying the method described in the explanation:
// - First move the empty space to the goal
// - Then get the goal to the start.
// By looking at the visualization of the graph (cf `print_graph`), we saw that all nodes
// are usable except for a horizontal line. So we move around this horizontal line,
// and we go as directly as possible, i.e. straight lines.
fn count_steps(graph: &[Vec<&Node>], empty_node: &Node) -> usize {
    let corner_node = graph[7][4];
    let start = graph[0][0];
    let goal = graph[0][graph[0].len() - 1];

    // First move the empty spot to the goal, by going around the line that blocks us.
    // (.) .  .  .  .  G
    //  .  .  .  .  .  .
    //  .  .  C  #  #  #
    //  .  .  .  .  .  .
    //  .  .  .  _  .  .
    let empty_to_goal = steps_between(empty_node, corner_node) + steps_between(corner_node, goal);

    // Which gets us to:
    // (.) .  .  .  G  _
    //  .  .  .  .  .  .
    //  .  .  .  #  #  #
    //  .  .  .  .  .  .
    //  .  .  .  .  .  .

    // Then we need to move goal (which is now just left of top-right corner) to the start place.
    // For this we need to move empty around it on each step, which needs 4 extra steps.
    // .  G  _  =>  .  G  .  =>  .  G  .  =>  .  G  .  =>  _  G  .  =>  G  _  .
    // .  .  .      .  .  _      .  _  .      _  .  .      .  .  .      .  .  .
    let goal_to_start = steps_between(goal, start) - 1;
    let back_to_start = goal_to_start * 5;

    empty_to_goal + back_to_start
}

fn fewest_steps_goal_to_start(nodes: &[Node], print: bool) -> usize {
    let empty_node = find_empty_node(nodes);
    let graph = convert_graph(nodes);

    if print {
        let viable_nodes = viable_nodes(nodes, empty_node);
        print_graph(&graph, empty_node, &viable_nodes);
    }

    count_steps(&graph, empty_node)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nodes = build(&input);

    println!("Part 1: {}", viable_pairs_count(&nodes));
    println!("Part 2: {}", fewest_steps_goal_to_start(&nodes, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(viable_pairs_count(&build(INPUT_TEST)), 7);
    }
}
