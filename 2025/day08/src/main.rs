use std::{
    fmt::Display,
    io::{self, Read},
};

use fxhash::FxHashMap;
use itertools::Itertools;

struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl Position {
    fn build(s: &str) -> Self {
        let parts: Vec<_> = s.split(',').map(|p| p.parse().unwrap()).collect();
        Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

fn build(input: &str) -> Vec<Position> {
    input.lines().map(Position::build).collect()
}

fn make_connections(positions: &[Position]) -> Vec<(usize, usize, u64)> {
    // List of all connections
    let mut connections: Vec<(usize, usize, u64)> = (0..positions.len())
        .combinations(2)
        .map(|p| {
            let a = &positions[p[0]];
            let b = &positions[p[1]];
            // No need to square root, we just care about ordering.
            let diff =
                a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) + a.z.abs_diff(b.z).pow(2);
            (p[0], p[1], diff)
        })
        .collect();
    // Sorted by closeness.
    connections.sort_unstable_by_key(|c| c.2);
    connections
}

fn three_largest_product(
    positions: &[Position],
    connections: &[(usize, usize, u64)],
    connections_count: usize,
) -> usize {
    let mut circuits: Vec<usize> = (0..positions.len()).collect();

    // Connect the requested number of circuits.
    for (i1, i2, _) in connections.iter().take(connections_count) {
        let (circuit_id1, circuit_id2) = (circuits[*i1], circuits[*i2]);

        // Skip if it's already in the same circuit.
        if circuit_id1 == circuit_id2 {
            // println!("Skipping {i1}:{} - {i2}:{}", positions[i1], positions[i2]);
            continue;
        }

        // Connect the two circuits.
        // println!("Connecting {i1}:{} - {i2}:{}", positions[i1], positions[i2]);
        for circuit_id in &mut circuits {
            if *circuit_id == circuit_id1 {
                *circuit_id = circuit_id2;
            }
        }
    }

    // Calculate the sizes of each circuit.
    let mut circuits_sizes: FxHashMap<usize, usize> = FxHashMap::default();
    for x in circuits {
        *circuits_sizes.entry(x).or_default() += 1;
    }
    let mut sizes: Vec<_> = circuits_sizes.values().collect();
    sizes.sort();
    // println!("{:?}", sizes);

    sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
}

fn x_product_all_connected(positions: &[Position], connections: &[(usize, usize, u64)]) -> u64 {
    let mut circuits: Vec<usize> = (0..positions.len()).collect();

    // Connect until we have only one circuit.
    for (i1, i2, _) in connections {
        let (circuit_id1, circuit_id2) = (circuits[*i1], circuits[*i2]);

        if circuit_id1 == circuit_id2 {
            continue;
        }

        for circuit_id in &mut circuits {
            if *circuit_id == circuit_id1 {
                *circuit_id = circuit_id2;
            }
        }

        if circuits.iter().all(|v| *v == circuits[0]) {
            // One circuit, done.
            return positions[*i1].x * positions[*i2].x;
        }
    }
    panic!("Didn't manage to create one circuit.");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let positions = build(&input);

    let connections = make_connections(&positions);

    println!(
        "Part 1: {}",
        three_largest_product(&positions, &connections, 1000)
    );
    println!(
        "Part 2: {}",
        x_product_all_connected(&positions, &connections)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let positions = build(INPUT_TEST);
        let connections = make_connections(&positions);
        assert_eq!(three_largest_product(&positions, &connections, 10), 40);
    }

    #[test]
    fn test_part2() {
        let positions = build(INPUT_TEST);
        let connections = make_connections(&positions);
        assert_eq!(x_product_all_connected(&positions, &connections), 25272);
    }
}
