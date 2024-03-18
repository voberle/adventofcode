use fxhash::{FxHashMap, FxHashSet};
use std::io::{self, Read};

mod tunnels;

use tunnels::Map;

// Recursive function.
fn find_keys(
    map: &Map,
    robots_positions: &[usize],
    keys_positions: &[usize],
    doors_positions: &[usize],
    distance_so_far: usize,
    shortest_distance: &mut usize,
    cache: &mut FxHashSet<(usize, Vec<usize>, Vec<usize>)>,
    path_cache: &mut FxHashMap<(usize, usize, Vec<usize>), Option<usize>>,
) {
    // robots_positions and keys_positions remain sorted.
    if !cache.insert((
        distance_so_far,
        robots_positions.to_vec(),
        keys_positions.to_vec(),
    )) {
        return;
    }

    // Robot position, Key position, Distance
    let mut reachable_keys: Vec<(usize, usize, usize)> = Vec::new();
    for robot_pos in robots_positions {
        let from = *robot_pos;
        reachable_keys.extend(keys_positions.iter().filter_map(|k_pos| {
            let pos = *k_pos;
            // Cache to the shortest path function results.
            if let Some(opt_d) = path_cache.get(&(from, pos, doors_positions.to_vec())) {
                opt_d.map(|d| (from, pos, d))
            } else if let Some(dist_to_key) =
                tunnels::find_shortest_path(map, doors_positions, from, pos)
            {
                path_cache.insert((from, pos, doors_positions.to_vec()), Some(dist_to_key));
                Some((from, pos, dist_to_key))
            } else {
                path_cache.insert((from, pos, doors_positions.to_vec()), None);
                None
            }
        }));
    }

    // Sort by distance, to explore the closest ones first.
    reachable_keys.sort_unstable_by_key(|e| e.2);

    for (robot_pos, key_pos, dist_to_key) in reachable_keys {
        let new_dist = distance_so_far + dist_to_key;

        if new_dist >= *shortest_distance {
            // We have better already.
            continue;
        }

        if keys_positions.len() == 1 {
            // This is the last key we needed to find.
            *shortest_distance = new_dist.min(*shortest_distance);
            println!(
                "Last key, dist to it {}, dist {}, shortest {}",
                dist_to_key, new_dist, shortest_distance
            );
            continue;
        }

        let new_robot_position: Vec<usize> = robots_positions
            .iter()
            .map(|&p| if p == robot_pos { key_pos } else { p })
            .collect();

        let new_keys_positions: Vec<usize> = keys_positions
            .iter()
            .filter(|&&p| p != key_pos)
            .copied()
            .collect();

        let new_doors_positions: Vec<usize> =
            if let Some(door_pos) = map.get_door_position_for_key(key_pos) {
                // Some keys don't have a corresponding door.
                doors_positions
                    .iter()
                    .filter(|&&p| p != door_pos)
                    .copied()
                    .collect()
            } else {
                // we could maybe save this clone
                doors_positions.to_vec()
            };

        find_keys(
            map,
            &new_robot_position,
            &new_keys_positions,
            &new_doors_positions,
            new_dist,
            shortest_distance,
            cache,
            path_cache,
        );
    }
}

fn shortest_path_all_keys(map: &Map) -> usize {
    // Find which keys are reachable and their distance.
    // Recursively:
    // - Unlock each.
    // - Start over: Find keys etc
    // Stop when we have key count.

    let mut shortest_distance = usize::MAX;

    let entrance_positions = map.get_entrance_positions();
    let keys_positions = map.get_keys_positions();
    let doors_positions = map.get_doors_positions();
    // println!("Keys {}; Doors {}", keys_positions.len(), doors_positions.len());

    // Cache of "distance so far" + "from where we are searching" + "positions of remaining keys to find".
    let mut cache: FxHashSet<(usize, Vec<usize>, Vec<usize>)> = FxHashSet::default();
    // Cache for Dijkstra shortest path function.
    let mut path_cache: FxHashMap<(usize, usize, Vec<usize>), Option<usize>> = FxHashMap::default();

    find_keys(
        map,
        &entrance_positions,
        &keys_positions,
        &doors_positions,
        0,
        &mut shortest_distance,
        &mut cache,
        &mut path_cache,
    );

    shortest_distance
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Map::build(&input);
    // map.print();

    println!("Part 1: {}", shortest_path_all_keys(&map));

    let updated_map = map.update_map();
    // updated_map.print();
    println!("Part 2: {}", shortest_path_all_keys(&updated_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");
    const INPUT_TEST_3: &str = include_str!("../resources/input_test_3");
    const INPUT_TEST_4: &str = include_str!("../resources/input_test_4");
    const INPUT_TEST_5: &str = include_str!("../resources/input_test_5");

    #[test]
    fn test_part1() {
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_1)), 8);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_2)), 86);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_3)), 132);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_4)), 136);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_5)), 81);
    }

    const INPUT_TEST_6: &str = include_str!("../resources/input_test_6");
    const INPUT_TEST_7: &str = include_str!("../resources/input_test_7");
    const INPUT_TEST_8: &str = include_str!("../resources/input_test_8");
    const INPUT_TEST_9: &str = include_str!("../resources/input_test_9");

    #[test]
    fn test_part2() {
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_6)), 8);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_7)), 24);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_8)), 32);
        assert_eq!(shortest_path_all_keys(&Map::build(INPUT_TEST_9)), 72);
    }
}
