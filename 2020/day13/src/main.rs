use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> (usize, Vec<Option<usize>>) {
    let (earliest_ts, bus_ids) = input.lines().collect_tuple().unwrap();
    (
        earliest_ts.parse().unwrap(),
        bus_ids.split(',').map(|id| id.parse().ok()).collect(),
    )
}

fn earliest_bus_wait_time(earliest_ts: usize, bus_ids: &[Option<usize>]) -> usize {
    let (id, diff) = bus_ids
        .iter()
        .filter_map(|id| *id)
        .map(|id| {
            let next_time = earliest_ts / id * id + id;
            (id, next_time - earliest_ts)
        })
        .min_by_key(|(_, diff)| *diff)
        .unwrap();
    id * diff
}

fn part2(earliest_ts: usize, bus_ids: &[Option<usize>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (earliest_ts, bus_ids) = build(&input);
    // println!("{}: {:?}", earliest_ts, bus_ids);

    println!("Part 1: {}", earliest_bus_wait_time(earliest_ts, &bus_ids));
    println!("Part 2: {}", part2(earliest_ts, &bus_ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (earliest_ts, bus_ids) = build(INPUT_TEST);
        assert_eq!(earliest_bus_wait_time(earliest_ts, &bus_ids), 295);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
