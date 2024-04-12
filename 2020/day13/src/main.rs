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

// Convert the bus ids into a list of time + offset.
fn get_time_offset_list(bus_ids: &[Option<u64>]) -> Vec<(i128, i128)> {
    bus_ids
        .iter()
        .enumerate()
        .filter_map(|(offset, id)| id.as_ref().map(|time| (i128::from(*time), offset as i128)))
        .collect()
}

// Brute force-version
fn _opt_depart_time(bus_ids: &[Option<u64>]) -> i128 {
    assert!(bus_ids.first().unwrap().is_some());

    let time_offset = get_time_offset_list(bus_ids);

    // We can biggest time as "base".
    let max_timeoffset = *time_offset.iter().max_by_key(|(time, _)| *time).unwrap();

    let start = 1;
    // For real input, description tells us it's at least that big.
    // let start = 100_000_000_000_000 / max_timeoffset.0;

    // For n = 1, n++
    // - Take first element, candidate = time * n.
    // - For all other elements in the list, check if:
    //   (candidate + offset) % time == 0
    for n in start.. {
        let candidate = max_timeoffset.0 * n - max_timeoffset.1;

        if n % 10_000_000_000 == 0 {
            println!("candidate {}", candidate);
        }

        if time_offset
            .iter()
            .all(|(time, offset)| (candidate + offset) % time == 0)
        {
            return candidate;
        }
    }
    panic!("No timestamp found");
}

// Optimized version using LCM with offset.
// From https://math.stackexchange.com/questions/2218763/how-to-find-lcm-of-two-numbers-when-one-starts-with-an-offset
fn opt_depart_time(bus_ids: &[Option<u64>]) -> i128 {
    let time_offset = get_time_offset_list(bus_ids);

    let (mut period, mut phase) = time_offset[0];
    for (p, o) in &time_offset[1..] {
        (period, phase) = combine_phased_rotations(period, phase, *p, *o);
    }
    (-phase).rem_euclid(period)
}

// Combine two phased rotations into a single phased rotation.
// Returns: combined_period, combined_phase
// The combined rotation is at its reference point if and only if both a and b are at their reference points.
fn combine_phased_rotations(
    a_period: i128,
    a_phase: i128,
    b_period: i128,
    b_phase: i128,
) -> (i128, i128) {
    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    let pd_mult = phase_difference / gcd;
    let pd_remainder = phase_difference.rem_euclid(gcd);
    assert!(
        pd_remainder == 0,
        "Rotation reference points never synchronize."
    );

    let combined_period = a_period / gcd * b_period;
    let combined_phase = (a_phase - s * pd_mult * a_period).rem_euclid(combined_period);
    (combined_period, combined_phase)
}

// Extended Greatest Common Divisor Algorithm.
// Returns:
//    gcd: The greatest common divisor of a and b.
//    s, t: Coefficients such that s*a + t*b = gcd
// Reference: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
#[allow(clippy::many_single_char_names)]
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let quotient = old_r / r;
        let remainder = old_r.rem_euclid(r);
        (old_r, r) = (r, remainder);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }
    (old_r, old_s, old_t)
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
