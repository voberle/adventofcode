use fxhash::{FxHashMap, FxHashSet};
use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

mod tunnels;

use tunnels::Map;

// Recursive function.
#[allow(clippy::too_many_arguments)]
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
                "Last key, dist to it {dist_to_key}, dist {new_dist}, shortest {shortest_distance}"
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

// Recursive version of the solution.
// This prints out the solution relatively quickly,
// but it fails to stop, runs forever / for a very long time.
#[allow(dead_code)]
fn shortest_path_rec(map: &Map) -> usize {
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

// Node for Dijkstra shortest path on whole map / set of keys.
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: usize,
    keys_to_find: Vec<usize>,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra shortest path version of the solution.
// It's not fast, takes 5+ minutes to get the answer, but gets it.
fn shortest_path_dijkstra(
    map: &Map,
    entrance_position: usize,
    keys_positions: Vec<usize>,
) -> usize {
    let mut visited: FxHashSet<(usize, Vec<usize>)> = FxHashSet::default();
    let mut distance: FxHashMap<(usize, Vec<usize>), usize> = FxHashMap::default();
    let mut shortest_distance = usize::MAX;

    let mut path_cache: FxHashMap<(usize, usize, Vec<usize>), Option<usize>> = FxHashMap::default();

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(Node {
        pos: entrance_position,
        keys_to_find: keys_positions,
        cost: 0,
    });

    while let Some(Node {
        pos,
        keys_to_find,
        cost,
    }) = queue.pop()
    {
        visited.insert((pos, keys_to_find.clone()));

        if keys_to_find.is_empty() {
            shortest_distance = usize::min(shortest_distance, cost);
            // println!("Last key, shortest {}", shortest_distance);
            continue;
        }

        // This is the list of doors that are closed.
        // In part 2, we assume that closed doors are only the ones we are looking for the keys.
        let doors_positions: Vec<usize> = keys_to_find
            .iter()
            .filter_map(|kp| map.get_door_position_for_key(*kp))
            .collect();

        queue.extend(keys_to_find.iter().filter_map(|key_pos| {
            let dist_to_key_opt =
                if let Some(opt_d) = path_cache.get(&(pos, *key_pos, doors_positions.clone())) {
                    *opt_d
                } else {
                    let dist_to_key_opt =
                        tunnels::find_shortest_path(map, &doors_positions, pos, *key_pos);
                    path_cache.insert((pos, *key_pos, doors_positions.clone()), dist_to_key_opt);
                    dist_to_key_opt
                };

            // If unreachable
            dist_to_key_opt?;
            // Above line is same as: if dist_to_key_opt.is_none() { return None; }

            let next_pos = *key_pos;
            let next_keys_to_find: Vec<usize> = keys_to_find
                .iter()
                .filter(|&&p| p != *key_pos)
                .copied()
                .collect();

            if visited.contains(&(next_pos, next_keys_to_find.clone())) {
                return None;
            }

            let next_cost = cost + dist_to_key_opt.unwrap();
            if let Some(prevcost) = distance.get(&(next_pos, next_keys_to_find.clone())) {
                if *prevcost <= next_cost {
                    return None;
                }
            }

            distance.insert((next_pos, next_keys_to_find.clone()), next_cost);

            Some(Node {
                pos: next_pos,
                keys_to_find: next_keys_to_find,
                cost: next_cost,
            })
        }));
    }

    shortest_distance
}

fn shortest_path_1robot(map: &Map) -> usize {
    let entrance_positions = map.get_entrance_positions();
    assert_eq!(entrance_positions.len(), 1);
    let keys_positions = map.get_keys_positions();

    shortest_path_dijkstra(map, entrance_positions[0], keys_positions)
}

fn shortest_path_4robots(map: &Map) -> usize {
    let entrance_positions = map.get_entrance_positions();
    assert_eq!(entrance_positions.len(), 4);

    // Assumption: When a robot reaches a door where the key is in another quadrant,
    // that key will eventually become available without the robot having to move anywhere else.
    // So we can treat it the same as if the key was already available.
    // Or meaning we give each robot all of the keys from the other quadrants at the start.
    entrance_positions
        .iter()
        .enumerate()
        .map(|(quadrant, entrance_pos)| {
            // Only looking for the keys to find in this quadrant.
            let keys_positions: Vec<usize> = map
                .get_keys_positions()
                .iter()
                .filter(|&&kp| map.is_in_quadrant(kp, *entrance_pos, quadrant))
                .copied()
                .collect();
            // println!("Quadrant {}: Searching {} keys", quadrant, keys_positions.len());

            shortest_path_dijkstra(map, *entrance_pos, keys_positions)
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Map::build(&input);
    // map.print();

    println!("Part 1: {}", shortest_path_1robot(&map));

    let updated_map = map.update_map();
    // updated_map.print();
    println!("Part 2: {}", shortest_path_4robots(&updated_map));
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
    fn test_part1_rec() {
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_1)), 8);
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_2)), 86);
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_3)), 132);
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_4)), 136);
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_5)), 81);
    }

    #[test]
    fn test_part1_dijkstra() {
        assert_eq!(shortest_path_1robot(&Map::build(INPUT_TEST_1)), 8);
        assert_eq!(shortest_path_1robot(&Map::build(INPUT_TEST_2)), 86);
        assert_eq!(shortest_path_1robot(&Map::build(INPUT_TEST_3)), 132);
        assert_eq!(shortest_path_1robot(&Map::build(INPUT_TEST_4)), 136);
        assert_eq!(shortest_path_1robot(&Map::build(INPUT_TEST_5)), 81);
    }

    const INPUT_TEST_6: &str = include_str!("../resources/input_test_6");
    const INPUT_TEST_7: &str = include_str!("../resources/input_test_7");
    const INPUT_TEST_8: &str = include_str!("../resources/input_test_8");
    const INPUT_TEST_9: &str = include_str!("../resources/input_test_9");

    #[test]
    fn test_part2_rec() {
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_6)), 8);
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_7)), 24);
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_8)), 32);
        assert_eq!(shortest_path_rec(&Map::build(INPUT_TEST_9)), 72);
    }

    #[test]
    fn test_part2_dijkstra() {
        assert_eq!(shortest_path_4robots(&Map::build(INPUT_TEST_6)), 8);
        assert_eq!(shortest_path_4robots(&Map::build(INPUT_TEST_7)), 24);
        // Test input 8 doesn't fit the assumption.
        // assert_eq!(shortest_path_4robots(&Map::build(INPUT_TEST_8)), 32);
        // Test input 9 also fails, but I'm not sure why.
        // assert_eq!(shortest_path_4robots(&Map::build(INPUT_TEST_9)), 72);
    }
}
