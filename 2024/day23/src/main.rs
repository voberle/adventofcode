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
    computer_count: usize,
    // For an index, finds the computer name.
    reverse_indexes: Vec<String>,
    // Given two indexes, we can check if the computers are connected.
    // The simple vector represents a grid.
    matrix: Vec<bool>,
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

        let computer_count = indexes.len();

        let mut matrix = vec![false; computer_count * computer_count];
        for conn in connections {
            let i1 = *indexes.get(&conn.0).unwrap();
            let i2 = *indexes.get(&conn.1).unwrap();
            matrix[i1 * computer_count + i2] = true;
            matrix[i2 * computer_count + i1] = true;
        }

        Self {
            computer_count,
            reverse_indexes,
            matrix,
        }
    }

    fn is_connected(&self, i: usize, j: usize) -> bool {
        self.matrix[i * self.computer_count + j]
    }

    fn get_line(&self, row: usize) -> &[bool] {
        &self.matrix[row * self.computer_count..(row + 1) * self.computer_count]
    }
}

// Returns the list of 3 computers all connected to each other.
fn build_list_of_3(graph: &Graph) -> FxHashSet<Vec<usize>> {
    let mut lists_of_3_connected = FxHashSet::default();

    // For each computer, we go through each pair of its connections and check of they are connected.
    for computer_index in 0..graph.computer_count {
        for pair in graph
            .get_line(computer_index)
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

// Part 1
fn set_counts_with_t_computer(
    graph: &Graph,
    lists_of_3_connected: &FxHashSet<Vec<usize>>,
) -> usize {
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

// Part 2
fn lan_party_password(graph: &Graph, lists_of_3_connected: &FxHashSet<Vec<usize>>) -> String {
    let mut groups = lists_of_3_connected.clone();

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

        std::mem::swap(&mut groups, &mut next_groups);

        if groups.len() == 1 {
            break;
        }

        next_groups.clear();
    }

    let answer_indexes = groups.iter().take(1).next().unwrap();
    indexes_to_string(graph, answer_indexes)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let connections = build(&input);
    let graph = Graph::new(&connections);
    let lists_of_3_connected = build_list_of_3(&graph);

    println!(
        "Part 1: {}",
        set_counts_with_t_computer(&graph, &lists_of_3_connected)
    );
    println!(
        "Part 2: {}",
        lan_party_password(&graph, &lists_of_3_connected)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let connections = build(INPUT_TEST);
        let graph = Graph::new(&connections);
        let lists_of_3_connected = build_list_of_3(&graph);
        assert_eq!(set_counts_with_t_computer(&graph, &lists_of_3_connected), 7);
    }

    #[test]
    fn test_part2() {
        let connections = build(INPUT_TEST);
        let graph = Graph::new(&connections);
        let lists_of_3_connected = build_list_of_3(&graph);
        assert_eq!(
            lan_party_password(&graph, &lists_of_3_connected),
            "co,de,ka,ta"
        );
    }
}
