//! This module handles the parsing of the reg ex into a graph.

use std::{
    io::{Error, Write},
    slice::Iter,
};

use fxhash::FxHashSet;

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
#[derive(Debug, PartialEq)]
pub struct GraphNode {
    pub value: Vec<Option<Direction>>,
    pub next: FxHashSet<usize>,
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

pub fn parse_regex(regex: &[u8]) -> Vec<GraphNode> {
    let regex_elts = preprocess_regex(regex);

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
                nodes[current_idx].next.insert(*idx);
                current_idx = *idx;
                for n in &last_exit_nodes {
                    nodes[*n].next.insert(*idx);
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
// View with:
//  dot -Tpdf resources/input.gv > input.pdf
//
// A good way to check it is to compare it against the output from
// https://regexper.com
#[allow(dead_code)]
pub fn write_graphviz<W>(out: &mut W, nodes: &[GraphNode]) -> Result<(), Error>
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_preprocess_regex() {
        let regex = preprocess_regex(crate::tests::INPUT_TEST_4);
        // println!("{:#?}", regex);
        assert_eq!(regex.len(), 22);
    }

    #[test]
    fn test_parse_regex() {
        let graph = parse_regex(crate::tests::INPUT_TEST_5);
        for (i, n) in graph.iter().enumerate() {
            println!("{}: {}; next={:?}", i, n.dirs_to_string(), n.next);
        }

        // https://regexper.com/#%5EENNWSWW%28NEWS%7C%29SSSEEN%28WNSE%7C%29EE%28SWEN%7C%29NNN%24
        assert_eq!(
            parse_regex(crate::tests::INPUT_TEST_3),
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
            parse_regex(crate::tests::INPUT_TEST_4),
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
            parse_regex(crate::tests::INPUT_TEST_5),
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
    }

    fn save_gv(input: &[u8], nb: usize) {
        let graph = parse_regex(input);
        let path = format!("resources/input_test_{}.gv", nb);
        let mut output = File::create(path).unwrap();
        write_graphviz(&mut output, &graph).unwrap();
    }

    #[test]
    fn write_all_graphviz() {
        save_gv(crate::tests::INPUT_TEST_3, 3);
        save_gv(crate::tests::INPUT_TEST_4, 4);
        save_gv(crate::tests::INPUT_TEST_5, 5);
    }
}
