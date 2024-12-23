use std::io::{self, Read};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

fn build(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            line.split('-')
                .map(std::borrow::ToOwned::to_owned)
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

// Returns a map with each computer to its connections.
fn make_graph(connections: &[(String, String)]) -> FxHashMap<&String, Vec<&String>> {
    let mut graph: FxHashMap<&String, Vec<&String>> = FxHashMap::default();
    for (c1, c2) in connections {
        graph
            .entry(c1)
            .and_modify(|s| s.push(c2))
            .or_insert(vec![c2]);
        graph
            .entry(c2)
            .and_modify(|s| s.push(c1))
            .or_insert(vec![c1]);
    }
    graph
}

fn set_counts_with_t_computer(connections: &[(String, String)]) -> usize {
    let graph = make_graph(connections);

    let mut lists_of_3_connected = FxHashSet::default();

    // For each computer, we go through each pair of its connections and check of they are connected.
    for (key, values) in &graph {
        for pair in values.iter().combinations(2) {
            if let Some(v) = graph.get(pair[0]) {
                if v.contains(pair[1]) {
                    let mut triplet = vec![key, pair[0], pair[1]];
                    triplet.sort();
                    lists_of_3_connected.insert(triplet);
                }
            }
        }
    }

    lists_of_3_connected
        .iter()
        .filter(|list| list.iter().any(|c| c.starts_with('t')))
        .count()
}

fn part2(connections: &[(String, String)]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let connections = build(&input);

    println!("Part 1: {}", set_counts_with_t_computer(&connections));
    println!("Part 2: {}", part2(&connections));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(set_counts_with_t_computer(&build(INPUT_TEST)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
