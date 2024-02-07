use std::{
    collections::VecDeque,
    io::{self, Read},
};

#[derive(Debug)]
struct Node {
    child_node_count: usize,
    metadata_count: usize,
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
            child_node_count,
            metadata_count,
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

fn part2(root: &Node) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let root = build_tree(input.trim());
    // println!("{:?}", root);

    println!("Part 1: {}", metadata_sum(&root));
    println!("Part 2: {}", part2(&root));
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
        assert_eq!(part2(&build_tree(INPUT_TEST)), 0);
    }
}
