use std::io::{self, Read};

use itertools::Itertools;

#[allow(dead_code)]
struct Artifact {
    code: String,
    id: u32,
}

impl Artifact {
    fn build(line: &str) -> Self {
        let parts: Vec<_> = line.split(" | ").collect();
        Self {
            code: parts[0].to_string(),
            id: parts[1].parse().unwrap(),
        }
    }
}

fn build(input: &str) -> (Vec<Artifact>, (Artifact, Artifact)) {
    let input_parts: Vec<_> = input.split("\n\n").collect();
    let artifacts = input_parts[0].lines().map(Artifact::build).collect();
    let pair = input_parts[1]
        .lines()
        .map(Artifact::build)
        .collect_tuple()
        .unwrap();
    (artifacts, pair)
}

// Our tree is a vector of Node. Each node has a value that is the index from the artifacts vector,
// and two children that are the indexes in the tree vector.
#[derive(Debug, Clone, Copy)]
struct Node {
    artifact_idx: usize,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new(artifact_idx: usize) -> Self {
        Self {
            artifact_idx,
            left: None,
            right: None,
        }
    }
}

type Tree = Vec<Node>;

fn make_tree(artifacts: &[Artifact]) -> Tree {
    assert!(!artifacts.is_empty());

    // Initialiaze the tree with the root node.
    let mut tree: Tree = vec![Node::new(0)];

    // Go through the remaining artifacts.
    for (artifact_idx, artifact) in artifacts.iter().enumerate().skip(1) {
        // New node for this artifact.
        let node_to_insert = Node::new(artifact_idx);

        let mut current_node_idx = 0;
        loop {
            if artifact.id > artifacts[tree[current_node_idx].artifact_idx].id {
                // Right
                if let Some(right_idx) = tree[current_node_idx].right {
                    // There is a node under right side, keep searching.
                    current_node_idx = right_idx;
                } else {
                    // No right node, adding it.
                    tree[current_node_idx].right = Some(tree.len());
                    tree.push(node_to_insert);
                    break;
                }
            } else {
                // Left
                if let Some(left_idx) = tree[current_node_idx].left {
                    current_node_idx = left_idx;
                } else {
                    tree[current_node_idx].left = Some(tree.len());
                    tree.push(node_to_insert);
                    break;
                }
            }
        }
    }

    tree
}

// Put the artifact contained in the specified node at the proper layer, and recursively explores the node's branches.
fn fill_layers(tree: &Tree, node_idx: usize, layer_level: usize, layers: &mut Vec<Vec<usize>>) {
    if layers.len() <= layer_level {
        layers.push(Vec::new());
    }

    layers[layer_level].push(node_idx);

    if let Some(left_idx) = tree[node_idx].left {
        fill_layers(tree, left_idx, layer_level + 1, layers);
    }

    if let Some(right_idx) = tree[node_idx].right {
        fill_layers(tree, right_idx, layer_level + 1, layers);
    }
}

// Finds on which layers are the artifacts.
fn collect_layers(tree: &Tree) -> Vec<Vec<usize>> {
    let mut layers: Vec<Vec<usize>> = Vec::new();
    fill_layers(tree, 0, 0, &mut layers);
    layers
}

#[allow(dead_code)]
fn print_layers(artifacts: &[Artifact], layers: &[Vec<usize>]) {
    for (i, layer) in layers.iter().enumerate() {
        print!("{}: {} artifacts, IDs: ", i + 1, layer.len());
        for artifact_idx in layer {
            print!("{}, ", artifacts[*artifact_idx].id);
        }
        println!();
    }
}

fn find_largest_layer_sum(artifacts: &[Artifact], layers: &[Vec<usize>]) -> u32 {
    layers
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|artifact_idx| artifacts[*artifact_idx].id)
                .sum()
        })
        .max()
        .unwrap()
}

fn largest_layer(artifacts: &[Artifact], tree: &Tree) -> u32 {
    let layers = collect_layers(tree);
    // print_layers(artifacts, &layers);

    let occupied_layers_cnt = u32::try_from(layers.len()).unwrap();
    let largest_sum = find_largest_layer_sum(artifacts, &layers);

    occupied_layers_cnt * largest_sum
}

fn find_seq_for(artifacts: &[Artifact], tree: &Tree, id: u32) -> Vec<String> {
    let mut sequence = Vec::new();

    let mut current_node_idx = 0;
    loop {
        sequence.push(artifacts[tree[current_node_idx].artifact_idx].code.clone());

        if id > artifacts[tree[current_node_idx].artifact_idx].id {
            if let Some(right_idx) = tree[current_node_idx].right {
                // There is a node under right side, keep searching.
                current_node_idx = right_idx;
            } else {
                // No right node, done.
                break;
            }
        } else if let Some(left_idx) = tree[current_node_idx].left {
            // There is a node under left side, keep searching.
            current_node_idx = left_idx;
        } else {
            // No left node, done.
            break;
        }
    }

    sequence
}

fn seq_for_id_500000(artifacts: &[Artifact], tree: &Tree) -> String {
    const ID_TO_INSERT: u32 = 500_000;

    let sequence = find_seq_for(artifacts, tree, ID_TO_INSERT);
    sequence.join("-")
}

fn least_common_ancestor(
    artifacts: &[Artifact],
    tree: &Tree,
    extra_pair: &(Artifact, Artifact),
) -> String {
    let seq1 = find_seq_for(artifacts, tree, extra_pair.0.id);
    let seq2 = find_seq_for(artifacts, tree, extra_pair.1.id);

    // Find the first code common in both sequences, starting from the end.
    for c1 in seq1.iter().rev() {
        for c2 in seq2.iter().rev() {
            if c1 == c2 {
                return c1.to_string();
            }
        }
    }
    panic!("No common ancestor found");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (artifacts, extra_pair) = build(&input);

    let tree = make_tree(&artifacts);

    println!("Part 1: {}", largest_layer(&artifacts, &tree));
    println!("Part 2: {}", seq_for_id_500000(&artifacts, &tree));
    println!(
        "Part 3: {}",
        least_common_ancestor(&artifacts, &tree, &extra_pair)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (artifacts, _) = build(&INPUT_TEST);

        let tree = make_tree(&artifacts);
        assert_eq!(largest_layer(&artifacts, &tree), 12645822);
    }

    #[test]
    fn test_seq_for_id_500000() {
        let (artifacts, _) = build(&INPUT_TEST);

        let tree = make_tree(&artifacts);
        assert_eq!(
            seq_for_id_500000(&artifacts, &tree),
            "ozNxANO-pYNonIG-MUantNm-lOSlxki-SDJtdpa-JSXfNAJ"
        );
    }

    #[test]
    fn test_least_common_ancestor() {
        let (artifacts, extra_pair) = build(&INPUT_TEST);

        let tree = make_tree(&artifacts);
        assert_eq!(
            least_common_ancestor(&artifacts, &tree, &extra_pair),
            "pYNonIG"
        );
    }
}
