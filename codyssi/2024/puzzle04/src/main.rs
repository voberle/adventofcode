use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
    iter::once,
};

// All connections for each locations.
type Connections = HashMap<String, HashSet<String>>;

fn create_connection_map(input: &str) -> Connections {
    let mut connections: Connections = HashMap::default();
    for line in input.lines() {
        let path: Vec<&str> = line.split(" <-> ").collect();

        for (p1, p2) in [(path[0], path[1]), (path[1], path[0])] {
            connections
                .entry(p1.to_string())
                .and_modify(|conns| {
                    conns.insert(p2.to_string());
                })
                .or_insert({
                    let mut set = HashSet::default();
                    set.insert(p2.to_string());
                    set
                });
        }
    }
    connections
}

fn unique_locations_count(connections: &Connections) -> usize {
    connections.len()
}

// Returns what can be reached directly from this set of locations.
fn reached_from(connections: &Connections, from: &HashSet<String>) -> HashSet<String> {
    let mut next: HashSet<String> = HashSet::default();
    next.extend(from.clone());
    for c in from {
        next.extend(connections.get(c).unwrap().iter().cloned());
    }
    next
}

fn unique_locations_under(connections: &Connections, from: &str, time: usize) -> usize {
    let mut reached: HashSet<String> = once(from.to_string()).collect();

    let mut t = 0;
    while t < time {
        reached = reached_from(connections, &reached);
        t += 1;
    }

    reached.len()
}

fn part2(connections: &Connections) -> usize {
    unique_locations_under(connections, "STT", 3)
}

fn shortests_time_from(connections: &Connections, from: &str) -> usize {
    let mut times: HashMap<String, usize> = HashMap::default();
    times.insert(from.to_string(), 0);

    let mut reached: HashSet<String> = once(from.to_string()).collect();

    let mut t = 1;
    // Loop until we have reached everything.
    while times.len() < connections.len() {
        reached = reached_from(connections, &reached);

        // Find if something can be reached that couldn't before.
        for loc in connections.keys() {
            if !times.contains_key(loc) && reached.contains(loc) {
                times.insert(loc.clone(), t);
            }
        }

        t += 1;
    }

    times.values().sum()
}

fn part3(connections: &Connections) -> usize {
    shortests_time_from(connections, "STT")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let connections = create_connection_map(&input);

    println!("Part 1: {}", unique_locations_count(&connections));
    println!("Part 2: {}", part2(&connections));
    println!("Part 3: {}", part3(&connections));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(
            unique_locations_count(&create_connection_map(INPUT_TEST)),
            7
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&create_connection_map(INPUT_TEST)), 6);
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(&create_connection_map(INPUT_TEST)), 15);
    }
}
