// https://adventofcode.com/2023/day/8
// Part 2 test 3: 6
// Part 2:

use regex::Regex;
use std::{collections::HashMap, io};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
    // helper values to build the optimized vec
    index: usize,
    ends_with_z: bool,
}

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();
    stdin.read_line(&mut n).unwrap();
    let instructions = n.trim();

    let node_re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut network: HashMap<String, Node> = HashMap::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        if s.is_empty() {
            continue;
        } // skip empty line
        let captures = node_re.captures(&s).unwrap();
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

    // Convert the network map into a vectors, should be much faster.
    // keys_vec is the mapping index => string. Only need to build the other vectors.
    let keys_vec: Vec<String> = network.keys().cloned().collect();
    keys_vec.iter().enumerate().for_each(|(i, k)| {
        let n = network.get_mut(k).unwrap();
        n.index = i;
        n.ends_with_z = k.ends_with('Z');
    });
    // These are the 3 vectors that the main loop will use.
    let mut left_nodes: Vec<usize> = vec![0; keys_vec.len()];
    let mut right_nodes: Vec<usize> = vec![0; keys_vec.len()];
    let mut ends_with_z: Vec<bool> = vec![false; keys_vec.len()];
    keys_vec.iter().enumerate().for_each(|(i, k)| {
        let n = network.get(k).unwrap();
        left_nodes[i] = network.get(&n.left).unwrap().index;
        right_nodes[i] = network.get(&n.right).unwrap().index;
        ends_with_z[i] = n.ends_with_z;
    });
    // println!("{:#?}", network);
    // println!("{:?}", keys_vec);
    // println!("{:?}", left_nodes);
    // println!("{:?}", right_nodes);
    // println!("{:?}", ends_with_z);


    let mut total_steps: u64 = 0;
    // Start with all nodes that end with A
    let mut keys: Vec<usize> = network.iter().filter(|(k, _)| k.ends_with('A')).map(|(_, n)| n.index).collect();
    // println!("Initial keys: {:?}", keys);

    for ins in instructions.chars().cycle() {
        let mut got_it = true;
        for i in 0..keys.len() {
            keys[i] = if ins == 'L' { left_nodes[keys[i]] } else { right_nodes[keys[i]] };
            got_it &= ends_with_z[keys[i]];
        }
        // println!("{:?}", keys);
        // println!("{:?}", got_it);

        total_steps += 1;
        if total_steps % 100_000_000 == 0 {
            println!(".. {} M", total_steps / 1_000_000);
            // break;
        }

        if got_it {
            break;
        }
    }

    println!("Part 2: {}", total_steps);
}
