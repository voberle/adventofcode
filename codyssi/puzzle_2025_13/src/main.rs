use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

struct Path {
    from: String,
    to: String,
    len: u32,
}

impl Path {
    fn build(line: &str) -> Self {
        let parts: Vec<_> = line.split(" | ").collect();
        let from_to: Vec<_> = parts[0].split(" -> ").collect();
        Self {
            from: from_to[0].to_string(),
            to: from_to[1].to_string(),
            len: parts[1].parse().unwrap(),
        }
    }
}

fn build(input: &str) -> Vec<Path> {
    input.lines().map(Path::build).collect()
}

// Convert the list of paths to a vector where locations are indexes,
// as this is easier to manipulate in an algorithm.
fn convert_paths(paths: &[Path], ignore_len: bool) -> (Vec<String>, Vec<Vec<u32>>) {
    // First we need to find the list of distinct locations.
    let mut locations_set: HashSet<String> = HashSet::default();
    for path in paths {
        locations_set.insert(path.from.clone());
        locations_set.insert(path.to.clone());
    }
    let locations_vec: Vec<String> = locations_set.iter().cloned().collect();

    // Helper map of location names to location IDs.
    let mut loc_to_id: HashMap<String, usize> = HashMap::default();
    for (index, loc) in locations_vec.iter().enumerate() {
        loc_to_id.insert(loc.clone(), index);
    }

    let mut paths_vec = vec![vec![0; locations_vec.len()]; locations_vec.len()];
    for path in paths {
        let from_id = *loc_to_id.get(&path.from).unwrap();
        let to_id = *loc_to_id.get(&path.to).unwrap();
        paths_vec[from_id][to_id] = if ignore_len { 1 } else { path.len };
    }

    (locations_vec, paths_vec)
}

fn three_longest_paths(paths: &[Path], ignore_len: bool) -> u32 {
    let (locations_vec, mut paths_vec) = convert_paths(paths, ignore_len);

    loop {
        let mut something_modified = false;

        for from_index in 0..paths_vec.len() {
            for to_index in 0..paths_vec.len() {
                if paths_vec[from_index][to_index] != 0 {
                    // For each path with a known distance, see if this path can be extended
                    // by checking in the table for paths starting with the end of this path.
                    for i in 0..paths_vec.len() {
                        if paths_vec[to_index][i] != 0 {
                            // This is a path that start with the end of the path, and that has a distance.

                            // Calculate if going via this path would be shorter.
                            let new_len = paths_vec[from_index][to_index] + paths_vec[to_index][i];
                            if paths_vec[from_index][i] == 0 || new_len < paths_vec[from_index][i] {
                                paths_vec[from_index][i] = new_len;
                                something_modified = true;
                            }
                        }
                    }
                }
            }
        }

        if !something_modified {
            break;
        }
    }

    let stt_index = locations_vec.iter().position(|l| l == "STT").unwrap();

    let mut stt_paths: Vec<u32> = paths_vec[stt_index].clone();
    stt_paths.sort_unstable();

    stt_paths[stt_paths.len() - 3..].iter().product()
}

// Recursive.
fn explore(
    paths_vec: &Vec<Vec<u32>>,
    from: usize,
    last: usize,
    visited: &[bool],
    current_dist: u32,
    best_dist: &mut u32,
) {
    for next_loc in paths_vec[last]
        .iter()
        .enumerate()
        .filter(|(_, d)| **d != 0)
        .map(|(i, _)| i)
    {
        if next_loc == from {
            // Cycle
            let new_dist = current_dist + paths_vec[last][next_loc];
            if new_dist > *best_dist {
                *best_dist = new_dist;
            }
        }

        if !visited[next_loc] {
            // Not yet visited, so explore it.
            let mut new_visited = visited.to_vec();
            new_visited[next_loc] = true;

            let new_dist = current_dist + paths_vec[last][next_loc];

            explore(paths_vec, from, next_loc, &new_visited, new_dist, best_dist);
        }
    }
}

fn longest_cycle_length(paths: &[Path]) -> u32 {
    let (_, paths_vec) = convert_paths(paths, false);

    let mut best_dist = 0;
    for from_index in 0..paths_vec.len() {
        for to_index in 0..paths_vec.len() {
            if paths_vec[from_index][to_index] != 0 {
                let mut visited = vec![false; paths_vec.len()];
                visited[from_index] = true;
                visited[to_index] = true;
                let current_dist = paths_vec[from_index][to_index];

                explore(
                    &paths_vec,
                    from_index,
                    to_index,
                    &visited,
                    current_dist,
                    &mut best_dist,
                );
            }
        }
    }

    best_dist
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let paths = build(&input);

    println!("Part 1: {}", three_longest_paths(&paths, true));
    println!("Part 2: {}", three_longest_paths(&paths, false));
    println!("Part 3: {}", longest_cycle_length(&paths));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let paths = build(&INPUT_TEST);
        assert_eq!(three_longest_paths(&paths, true), 36);
    }

    #[test]
    fn test_part2() {
        let paths = build(&INPUT_TEST);
        assert_eq!(three_longest_paths(&paths, false), 44720);
    }

    #[test]
    fn test_part3() {
        let paths = build(&INPUT_TEST);
        assert_eq!(longest_cycle_length(&paths), 18);
    }
}
