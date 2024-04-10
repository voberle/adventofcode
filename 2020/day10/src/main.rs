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

fn jolt_differences(adapters: &[u32], graph: &[Vec<usize>]) -> (u32, u32) {
    // Start is the charging outlet, with joltage 0, so in first position in the sorted list.
    let start = 0;

    let mut path = Vec::new();
    path.push(start);

    find_path(graph, start, &mut path);

    count_jold_differences(adapters, &path)
}

fn total_arrangements(_adapters: &[u32], graph: &[Vec<usize>]) -> usize {
    // We detect which patters are in the graph, each having a specific number of options.
    // Then we just need to multiply the options count for each pattern.
    //
    // There are 3 types of patterns:
    //
    // [7, 8]
    // [8]
    // => 2 options
    //
    // [3, 4, 5]
    // [4, 5]
    // [5]
    // => 4 options
    //
    // [1, 2, 3]
    // [2, 3, 4]
    // [3, 4]
    // [4]
    // => 4 + 3 = 7 options

    let options_counts: Vec<usize> = graph.iter().map(Vec::len).collect();
    let mut arrangements = 1;

    let mut i = 0;
    while i < options_counts.len() - 1 {
        // skip last one
        let options = options_counts[i];
        if options == 2 {
            arrangements *= 2;
            assert_eq!(options_counts[i + 1], 1);
            assert_eq!(options_counts[i + 2], 1);
        }
        if options == 3 {
            if options_counts[i + 1] == 2 {
                arrangements *= 4;
                assert_eq!(options_counts[i + 2], 1);
                i += 1;
            }
            if options_counts[i + 1] == 3 {
                arrangements *= 7;
                assert_eq!(options_counts[i + 2], 2);
                assert_eq!(options_counts[i + 3], 1);
                i += 2;
            }
        }

        i += 1;
    }

    arrangements
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let adapters = build(&input);

    // Extend the adapter list with the charging outlet.
    let extended_adapters = extend_adapter_list(&adapters);
    // Create a graph from the list of adapters.
    let graph = create_graph(&extended_adapters);
    // print_graph(&extended_adapters, &graph);

    let (d1, d3) = jolt_differences(&extended_adapters, &graph);
    println!("Part 1: {}", d1 * d3);
    println!("Part 2: {}", total_arrangements(&extended_adapters, &graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1_1() {
        let adapters = extend_adapter_list(&build(INPUT_TEST_1));
        let graph = create_graph(&adapters);
        assert_eq!(jolt_differences(&adapters, &graph), (7, 5));
    }

    #[test]
    fn test_part1_2() {
        let adapters = extend_adapter_list(&build(INPUT_TEST_2));
        let graph = create_graph(&adapters);
        assert_eq!(jolt_differences(&adapters, &graph), (22, 10));
    }

    #[test]
    fn test_part2_1() {
        let adapters = extend_adapter_list(&build(INPUT_TEST_1));
        let graph = create_graph(&adapters);
        assert_eq!(total_arrangements(&adapters, &graph), 8);
    }

    #[test]
    fn test_part2_2() {
        let adapters = extend_adapter_list(&build(INPUT_TEST_2));
        let graph = create_graph(&adapters);
        assert_eq!(total_arrangements(&adapters, &graph), 19208);
    }
}
