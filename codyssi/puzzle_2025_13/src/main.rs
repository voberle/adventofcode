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

    let mut id2loc: HashMap<String, usize> = HashMap::default();
    for (index, loc) in locations_vec.iter().enumerate() {
        id2loc.insert(loc.clone(), index);
    }

    let mut paths_vec = vec![vec![0; locations_vec.len()]; locations_vec.len()];
    for path in paths {
        let from_id = *id2loc.get(&path.from).unwrap();
        let to_id = *id2loc.get(&path.to).unwrap();
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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let paths = build(&input);

    println!("Part 1: {}", three_longest_paths(&paths, true));
    println!("Part 2: {}", three_longest_paths(&paths, false));
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
}
