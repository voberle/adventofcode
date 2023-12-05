// https://adventofcode.com/2023/day/5
// Part 1 test: 35
// Part 1: 282277027

use regex::Regex;
use std::{collections::HashMap, io};

#[derive(Debug)]
struct Range {
    dest_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}

impl Range {
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
    let r1 = Range::new(50, 98, 2);
    assert!(r1.is_in(98));
    assert_eq!(r1.convert(98), 50);
    assert!(r1.is_in(99));
    assert_eq!(r1.convert(99), 51);
    assert!(!r1.is_in(10));
    let r2 = Range::new(52, 50, 48);
    assert!(r2.is_in(53));
    assert_eq!(r2.convert(53), 55);
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

    let mut maps: HashMap<(String, String), Vec<Range>> = HashMap::new();
    // Initialization isn't used but needed to keep compiler happy
    let mut current_range_list: &mut Vec<Range> = &mut Vec::new();
    for line in stdin.lines() {
        let s = line.unwrap();
        if s.ends_with(" map:") {
            let captures = map_re.captures(&s).unwrap();
            let k = (captures[1].to_string(), captures[2].to_string());

            current_range_list = maps.entry(k).or_insert_with(Vec::new);
        } else if !s.is_empty() {
            // ranges
            let captures = range_re.captures(&s).unwrap();
            current_range_list.push(Range::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ));
        }
    }

    // println!("Seeds {:?}", seeds);
    // println!("Maps {:#?}", maps);

    let location = seeds
        .iter()
        .map(|seed| {
            let mut n = *seed;
            let mut item = "seed";
            while item != "location" {
                let map = maps.iter().find(|(k, _)| k.0 == item).unwrap();
                if let Some(range) = map.1.iter().find(|r| r.is_in(n)) {
                    n = range.convert(n);
                }
                item = &map.0 .1;
            }
            n
        })
        .min()
        .unwrap();

    println!("Part 1: {}", location);
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
