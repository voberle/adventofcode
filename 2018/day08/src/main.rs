use std::{
    collections::VecDeque,
    io::{self, Read},
};

#[derive(Debug)]
struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(numbers: &mut VecDeque<u32>) -> Self {
        let child_node_count = numbers.pop_front().unwrap() as usize;
        let metadata_count = numbers.pop_front().unwrap() as usize;
        let child_nodes = (0..child_node_count).map(|_| Node::new(numbers)).collect();
        let metadata = (0..metadata_count)
            .map(|_| numbers.pop_front().unwrap())
            .collect();
        Self {
            child_nodes,
            metadata,
        }
    }
}

fn build_tree(input: &str) -> Node {
    // Using a VecDeque as we will pop items from front.
    let mut numbers: VecDeque<u32> = input.split(' ').map(|i| i.parse().unwrap()).collect();
    Node::new(&mut numbers)
}

fn metadata_sum(node: &Node) -> u32 {
    node.metadata.iter().sum::<u32>() + node.child_nodes.iter().map(metadata_sum).sum::<u32>()
}

fn node_value(node: &Node) -> u32 {
    if node.child_nodes.is_empty() {
        node.metadata.iter().sum::<u32>()
    } else {
        node.metadata
            .iter()
            .map(|m| {
                let m_idx = (*m - 1) as usize;
                if m_idx < node.child_nodes.len() {
                    node_value(&node.child_nodes[m_idx])
                } else {
                    0
                }
            })
            .sum::<u32>()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let root = build_tree(input.trim());
    // println!("{:?}", root);

    println!("Part 1: {}", metadata_sum(&root));
    println!("Part 2: {}", node_value(&root));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(metadata_sum(&build_tree(INPUT_TEST)), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(node_value(&build_tree(INPUT_TEST)), 66);
    }
}
