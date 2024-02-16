//! This module handles the parsing of the reg ex into a graph.

use std::slice::Iter;

use crate::Direction;

// Intermediary structure representing the reg ex, to make life easier.
// It won't be the final graph.
#[derive(Debug)]
enum Elt {
    Value(Vec<Direction>, usize), // The second param is just an index counting each value.
    OpenGroup(usize),
    CloseGroup(usize),
    Pipe,
    Empty,
}

fn preprocess_regex(regex: &[u8]) -> Vec<Elt> {
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
            if let Some(Elt::Pipe) = output.last() {
                output.push(Elt::Empty);
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
#[derive(Debug)]
pub struct GraphNode {
    pub value: Vec<Option<Direction>>,
    pub next: Vec<usize>, // REPLACE WITH SET TO HANDLE DUPES
}

impl GraphNode {
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

pub fn parse_regex(regex: &[u8]) -> Vec<GraphNode> {
    let regex_elts = preprocess_regex(regex);

    // Create the node vector with all the values
    let mut nodes: Vec<GraphNode> = regex_elts
        .iter()
        .filter_map(|e| {
            if let Elt::Value(s, _) = e {
                Some(GraphNode {
                    value: s.iter().map(|x| Some(*x)).collect(),
                    next: vec![],
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
                nodes[current_idx].next.push(*idx);
                current_idx = *idx;
                for n in &last_exit_nodes {
                    nodes[*n].next.push(*idx);
                }
                last_exit_nodes.clear();
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
            Elt::Empty => {
                // Turns out this wasn't necessary
            }
        }
    }
    exit_nodes
}

// Prints a Graphviz version of the regex.
// A good way to check it is to compare it against the output from
// https://regexper.com
#[allow(dead_code)]
pub fn print_graphviz(nodes: &[GraphNode]) {
    println!("digraph {{");
    println!("\trankdir=LR;");
    println!("\tnode [shape = diamond]; \"END\";");
    println!("\tnode [shape = oval];");

    for (i, n) in nodes.iter().enumerate() {
        let name = n.dirs_to_string();
        println!("\t{} [shape = oval label = \"{}\"];", i, name);
    }

    for (i, f) in nodes.iter().enumerate() {
        if f.next.is_empty() {
            println!("\t{} -> END;", i);
        } else {
            for n in &f.next {
                println!("\t{} -> {};", i, n);
            }
        }
    }
    println!("}}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_regex() {
        let input_test_4 = include_bytes!("../resources/input_test_4");

        let regex = preprocess_regex(input_test_4);
        // println!("{:#?}", regex);
        assert_eq!(regex.len(), 22);
    }

    #[test]
    fn test_parse_regex() {
        let input_test_4 = include_bytes!("../resources/input_test_4");

        let graph = parse_regex(input_test_4);
        // for (i, n) in graph.iter().enumerate() {
        //     println!("{}: {}; next={:?}", i, n.dirs_to_string(), n.next);
        // }

        // print_graphviz(&graph);

        assert_eq!(graph.len(), 9);
    }
}
