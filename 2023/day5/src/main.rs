// https://adventofcode.com/2023/day/5
// Part 1 test: 35
// Part 1: 282277027
// Part 2 test: 46
// Part 2: 11554135

use regex::Regex;
use std::{collections::HashMap, io, ops::Range};

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

fn convert(maps: &HashMap<(String, String), Vec<RangeMapping>>, seed: u64) -> u64 {
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

#[derive(Debug)]
struct SeedRanges{
    ranges: Vec<Range<u64>>,
}

impl SeedRanges {
    fn new(ranges: Vec<Range<u64>>) -> Self {
        let mut s = Self {
            ranges
        };
        s.ranges.sort_by_key(|r| r.start);
        s
    }

    fn first(&self) -> u64 {
        self.ranges[0].start
    }

    fn add(&self, val: u64, to_add: u64) -> Result<u64, &'static str> {
        let mut res = val;
        let mut inc = to_add;
        let mut idx: usize = self.ranges.iter().enumerate().find(|(_, r)| r.contains(&val)).unwrap().0;
        // println!("{val}");
        while !self.ranges[idx].contains(&(res + inc)) {
            inc -= self.ranges[idx].end - res;
            idx += 1;
            if idx >= self.ranges.len() {
                return Err("Reached end of ranges");
            }
            res = self.ranges[idx].start;
        }
        res += inc;
        Ok(res)
    }
}

#[test]
fn check_seed_range() {
    let ranges = vec![79..(79+14), 55..(55+13)];
    let s = SeedRanges::new(ranges);
    assert_eq!(s.first(), 55);
    assert_eq!(s.add(55, 2), Ok(57));
    assert_eq!(s.add(57, 13), Ok(81));
    assert_eq!(s.add(81, 11), Ok(92));
    assert!(s.add(92, 1).is_err());
}

fn main() {
    let stdin = io::stdin();
    let mut n = String::new();
    stdin.read_line(&mut n).unwrap();
    let seeds: Vec<u64> = n
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();

    let map_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let range_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let mut maps: HashMap<(String, String), Vec<RangeMapping>> = HashMap::new();
    // Initialization isn't used but needed to keep compiler happy
    let mut current_range_list: &mut Vec<RangeMapping> = &mut Vec::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        if s.ends_with(" map:") {
            let captures = map_re.captures(&s).unwrap();
            let k = (captures[1].to_string(), captures[2].to_string());

            current_range_list = maps.entry(k).or_insert_with(Vec::new);
        } else if !s.is_empty() {
            // ranges
            let captures = range_re.captures(&s).unwrap();
            current_range_list.push(RangeMapping::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ));
        }
    }

    // println!("Seeds {:?}", seeds);
    // println!("Maps {:#?}", maps);

    let location_part_1 = seeds
        .iter()
        .map(|seed| convert(&maps, *seed))
        .min()
        .unwrap();
    println!("Part 1: {}", location_part_1);


    let mut seed_ranges: Vec<Range<u64>> = seeds.chunks(2).map(|c| (c[0]..c[0] + c[1])).collect();
    seed_ranges.sort_by_key(|r| r.start);
    println!("Seed ranges {:#?}", seed_ranges);

    // Very crude brute force way of doing it.
    // First with a big STEP, identify which range most likely has the lowest.
    // Then on this range only decrease the step until 1 to get the lowest.
    let mut location = u64::MAX;
    const STEP: u64 = 100000;
    // const STEP: u64 = 1;
    let mut lowest_seed_idx = 0;
    seed_ranges.iter().enumerate()
    // .filter(|(i, _)| *i == 9)
    .for_each(|(i, range)| {
        let mut seed = range.start;
        while seed < range.end {
            let r = convert(&maps, seed);
            if r < location {
                lowest_seed_idx = i;
                location = r;
                println!("In {:?} found {}", range, location);
            }
            seed += STEP;
        }
    });
    println!("Idx: {}: {}", lowest_seed_idx, location);

    println!("Part 2: {}", location);

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
