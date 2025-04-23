use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn build(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.split(" <-> ").map(ToString::to_string).collect())
        .collect()
}

fn unique_locations_count(paths: &[Vec<String>]) -> usize {
    paths.iter().flatten().collect::<HashSet<&String>>().len()
}

// All connections for each locations.
fn create_connection_map(paths: &[Vec<String>]) -> HashMap<String, HashSet<String>> {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::default();
    for path in paths {
        for (p1, p2) in [(&path[0], &path[1]), (&path[1], &path[0])] {
            connections
                .entry(p1.clone())
                .and_modify(|conns| {
                    conns.insert(p2.clone());
                })
                .or_insert({
                    let mut set = HashSet::default();
                    set.insert(p2.clone());
                    set
                });
        }
    }
    connections
}

fn unique_locations_under(paths: &[Vec<String>], from: &str, time: usize) -> usize {
    let connections = create_connection_map(paths);

    let mut reached: HashSet<String> = HashSet::default();
    reached.insert(from.to_string());

    let mut t = 0;
    while t < time {
        let mut next: HashSet<String> = HashSet::default();
        next.extend(reached.clone());
        for c in &reached {
            next.extend(connections.get(c).unwrap().iter().cloned());
        }
        reached = next;
        t += 1;
    }

    reached.len()
}

fn part2(paths: &[Vec<String>]) -> usize {
    unique_locations_under(paths, "STT", 3)
}

fn shortests_time_from(paths: &[Vec<String>], from: &str) -> usize {
    let connections = create_connection_map(paths);

    let mut times: HashMap<String, usize> = HashMap::default();
    times.insert(from.to_string(), 0);

    // Reuse the brute-force solution of part 2.
    let mut reached: HashSet<String> = HashSet::default();
    reached.insert(from.to_string());

    let mut t = 1;
    while times.len() < connections.len() {
        // Build what can be reached next.
        let mut next: HashSet<String> = HashSet::default();
        next.extend(reached.clone());
        for c in &reached {
            next.extend(connections.get(c).unwrap().iter().cloned());
        }

        // Find if something can be reached that couldn't before.
        for c in connections.keys() {
            if !times.contains_key(c) && next.contains(c) {
                times.insert(c.clone(), t);
            }
        }

        t += 1;
        reached = next;
    }

    times.values().sum()
}

fn part3(paths: &[Vec<String>]) -> usize {
    shortests_time_from(paths, "STT")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let paths = build(&input);

    println!("Part 1: {}", unique_locations_count(&paths));
    println!("Part 2: {}", part2(&paths));
    println!("Part 3: {}", part3(&paths));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(unique_locations_count(&build(INPUT_TEST)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 6);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(&build(INPUT_TEST)), 15);
    }
}
