//! This module handles the parsing of the reg ex into a graph.

use std::{
    collections::BinaryHeap,
    io::{Error, Write},
    slice::Iter,
};

use fxhash::{FxHashMap, FxHashSet};

use crate::{Direction, Map, Pos};

// Intermediary structure representing the reg ex, to make life easier.
// It won't be the final graph.
#[derive(Debug)]
enum Elt {
    Value(Vec<Direction>, usize), // The second param is just an index counting each value.
    OpenGroup(usize),
    CloseGroup(usize),
    Pipe,
}

// When `skip_empty_options` is set, empty options like "(WNSE|)" will be skipped.
fn preprocess_regex(regex: &[u8], skip_empty_options: bool) -> Vec<Elt> {
    // Ignoring first ^ and last $
    let regex = &regex[1..regex.len() - 1];

    let mut output: Vec<Elt> = Vec::new();
    let mut level = 0;
    let mut value_idx = 0;
    for c in regex {
        if let Some(dir) = Direction::new(*c) {
            if let Some(Elt::Value(last, _)) = output.last_mut() {
                last.push(dir);
            } else {
                output.push(Elt::Value(vec![dir], value_idx));
                value_idx += 1;
            }
        } else if *c == b'(' {
            level += 1;
            output.push(Elt::OpenGroup(level));
        } else if *c == b'|' {
            output.push(Elt::Pipe);
        } else if *c == b')' {
            if skip_empty_options {
                if let Some(Elt::Pipe) = output.last() {
                    output.pop();
                    output.pop();
                    value_idx -= 1;
                }
            }
            output.push(Elt::CloseGroup(level));
            level -= 1;
        } else {
            panic!("Invalid char in regex {}", c)
        }
    }
    output
}

// The node of the graph we use to represent the regex.
#[derive(Debug, PartialEq)]
struct GraphNode {
    value: Vec<Option<Direction>>,
    next: FxHashSet<usize>,
}

impl GraphNode {
    #[cfg(test)]
    fn new(value: &str, next: &[usize]) -> Self {
        Self {
            value: value.bytes().map(|c| Direction::new(c)).collect(),
            next: FxHashSet::from_iter(next.iter().cloned()),
        }
    }

    #[allow(dead_code)]
    fn dirs_to_string(&self) -> String {
        self.value
            .iter()
            .map(|v| {
                if let Some(d) = v {
                    d.to_string()
                } else {
                    ".".to_string()
                }
            })
            .collect()
    }
}

fn parse_regex(regex: &[u8]) -> Vec<GraphNode> {
    parse_regex_with(regex, false)
}

fn parse_regex_with(regex: &[u8], skip_empty_options: bool) -> Vec<GraphNode> {
    let regex_elts = preprocess_regex(regex, skip_empty_options);

    // Create the node vector with all the values
    let mut nodes: Vec<GraphNode> = regex_elts
        .iter()
        .filter_map(|e| {
            if let Elt::Value(s, _) = e {
                Some(GraphNode {
                    value: s.iter().map(|x| Some(*x)).collect(),
                    next: FxHashSet::default(),
                })
            } else {
                None
            }
        })
        .collect();

    // Now we need to set the next field of the nodes, creating the graph.
    let mut it = regex_elts.iter();
    it.next();
    update_nodes(&mut nodes, &mut it, 0);

    nodes
}

// Recursive function that sets the connections of the graphs.
// level_idx points to the node just before this group
fn update_nodes(
    nodes: &mut Vec<GraphNode>,
    it: &mut Iter<'_, Elt>,
    level_idx: usize,
) -> Vec<usize> {
    // current_idx will be the index of the value elt iterated on.
    let mut current_idx: usize = level_idx;

    let mut exit_nodes: Vec<usize> = Vec::new();
    let mut last_exit_nodes: Vec<usize> = Vec::new();
    while let Some(elt) = it.next() {
        match elt {
            Elt::Value(_val, idx) => {
                if last_exit_nodes.is_empty() {
                    nodes[current_idx].next.insert(*idx);
                } else {
                    for n in &last_exit_nodes {
                        nodes[*n].next.insert(*idx);
                    }
                    last_exit_nodes.clear();
                }
                current_idx = *idx;
            }
            Elt::OpenGroup(_) => {
                last_exit_nodes = update_nodes(nodes, it, current_idx);
            }
            Elt::CloseGroup(_) => {
                exit_nodes.push(current_idx);
                return exit_nodes;
            }
            Elt::Pipe => {
                last_exit_nodes.clear();
                exit_nodes.push(current_idx);
                current_idx = level_idx;
            }
        }
    }
    exit_nodes
}

