use std::{
    io::{self, Read},
    ops::Range,
};

use fxhash::FxHashMap;
use regex::Regex;

#[derive(Debug)]
struct RangeMapping {
    dest_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl RangeMapping {
    fn new(dest_range_start: u64, src_range_start: u64, range_len: u64) -> Self {
        Self {
            dest_range_start,
            src_range_start,
            range_len,
        }
    }

    fn is_in(&self, src: u64) -> bool {
        src >= self.src_range_start && src < self.src_range_start + self.range_len
    }

    fn convert(&self, src: u64) -> u64 {
        assert!(self.is_in(src));
        self.dest_range_start + (src - self.src_range_start)
    }
}

type SeedMap = FxHashMap<(String, String), Vec<RangeMapping>>;

fn convert(maps: &SeedMap, seed: u64) -> u64 {
    let mut n = seed;
    let mut item = "seed";
    while item != "location" {
        let map = maps.iter().find(|(k, _)| k.0 == item).unwrap();
        if let Some(range) = map.1.iter().find(|r| r.is_in(n)) {
            n = range.convert(n);
        }
        item = &map.0 .1;
    }
    n
}

// Parses the input, returning the list of seeds and the seed maps.
fn build(input: &str) -> (Vec<u64>, SeedMap) {
    let mut it = input.lines();

    let seeds: Vec<u64> = it
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    it.next();

    let map_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let range_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut maps: SeedMap = FxHashMap::default();
    // Initialization isn't used but needed to keep compiler happy
    let mut current_range_list: &mut Vec<RangeMapping> = &mut Vec::new();
    for s in it {
        if s.ends_with(" map:") {
            let captures = map_re.captures(s).unwrap();
            let k = (captures[1].to_string(), captures[2].to_string());

            current_range_list = maps.entry(k).or_default();
        } else if !s.is_empty() {
            // ranges
            let captures = range_re.captures(s).unwrap();
            current_range_list.push(RangeMapping::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ));
        }
    }
    (seeds, maps)
}

fn lowest_location_v1(seeds: &[u64], maps: &SeedMap) -> u64 {
    seeds.iter().map(|seed| convert(maps, *seed)).min().unwrap()
}

// Get the seed ranges used by part 2.
fn get_initial_seed_ranges(seeds: &[u64]) -> Vec<Range<u64>> {
    let mut seed_ranges: Vec<Range<u64>> = seeds.chunks(2).map(|c| (c[0]..c[0] + c[1])).collect();
    seed_ranges.sort_by_key(|r| r.start);
    seed_ranges
}

// Recursive.
// Idea is to binary-search it basically, and whenever we have a range where the
// difference in source [beginning,end] is same as destination [beginning,end],
// then the minimum is beginning.
fn lowest_location_between(maps: &SeedMap, low_seed: u64, high_seed: u64) -> u64 {
    if low_seed == high_seed {
        return convert(maps, low_seed);
    }

    let low_loc = convert(maps, low_seed);
    let high_loc = convert(maps, high_seed);
    if low_seed <= high_seed && low_loc <= high_loc && (high_loc - low_loc == high_seed - low_seed)
    {
        return low_loc;
    }
    let mid_seed = ((high_seed - low_seed) / 2) + low_seed;
    let low_part = lowest_location_between(maps, low_seed, mid_seed);
    let high_part = lowest_location_between(maps, mid_seed + 1, high_seed);
    low_part.min(high_part)
}

fn lowest_location_v2(seeds: &[u64], maps: &SeedMap) -> u64 {
    let seed_ranges = get_initial_seed_ranges(seeds);
    seed_ranges
        .iter()
        .map(|range| lowest_location_between(maps, range.start, range.end - 1))
        .min()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (seeds, maps) = build(&input);

    println!("Part 1: {}", lowest_location_v1(&seeds, &maps));
    println!("Part 2: {}", lowest_location_v2(&seeds, &maps));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn check_convert() {
        let r1 = RangeMapping::new(50, 98, 2);
        assert!(r1.is_in(98));
        assert_eq!(r1.convert(98), 50);
        assert!(r1.is_in(99));
        assert_eq!(r1.convert(99), 51);
        assert!(!r1.is_in(10));
        let r2 = RangeMapping::new(52, 50, 48);
        assert!(r2.is_in(53));
        assert_eq!(r2.convert(53), 55);
    }

    #[test]
    fn check_re() {
        let s = "seed-to-soil map:";
        let map_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
        assert!(map_re.is_match(s));
        let captures = map_re.captures(s).unwrap();
        assert_eq!(&captures[1], "seed");
        assert_eq!(&captures[2], "soil");
    }

    #[test]
    fn test_part1() {
        let (seeds, maps) = build(INPUT_TEST);
        assert_eq!(lowest_location_v1(&seeds, &maps), 35);
    }

    #[test]
    fn test_part2() {
        let (seeds, maps) = build(INPUT_TEST);
        assert_eq!(lowest_location_v2(&seeds, &maps), 46);
    }
}
