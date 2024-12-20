use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

// Brute force version.
// The goal of this implementation is not really to find the answer,
// as it requires 13_000 billions loops to get there, but to play with optimizations.
// Check with:
//   cargo b --release
//   hyperfine --warmup 5 'cat resources/input | ./target/release/day8'

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
    // helper values to build the optimized vec
    index: usize,
    ends_with_z: bool,
}

// We hard-code the number of keys to process in parallel, as an optimization.
// As the number is small, it allows the compiler to unroll things.
const KEYS_LEN: usize = 6;

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    // -- Input parsing.
    let (instructions, nodes) = input.split("\n\n").collect_tuple().unwrap();

    let node_re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut network: HashMap<String, Node> = HashMap::new();
    for line in nodes.lines() {
        let captures = node_re.captures(line).unwrap();
        network.insert(
            captures[1].to_owned(),
            Node {
                left: captures[2].to_owned(),
                right: captures[3].to_owned(),
                index: 0,
                ends_with_z: false,
            },
        );
    }

    // -- Input structure remodelling.
    // Convert the network map into a vectors, should be much faster.
    // keys_vec is the mapping index => string. Only need to build the other vectors.
    let mut keys_vec: Vec<String> = network.keys().cloned().collect();
    // Put the keys ends with Z as first, as this allows to optimize checking if we got the Zs.
    keys_vec.sort_by(|a, b| {
        let ac = a.ends_with('Z');
        let bc = b.ends_with('Z');
        if ac && !bc {
            Ordering::Greater
        } else if !ac & bc {
            Ordering::Less
        } else {
            a.partial_cmp(b).unwrap()
        }
    });
    keys_vec.reverse();
    keys_vec.iter().enumerate().for_each(|(i, k)| {
        let n = network.get_mut(k).unwrap();
        n.index = i;
        n.ends_with_z = k.ends_with('Z');
    });
    // These are the vectors that the main loop will use.
    let mut left_nodes: Vec<usize> = vec![0; keys_vec.len()];
    let mut right_nodes: Vec<usize> = vec![0; keys_vec.len()];
    keys_vec.iter().enumerate().for_each(|(i, k)| {
        let n = network.get(k).unwrap();
        left_nodes[i] = network.get(&n.left).unwrap().index;
        right_nodes[i] = network.get(&n.right).unwrap().index;
    });
    // println!("{:#?}", network);
    // println!("{:?}", keys_vec);
    // println!("{:?}", left_nodes);
    // println!("{:?}", right_nodes);

    let mut total_steps: u64 = 0;
    // Start with all nodes that end with A
    // Using an array instead of a vector, as it's small and results in a faster loop.
    let mut keys: [usize; KEYS_LEN] = network
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, n)| n.index)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap();
    assert_eq!(KEYS_LEN, keys.len());
    let z_index_sum = (0..KEYS_LEN).sum::<usize>();
    // println!("Initial keys: {:?}", keys);
    // println!("z_index_sum: {}", z_index_sum);

    // -- The big loop that runs billions of times.
    for ins in instructions.chars().cycle() {
        for i in 0..KEYS_LEN {
            keys[i] = if ins == 'L' {
                left_nodes[keys[i]]
            } else {
                right_nodes[keys[i]]
            };
        }
        // println!("{:?}", keys);

        total_steps += 1;
        if total_steps % 100_000_000 == 0 {
            println!(".. {} M", total_steps / 1_000_000);
            break;
        }

        if sum_keys(&keys) == z_index_sum {
            break;
        }
    }

    total_steps
}

#[inline]
fn sum_keys(keys: &[usize; KEYS_LEN]) -> usize {
    // This will be optimized in the same way as a for loop
    keys.iter().sum::<usize>()
    // let mut t = 0;
    // for i in 0..KEYS_LEN {
    //     t += keys[i]
    // }
    // t
}
