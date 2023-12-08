// https://adventofcode.com/2023/day/8
// Part 2 test 3: 6
// Part 2:

use regex::Regex;
use std::{collections::HashMap, io};

fn main() {
    let stdin = io::stdin();

    let mut n = String::new();
    stdin.read_line(&mut n).unwrap();
    let instructions = n.trim();

    let node_re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        if s.is_empty() {
            continue;
        } // skip empty line
        let captures = node_re.captures(&s).unwrap();
        network.insert(
            captures[1].to_owned(),
            (captures[2].to_owned(), captures[3].to_owned()),
        );
    }

    let mut total_steps = 0;
    // Start with all nodes that end with A
    let mut keys: Vec<&String> = network.keys().filter(|n| n.ends_with('A')).collect();
    'outer: loop {
        for i in instructions.chars() {
            let nodes: Vec<&(String, String)> = network
                .iter()
                .filter(|(k, v)| keys.contains(k))
                .map(|(k, v)| v)
                .collect();
            total_steps += 1;

            keys.clear();
            keys = nodes
                .iter()
                .map(|node| if i == 'L' { &node.0 } else { &node.1 })
                .collect();
            if keys.iter().all(|n| n.ends_with('Z')) {
                break 'outer;
            }
        }
    }

    println!("Part 2: {}", total_steps);
}
