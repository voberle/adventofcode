use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let p: Vec<&str> = line.split(" <-> ").collect();
            assert_eq!(p[0].parse::<usize>().unwrap(), i);
            p[1].split(", ").map(|e| e.parse().unwrap()).collect()
        })
        .collect()
}

// Recursive function.
fn mark_connected(connections: &[Vec<usize>], connected: &mut Vec<bool>, list: &[usize]) {
    for i in list {
        if !connected[*i] {
            connected[*i] = true;
            mark_connected(connections, connected, &connections[*i]);
        }
    }
}

fn connected_to(connections: &[Vec<usize>], program_nb: usize) -> Vec<bool> {
    let mut connected: Vec<bool> = vec![false; connections.len()];
    connected[program_nb] = true;

    mark_connected(connections, &mut connected, &connections[program_nb]);

    connected
}

fn connected_to_count(connections: &[Vec<usize>], program_nb: usize) -> usize {
    let connected = connected_to(connections, program_nb);
    connected.iter().filter(|v| **v).count()
}

fn groups_count(connections: &[Vec<usize>]) -> usize {
    // Find one connected group, remove it from the list, find the next and so on.
    let mut connections = connections.to_vec();
    let mut group_total = 0;
    while let Some(first_non_empty_idx) = connections.iter().position(|v| !v.is_empty()) {
        let conn_to = connected_to(&connections, first_non_empty_idx);

        let mut iter = conn_to.iter();
        // we don't actually remove from the connections list, as it would change the indexes.
        for e in &mut connections {
            if *iter.next().unwrap() {
                *e = Vec::new();
            }
        }

        group_total += 1;
    }

    group_total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let connections = build(&input);

    println!("Part 1: {}", connected_to_count(&connections, 0));
    println!("Part 2: {}", groups_count(&connections));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(connected_to_count(&build(INPUT_TEST), 0), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(groups_count(&build(INPUT_TEST)), 2);
    }
}
