use std::io::{self, Read};

use itertools::Itertools;

fn build_bus_ids(input: &str) -> Vec<Option<u64>> {
    input.split(',').map(|id| id.parse().ok()).collect()
}

fn build(input: &str) -> (u64, Vec<Option<u64>>) {
    let (earliest_ts, bus_ids) = input.lines().collect_tuple().unwrap();
    (earliest_ts.parse().unwrap(), build_bus_ids(bus_ids))
}

fn earliest_bus_wait_time(earliest_ts: u64, bus_ids: &[Option<u64>]) -> u64 {
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

fn opt_depart_time(bus_ids: &[Option<u64>]) -> u64 {
    assert!(bus_ids.first().unwrap().is_some());

    // Convert the bus ids into a list of time + offset.
    let time_offset: Vec<(u64, u64)> = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(offset, id)| id.as_ref().map(|time| (*time, offset as u64)))
        .collect();
    // NB: Optimize by sorting by time desc??

    // For n = 1, n++
    // - Take first element, candidate = time * n.
    // - For all other elements in the list, check if:
    //   (candidate + offset) % time == 0
    for n in 1.. {
        let time = time_offset[0].0;
        let candidate = time * n;
        if time_offset
            .iter()
            .skip(1)
            .all(|(time, offset)| (candidate + offset) % time == 0)
        {
            return candidate;
        }
    }
    panic!("No timestamp found");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (earliest_ts, bus_ids) = build(&input);

    println!("Part 1: {}", earliest_bus_wait_time(earliest_ts, &bus_ids));
    println!("Part 2: {}", opt_depart_time(&bus_ids));
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
        let (_, bus_ids) = build(INPUT_TEST);
        assert_eq!(opt_depart_time(&bus_ids), 1068781);

        assert_eq!(opt_depart_time(&build_bus_ids("17,x,13,19")), 3417);
        assert_eq!(opt_depart_time(&build_bus_ids("67,7,59,61")), 754018);
        assert_eq!(opt_depart_time(&build_bus_ids("67,x,7,59,61")), 779210);
        assert_eq!(opt_depart_time(&build_bus_ids("67,7,x,59,61")), 1261476);
        assert_eq!(
            opt_depart_time(&build_bus_ids("1789,37,47,1889")),
            1202161486
        );
    }
}