// Prints a Graphviz version of the regex.
// View with:
//  dot -Tpdf resources/input.gv > input.pdf
//
// A good way to check it is to compare it against the output from
// https://regexper.com
#[allow(dead_code)]
fn write_graphviz<W>(out: &mut W, nodes: &[GraphNode]) -> Result<(), Error>
where
    W: Write,
{
    writeln!(out, "digraph {{")?;
    writeln!(out, "\trankdir=LR;")?;
    writeln!(out, "\tnode [shape = diamond]; \"END\";")?;
    writeln!(out, "\tnode [shape = oval];")?;

    for (i, n) in nodes.iter().enumerate() {
        let name = n.dirs_to_string();
        writeln!(out, "\t{} [shape = oval label = \"{}\"];", i, name)?;
    }

    for (i, f) in nodes.iter().enumerate() {
        if f.next.is_empty() {
            writeln!(out, "\t{} -> END;", i)?;
        } else {
            for n in &f.next {
                writeln!(out, "\t{} -> {};", i, n)?;
            }
        }
    }
    writeln!(out, "}}")?;
    Ok(())
}

// Attempt to build the map from the graph, but it doesn't work for the full input.
#[allow(dead_code)]
fn build_map(graph: &[GraphNode]) -> Map {
    let mut map: Map = Map::new();

    let pos = Pos::new(0, 0);
    // While building, "false" in the allowed array actually means "maybe",
    // but at the end it means "wall".
    map.0.insert(pos, [false, false, false, false]);

    walk(graph, 0, pos, &mut map);

    map
}

fn walk(graph: &[GraphNode], node_idx: usize, pos: Pos, map: &mut Map) {
    // println!("Walking {}", node_idx);

    let mut pos = pos;
    for dir in &graph[node_idx].value {
        let dir = dir.unwrap();
        // We can go in that direction from current position.
        map.update(pos, dir);

        // From next position, we can go back.
        pos = pos.next(dir);
        map.update(pos, dir.opposite());
    }

    // A clone() is needed for the borrow checker
    let next_nodes = graph[node_idx].next.clone();
    for n in next_nodes {
        walk(graph, n, pos, map);
    }
}

// Node we are exploring with Dijkstra.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize, // Index in the parsing::Node vector
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
fn find_shortest_path(graph: &[GraphNode], start: usize, end: usize) -> usize {
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
            shortest_distance = usize::min(shortest_distance, cost);
            continue;
        }

        queue.extend(graph[pos].next.iter().filter_map(|next_pos| {
            if visited.contains(next_pos) {
                return None;
            }
            let next_cost = cost + graph[pos].value.len();
            if let Some(prevcost) = distance.get(next_pos) {
                if *prevcost <= next_cost {
                    return None;
                }
            }
            distance.insert(*next_pos, next_cost);
            Some(Node {
                pos: *next_pos,
                cost: next_cost,
            })
        }));
    }
    // Need to add the last node len, as it's not been added before.
    shortest_distance + graph[end].value.len()
}

// Largest number of doors required to pass through to reach a room.
#[allow(dead_code)]
pub fn dist_to_furthest_room(regex: &[u8]) -> usize {
    let graph = parse_regex(regex);
    // parsing::print_graphviz(&nodes);

    // let map = build_map(&graph);
    // println!("{}", regex_to_string(regex));
    // println!("{}", map);

    // Find all the nodes that don't have any next, meaning they are at the end.
    let ending_nodes: Vec<usize> = graph
        .iter()
        .enumerate()
        .filter(|(_, n)| n.next.is_empty())
        .map(|(i, _)| i)
        .collect();

    // Compute the shortest path to each of those ending nodes, and take the max.
    // This produces the right answer, probably because no path overlap each other?
    ending_nodes
        .iter()
        .map(|end| find_shortest_path(&graph, 0, *end))
        .max()
        .unwrap()
}

// Walk through the nodes and mark all rooms less than 1000 doors away
fn walk_and_mark<const LIMIT: usize>(graph: &mut Vec<GraphNode>, node: usize, steps: usize) {
    if steps >= LIMIT - 1 {
        return;
    }

    let mut steps = steps;
    for i in 0..graph[node].value.len() {
        graph[node].value[i] = None;
        steps += 1;
        if steps >= LIMIT - 1 {
            break;
        }
    }

    // A clone() is needed for the borrow checker
    let next_nodes = graph[node].next.clone();
    for n in next_nodes {
        walk_and_mark::<LIMIT>(graph, n, steps);
    }
}

