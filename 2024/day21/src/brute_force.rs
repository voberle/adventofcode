//! Brute-force implementation generating a list of all directions.
//! Works for 2 robots, but not for 25...

use crate::{convert_num_paths_to_directions, find_code_paths, model::DirKey, prepend};

use itertools::Itertools;

// Finds the short path(s) to reach all the direction values.
// The code needs to include the starting position.
fn find_dir_paths(directions: &[DirKey]) -> Vec<Vec<DirKey>> {
    let mut paths: Vec<Vec<DirKey>> = vec![vec![]];
    for pair in directions.windows(2) {
        let paths_for_pair = pair[0].go_press(pair[1]);
        assert!(!paths_for_pair.is_empty());

        paths = paths_for_pair
            .iter()
            .flat_map(|path| {
                paths.iter().map(move |base_path| {
                    let mut p = base_path.clone();
                    p.extend(path);
                    p
                })
            })
            .collect();
    }
    paths
}

fn next_sequence(paths_as_dirs: &[Vec<DirKey>]) -> Vec<Vec<DirKey>> {
    paths_as_dirs
        .iter()
        .flat_map(|dirs| find_dir_paths(&prepend(dirs, DirKey::A)))
        .collect()
}

#[allow(dead_code)]
pub fn shortest_sequence_length(code: &[char], robots_count: usize) -> usize {
    let paths = find_code_paths(&prepend(code, 'A'));
    let mut next_paths = paths
        .iter()
        .map(|p| convert_num_paths_to_directions(p))
        .collect_vec();

    for _ in 0..robots_count {
        next_paths = next_sequence(&next_paths);
    }

    next_paths.iter().map(std::vec::Vec::len).min().unwrap()
}
