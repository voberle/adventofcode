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
    // println!("{} computers", graph.len());
    graph
}

// Returns the list of 3 computers all connected to each other.
fn build_list_of_3<'a>(
    graph: &'a FxHashMap<&'a String, Vec<&'a String>>,
) -> FxHashSet<Vec<&'a &'a String>> {
    let mut lists_of_3_connected = FxHashSet::default();

    // For each computer, we go through each pair of its connections and check of they are connected.
    for (key, values) in graph {
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
}

fn set_counts_with_t_computer(connections: &[(String, String)]) -> usize {
    let graph = make_graph(connections);

    let lists_of_3_connected = build_list_of_3(&graph);

    lists_of_3_connected
        .iter()
        .filter(|list| list.iter().any(|c| c.starts_with('t')))
        .count()
}

fn lan_party_password(connections: &[(String, String)]) -> String {
    let graph = make_graph(connections);

    let mut groups = build_list_of_3(&graph);

    // Finding groups of 4:
    // Take a group of 3
    // For all other computers:
    //     Add a 4th. Gen the combi of 3, and check if the 4 combinations are in the set.
    //     If yes, we have a group.

    let mut next_groups = FxHashSet::default();
    loop {

        let computers = groups.iter().flatten().collect_vec();
        for group in &groups {
            for d in &computers {
                if group.contains(d) {
                    continue;
                }
                let mut g = group.clone();
                g.push(d);

                if g.iter()
                    .cloned()
                    .combinations(group.len())
                    .all(|combi| groups.contains(&combi))
                {
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

    println!("{:?}", groups.iter().take(1).next().unwrap());

    "".to_string()
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