#[allow(dead_code)]
pub fn rooms_dist_over_1000_doors(regex: &[u8]) -> usize {
    let mut nodes = parse_regex(regex);

    // Parsing and ignore empty nodes is possibly working for some inputs,
    // but it doesn't for mine.
    // let mut nodes = parsing::parse_regex_with(regex, true);

    walk_and_mark::<1000>(&mut nodes, 0, 0);

    // Unlike part 1, this doesn't work. Don't know why yet.
    nodes
        .iter()
        .map(|n| n.value.iter().filter(|d| d.is_some()).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::tests::INPUT_TEST_1;
    use crate::tests::INPUT_TEST_2;
    use crate::tests::INPUT_TEST_3;
    use crate::tests::INPUT_TEST_4;
    use crate::tests::INPUT_TEST_5;
    use crate::tests::INPUT_TEST_6;

    use super::*;

    #[test]
    fn test_preprocess_regex() {
        let regex = preprocess_regex(INPUT_TEST_4, false);
        // println!("{:#?}", regex);
        assert_eq!(regex.len(), 21);
    }

    #[test]
    fn test_parse_regex() {
        let graph = parse_regex(INPUT_TEST_6);
        for (i, n) in graph.iter().enumerate() {
            println!("{}: {}; next={:?}", i, n.dirs_to_string(), n.next);
        }

        // https://regexper.com/#%5EENNWSWW%28NEWS%7C%29SSSEEN%28WNSE%7C%29EE%28SWEN%7C%29NNN%24
        assert_eq!(
            parse_regex(INPUT_TEST_3),
            vec![
                /* 0 */ GraphNode::new("ENNWSWW", &[1, 2]),
                /* 1 */ GraphNode::new("NEWS", &[2]),
                /* 2 */ GraphNode::new("SSSEEN", &[4, 3]),
                /* 3 */ GraphNode::new("WNSE", &[4]),
                /* 4 */ GraphNode::new("EE", &[5, 6]),
                /* 5 */ GraphNode::new("SWEN", &[6]),
                /* 6 */ GraphNode::new("NNN", &[]),
            ]
        );

        // https://regexper.com/#%5EESSWWN%28E%7CNNENN%28EESS%28WNSE%7C%29SSS%7CWWWSSSSE%28SW%7CNNNE%29%29%29%24
        assert_eq!(
            parse_regex(INPUT_TEST_4),
            vec![
                /* 0 */ GraphNode::new("ESSWWN", &[1, 2]),
                /* 1 */ GraphNode::new("E", &[]),
                /* 2 */ GraphNode::new("NNENN", &[6, 3]),
                /* 3 */ GraphNode::new("EESS", &[4, 5]),
                /* 4 */ GraphNode::new("WNSE", &[5]),
                /* 5 */ GraphNode::new("SSS", &[]),
                /* 6 */ GraphNode::new("WWWSSSSE", &[8, 7]),
                /* 7 */ GraphNode::new("SW", &[]),
                /* 8 */ GraphNode::new("NNNE", &[]),
            ]
        );

        // https://regexper.com/#%5EWSSEESWWWNW%28S%7CNENNEEEENN%28ESSSSW%28NWSW%7CSSEN%29%7CWSWWN%28E%7CWWS%28E%7CSS%29%29%29%29%24
        assert_eq!(
            parse_regex(INPUT_TEST_5),
            vec![
                /* 0 */ GraphNode::new("WSSEESWWWNW", &[1, 2]),
                /* 1 */ GraphNode::new("S", &[]),
                /* 2 */ GraphNode::new("NENNEEEENN", &[6, 3]),
                /* 3 */ GraphNode::new("ESSSSW", &[4, 5]),
                /* 4 */ GraphNode::new("NWSW", &[]),
                /* 5 */ GraphNode::new("SSEN", &[]),
                /* 6 */ GraphNode::new("WSWWN", &[8, 7]),
                /* 7 */ GraphNode::new("E", &[]),
                /* 8 */ GraphNode::new("WWS", &[9, 10]),
                /* 9 */ GraphNode::new("E", &[]),
                /* 10*/ GraphNode::new("SS", &[]),
            ]
        );

        // https://regexper.com/#%5EW%28SSS%7CEEESSSWWW%29ENNES%24
        assert_eq!(
            parse_regex(INPUT_TEST_6),
            vec![
                /* 0 */ GraphNode::new("W", &[1, 2]),
                /* 1 */ GraphNode::new("SSS", &[3]),
                /* 2 */ GraphNode::new("EEESSSWWW", &[3]),
                /* 3 */ GraphNode::new("ENNES", &[]),
            ]
        );
    }

    fn save_gv(input: &[u8], nb: usize) {
        let graph = parse_regex(input);
        let path = format!("resources/input_test_{}.gv", nb);
        let mut output = File::create(path).unwrap();
        write_graphviz(&mut output, &graph).unwrap();
    }

    #[test]
    fn write_all_graphviz() {
        save_gv(INPUT_TEST_3, 3);
        save_gv(INPUT_TEST_4, 4);
        save_gv(INPUT_TEST_5, 5);
        save_gv(INPUT_TEST_6, 6);
    }

    #[test]
    fn test_map_generation() {
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_1)).to_string().trim(),
            include_str!("../resources/input_test_1.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_2)).to_string().trim(),
            include_str!("../resources/input_test_2.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_3)).to_string().trim(),
            include_str!("../resources/input_test_3.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_4)).to_string().trim(),
            include_str!("../resources/input_test_4.map")
        );
        assert_eq!(
            build_map(&parse_regex(INPUT_TEST_5)).to_string().trim(),
            include_str!("../resources/input_test_5.map")
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(dist_to_furthest_room(INPUT_TEST_1), 3);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_2), 10);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_3), 18);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_4), 23);
        assert_eq!(dist_to_furthest_room(INPUT_TEST_5), 31);
    }
}
