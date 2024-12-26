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

// Fast implementation of the graph.
struct Graph {
    // For an index, finds the computer name.
    reverse_indexes: Vec<String>,
    // Given two indexes, we can check if the computers are connected.
    matrix: Vec<Vec<bool>>,
}

impl Graph {
    fn new(connections: &[(String, String)]) -> Self {
        let mut indexes = FxHashMap::default();
        for conn in connections {
            let cnt = indexes.len();
            indexes.entry(conn.0.clone()).or_insert(cnt);
            let cnt = indexes.len();
            indexes.entry(conn.1.clone()).or_insert(cnt);
        }

        let reverse_indexes = indexes
            .iter()
            .sorted_unstable_by_key(|(_, index)| **index)
            .map(|(name, _)| name.clone())
            .collect();

        let mut matrix = vec![vec![false; indexes.len()]; indexes.len()];
        for conn in connections {
            let i1 = *indexes.get(&conn.0).unwrap();
            let i2 = *indexes.get(&conn.1).unwrap();
            matrix[i1][i2] = true;
            matrix[i2][i1] = true;
        }

        Self {
            reverse_indexes,
            matrix,
        }
    }

    fn computer_count(&self) -> usize {
        self.matrix.len()
    }

    fn is_connected(&self, i: usize, j: usize) -> bool {
        self.matrix[i][j]
    }
}

// Returns the list of 3 computers all connected to each other.
fn build_list_of_3(graph: &Graph) -> FxHashSet<Vec<usize>> {
    let mut lists_of_3_connected = FxHashSet::default();

    // For each computer, we go through each pair of its connections and check of they are connected.
    for computer_index in 0..graph.computer_count() {
        for pair in graph.matrix[computer_index]
            .iter()
            .enumerate()
            .filter_map(|(i, is_set)| if *is_set { Some(i) } else { None })
            .combinations(2)
        {
            if graph.is_connected(pair[0], pair[1]) {
                let mut triplet = vec![computer_index, pair[0], pair[1]];
                triplet.sort_unstable();
                lists_of_3_connected.insert(triplet);
            }
        }
    }
    lists_of_3_connected
}

fn set_counts_with_t_computer(connections: &[(String, String)]) -> usize {
    let graph = Graph::new(connections);

    let lists_of_3_connected = build_list_of_3(&graph);

    lists_of_3_connected
        .iter()
        .filter(|list| {
            list.iter()
                .any(|&c| graph.reverse_indexes[c].starts_with('t'))
        })
        .count()
}

fn indexes_to_string(graph: &Graph, indexes: &[usize]) -> String {
    indexes
        .iter()
        .map(|&i| graph.reverse_indexes[i].clone())
        .sorted()
        .join(",")
}

fn lan_party_password(connections: &[(String, String)]) -> String {
    let graph = Graph::new(connections);

    let mut groups = build_list_of_3(&graph);

    // Finding groups of n + 1:
    // For each group of n, take a computer not part of the group, and check if connected to all of the group.
    // If yes, we have a group.
    let computers = groups.iter().flatten().copied().collect_vec();

    let mut next_groups = FxHashSet::default();
    loop {
        for group in &groups {
            for to_check in &computers {
                if group.iter().all(|gr_index| {
                    if to_check == gr_index {
                        false
                    } else {
                        graph.is_connected(*to_check, *gr_index)
                    }
                }) {
                    let mut g = group.clone();
                    g.push(*to_check);
                    g.sort_unstable();
                    next_groups.insert(g);
                }
            }
        }

        // println!("{} groups", next_groups.len());
        // for g in &next_groups {
        //     println!("Groups size: {}", g.len());
        //     break;
        // }

        std::mem::swap(&mut groups, &mut next_groups);

        if groups.len() == 1 {
            break;
        }

        next_groups.clear();
    }

    let answer_indexes = groups.iter().take(1).next().unwrap();
    // println!("{:?}", answer_indexes);

    indexes_to_string(&graph, answer_indexes)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let connections = build(&input);

    println!("Part 1: {}", set_counts_with_t_computer(&connections));
    println!("Part 2: {}", lan_party_password(&connections));
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
        assert_eq!(lan_party_password(&build(INPUT_TEST)), "co,de,ka,ta");
    }
}
