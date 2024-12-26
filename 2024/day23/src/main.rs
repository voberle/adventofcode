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

struct Graph {
    indexes: FxHashMap<String, usize>,
    reverse_indexes: Vec<String>,
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
            indexes,
            reverse_indexes,
            matrix,
        }
    }
}

// Returns the list of 3 computers all connected to each other.
// fn build_list_of_3<'a>(
//     graph: &'a FxHashMap<&'a String, Vec<&'a String>>,
// ) -> FxHashSet<Vec<&'a &'a String>> {
fn build_list_of_3(graph: &Graph) -> FxHashSet<Vec<usize>> {
    let mut lists_of_3_connected = FxHashSet::default();

    // For each computer, we go through each pair of its connections and check of they are connected.
    for computer_index in 0..graph.matrix.len() {
        for pair in graph.matrix[computer_index]
            .iter()
            .enumerate()
            .filter_map(|(i, is_set)| if *is_set { Some(i) } else { None })
            .combinations(2)
        {
            if graph.matrix[pair[0]][pair[1]] {
                let mut triplet = vec![computer_index, pair[0], pair[1]];
                triplet.sort();
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

    // Finding groups of 4:
    // Take a group of 3
    // Take a 4th computer and check if connected to all 3.
    // If yes, we have a group.

    let mut next_groups = FxHashSet::default();
    loop {
        let computers = groups.iter().flatten().collect_vec();
        for group in &groups {
            for to_check in &computers {
                if group.contains(to_check) {
                    continue;
                }
                if group.iter().all(|gr_index| {
                    graph.matrix[**to_check][*gr_index]
                }) {
                    let mut g = group.clone();
                    g.push(**to_check);
                    g.sort();
                    next_groups.insert(g);
                }
            }
        }

        for g in &next_groups {
            println!("{:?}", g);
        }

        std::mem::swap(&mut groups, &mut next_groups);

        if groups.len() == 1 {
            break;
        }

        next_groups.clear();
    }

    let answer_indexes = groups.iter().take(1).next().unwrap();
    println!("{:?}", answer_indexes);

    indexes_to_string(&graph, &answer_indexes)
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
