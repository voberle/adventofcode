use std::io::{self, Read};

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

// Adds the charging outlet (joltage 0) to the list of adapters,
// and sorts the list.
// Sorting the list allows the graph trasversal to be quick after.
fn extend_adapter_list(adapters: &[u32]) -> Vec<u32> {
    let mut adapters = adapters.to_vec();
    adapters.push(0);
    adapters.sort_unstable();
    adapters
}

// Returns for each adapter (same index as in adapters vector) the index of next possible adapters.
fn create_graph(adapters: &[u32]) -> Vec<Vec<usize>> {
    adapters
        .iter()
        .map(|&a| {
            adapters
                .iter()
                .enumerate()
                .filter(|(_, &n)| a < n && n <= a + 3)
                .map(|(i, _)| i)
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn print_graph(adapters: &[u32], graph: &[Vec<usize>]) {
    for (i, n) in adapters.iter().enumerate() {
        println!("{}: jolt {}", i, n);
    }
    for (i, n) in graph.iter().enumerate() {
        println!("{}: {:?}", i, n);
    }
}

// Walks through the path and counts the number of 1 and 3 jolt differences.
fn count_jold_differences(adapters: &[u32], path: &[usize]) -> (u32, u32) {
    let mut d1 = 0;
    let mut d3 = 0;
    for n in path.windows(2) {
        let diff = adapters[n[1]] - adapters[n[0]];
        if diff == 1 {
            d1 += 1;
        } else if diff == 3 {
            d3 += 1;
        }
    }
    // Add the difference to the device
    d3 += 1;

    (d1, d3)
}

fn find_path(graph: &[Vec<usize>], current: usize, path_so_far: &mut Vec<usize>) -> bool {
    if path_so_far.len() == graph.len() {
        return true;
    }

    let next_nodes = &graph[current];
    for next in next_nodes {
        path_so_far.push(*next);
        if find_path(graph, *next, path_so_far) {
            return true;
        }
        path_so_far.pop();
    }
    false
}

fn jolt_differences(adapters: &[u32]) -> (u32, u32) {
    let extended_adapters = extend_adapter_list(adapters);

    // Create a graph from the list of adapters.
    let graph = create_graph(&extended_adapters);

    // Start is the charging outlet, with joltage 0, so in first position in the sorted list.
    let start = 0;

    let mut path = Vec::new();
    path.push(start);

    find_path(&graph, start, &mut path);
    // println!("Path {:?}", path);

    count_jold_differences(&extended_adapters, &path)
}

fn total_arrangements(adapters: &[u32]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let adapters = build(&input);

    let (d1, d3) = jolt_differences(&adapters);
    println!("Part 1: {}", d1 * d3);
    println!("Part 2: {}", total_arrangements(&adapters));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(jolt_differences(&build(INPUT_TEST_1)), (7, 5));
        assert_eq!(jolt_differences(&build(INPUT_TEST_2)), (22, 10));
    }

    #[test]
    fn test_part2() {
        assert_eq!(total_arrangements(&build(INPUT_TEST_1)), 8);
        assert_eq!(total_arrangements(&build(INPUT_TEST_2)), 19208);
    }
}
