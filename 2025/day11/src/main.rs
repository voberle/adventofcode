use std::io::{self, Read};

use fxhash::FxHashMap;

struct Rack {
    graph: Vec<Vec<usize>>,
    devices: FxHashMap<String, usize>,
}

impl Rack {
    fn build(input: &str) -> Self {
        let list: FxHashMap<String, Vec<String>> = input
            .lines()
            .map(|line| {
                let parts: Vec<_> = line.split(": ").collect();
                let conns: Vec<_> = parts[1]
                    .split_ascii_whitespace()
                    .map(std::string::ToString::to_string)
                    .collect();
                (parts[0].to_string(), conns)
            })
            .collect();
        // println!("{:#?}", list);

        let mut devices: FxHashMap<String, usize> = FxHashMap::default();
        devices.extend(
            list.iter()
                .flat_map(|(k, v)| std::iter::once(k).chain(v.iter()))
                .map(|s| (s.clone(), usize::MAX)),
        );

        for (i, val) in devices.iter_mut().enumerate() {
            *val.1 = i;
        }
        // println!("{:#?}", names_to_id);

        let mut graph: Vec<Vec<usize>> = vec![vec![]; devices.len()];
        for (dev, conns) in list {
            let i = *devices.get(&dev).unwrap();
            for conn in conns {
                let k = *devices.get(&conn).unwrap();
                graph[i].push(k);
            }
        }
        // println!("{:#?}", graph);

        Self { graph, devices }
    }

    fn get_id(&self, device: &str) -> usize {
        *self.devices.get(device).unwrap()
    }

    // Graphviz output
    #[allow(dead_code)]
    fn print_graph(&self) {
        let mut id_to_name: Vec<String> = vec![String::new(); self.devices.len()];
        for (n, i) in &self.devices {
            id_to_name[*i].clone_from(n);
        }

        println!("digraph {{");
        for (dev, conns) in self.graph.iter().enumerate() {
            let from = &id_to_name[dev];
            for conn in conns {
                let to = &id_to_name[*conn];
                println!("\t{from} -> {to}");
            }
        }
        println!("\tsvr [style=filled color=green]");
        println!("\tdac [style=filled color=yellow]");
        println!("\tfft [style=filled color=yellow]");
        println!("\tout [style=filled color=red]");

        println!("}}");
    }
}

// Depth-First Search (DFS) with backtracking.
// Recursive function.
fn find_all_paths(
    graph: &[Vec<usize>],
    current: usize,
    end: usize,
    visited: &mut Vec<bool>,
    cache: &mut Vec<u64>,
) -> u64 {
    if cache[current] != u64::MAX {
        return cache[current];
    }

    visited[current] = true;

    let mut results_count = 0;
    if current == end {
        results_count = 1;
    } else if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if !visited[*neighbor] {
                results_count += find_all_paths(graph, *neighbor, end, visited, cache);
            }
        }
    }

    // Backtrack
    visited[current] = false;

    cache[current] = results_count;
    results_count
}

fn count_all_paths(graph: &[Vec<usize>], from: usize, to: usize) -> u64 {
    let mut visited = vec![false; graph.len()];
    let mut cache = vec![u64::MAX; graph.len()];
    find_all_paths(graph, from, to, &mut visited, &mut cache)
}

fn total_paths(rack: &Rack) -> u64 {
    count_all_paths(&rack.graph, rack.get_id("you"), rack.get_id("out"))
}

fn total_paths_dac_fft(rack: &Rack) -> u64 {
    let svr = rack.get_id("svr");
    let dac = rack.get_id("dac");
    let fft = rack.get_id("fft");
    let out = rack.get_id("out");

    // With the Graphviz output, we see that all paths pass first by fft and then dac.
    // So we can count all paths from svr -> fft, fft -> dac and dac -> out and multiply the results.
    count_all_paths(&rack.graph, svr, fft)
        * count_all_paths(&rack.graph, fft, dac)
        * count_all_paths(&rack.graph, dac, out)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rack = Rack::build(&input);

    // dot -Tpdf -Kdot input.gv > input.pdf
    // rack.print_graph();

    println!("Part 1: {}", total_paths(&rack));
    println!("Part 2: {}", total_paths_dac_fft(&rack));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(total_paths(&Rack::build(INPUT_TEST_1)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(total_paths_dac_fft(&Rack::build(INPUT_TEST_2)), 2);
    }
}
