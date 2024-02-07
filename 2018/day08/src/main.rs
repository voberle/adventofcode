use std::{
    collections::VecDeque,
    io::{self, Read},
};

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
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
            children: child_nodes,
            metadata,
        }
    }

    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().sum()
    }
}

// Here is a neat solution with iterators
// https://www.reddit.com/r/adventofcode/comments/a47ubw/comment/ebc7orl/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
fn build_tree(input: &str) -> Node {
    // Using a VecDeque as we will pop items from front.
    let mut numbers: VecDeque<u32> = input.split(' ').map(|i| i.parse().unwrap()).collect();
    Node::new(&mut numbers)
}

fn all_metadata_sum(node: &Node) -> u32 {
    node.metadata_sum() + node.children.iter().map(all_metadata_sum).sum::<u32>()
}

fn node_value(node: &Node) -> u32 {
    if node.children.is_empty() {
        node.metadata_sum()
    } else {
        node.metadata
            .iter()
            .map(|m| {
                node.children
                    .get(*m as usize - 1)
                    .map(node_value)
                    .unwrap_or_default()
            })
            .sum::<u32>()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let root = build_tree(input.trim());
    // println!("{:?}", root);

    println!("Part 1: {}", all_metadata_sum(&root));
    println!("Part 2: {}", node_value(&root));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(all_metadata_sum(&build_tree(INPUT_TEST)), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(node_value(&build_tree(INPUT_TEST)), 66);
    }
}
