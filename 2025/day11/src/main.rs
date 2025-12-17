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
}

// Depth-First Search (DFS) with backtracking.
fn find_all_paths(
    graph: &[Vec<usize>],
    filter: &[usize],
    current: usize,
    end: usize,
    visited: &mut Vec<bool>,
    results_count: &mut usize,
) {
    visited[current] = true;

    if current == end {
        if filter.iter().all(|i| visited[*i]) {
            *results_count += 1;
        }
    } else if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if !visited[*neighbor] {
                find_all_paths(graph, filter, *neighbor, end, visited, results_count);
            }
        }
    }

    // Backtrack
    visited[current] = false;
}

fn count_all_paths(graph: &[Vec<usize>], filter: &[usize], from: usize, to: usize) -> usize {
    let mut visited = vec![false; graph.len()];
    let mut results_count = 0;
    find_all_paths(graph, filter, from, to, &mut visited, &mut results_count);

    results_count
}

fn total_paths(rack: &Rack) -> usize {
    count_all_paths(&rack.graph, &[], rack.get_id("you"), rack.get_id("out"))
}

fn total_paths_dac_fft(rack: &Rack) -> usize {
    let dac = rack.get_id("dac");
    let fft = rack.get_id("fft");
    count_all_paths(
        &rack.graph,
        &[dac, fft],
        rack.get_id("svr"),
        rack.get_id("out"),
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = Rack::build(&input);

    println!("Part 1: {}", total_paths(&input_parsed));
    println!("Part 2: {}", total_paths_dac_fft(&input_parsed));
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
