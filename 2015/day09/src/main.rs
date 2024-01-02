use std::{
    collections::HashMap,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};
use regex::Regex;

type Location = String;
type Graph = HashMap<Location, Vec<(Location, u32)>>;

fn build(input: &str) -> Graph {
    let mut graph = HashMap::new();
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    input.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let dist = caps[3].parse().unwrap();
        graph
            .entry(caps[1].to_string())
            .or_insert_with(Vec::new)
            .push((caps[2].to_string(), dist));
        graph
            .entry(caps[2].to_string())
            .or_insert_with(Vec::new)
            .push((caps[1].to_string(), dist));
    });
    graph
}

// Recursive traversal to find the shortest or longest route.
fn best_route_from(
    graph: &Graph,
    cmp: fn(u32, u32) -> bool,
    final_distances: &mut FxHashMap<Location, u32>, // has the distances once we've gone through all nodes
    route_so_far: &mut FxHashSet<Location>,
    node: &Location,
    curr_distance: u32,
) {
    if route_so_far.contains(node) {
        return;
    }
    route_so_far.insert(node.clone());

    // If we have visited all locations
    if route_so_far.len() == graph.len() {
        // if the found path is shorter, save it
        final_distances
            .entry(node.clone())
            .and_modify(|e| {
                if cmp(*e, curr_distance) {
                    *e = curr_distance
                }
            })
            .or_insert(curr_distance);
    }

    // call the method on the neighbour nodes
    graph.get(node).unwrap().iter().for_each(|(loc, dist)| {
        let length = curr_distance + dist;
        best_route_from(graph, cmp, final_distances, route_so_far, loc, length);
    });

    route_so_far.remove(node);
}

fn shortest_route(graph: &Graph) -> u32 {
    graph
        .keys()
        .map(|start| {
            let mut route_so_far = FxHashSet::default();
            let mut final_distances = FxHashMap::default();
            best_route_from(
                graph,
                |a, b| a > b,
                &mut final_distances,
                &mut route_so_far,
                start,
                0,
            );

            *final_distances.values().min().unwrap()
        })
        .min()
        .unwrap()
}

fn longest_route(graph: &Graph) -> u32 {
    graph
        .keys()
        .map(|start| {
            let mut route_so_far = FxHashSet::default();
            let mut final_distances = FxHashMap::default();
            best_route_from(
                graph,
                |a, b| a < b,
                &mut final_distances,
                &mut route_so_far,
                start,
                0,
            );

            *final_distances.values().max().unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let graph = build(&input);
    // println!("{:?}", graph);

    println!("Part 1: {}", shortest_route(&graph));
    println!("Part 2: {}", longest_route(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(shortest_route(&build(INPUT_TEST)), 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(longest_route(&build(INPUT_TEST)), 982);
    }
}
