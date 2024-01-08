use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn build(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.+)-(\d+)\[(\w+)\]").unwrap();
        }
        let parts = RE.captures(input).unwrap();
        Self {
            name: parts[1].to_string(),
            sector_id: parts[2].parse().unwrap(),
            checksum: parts[3].to_string(),
        }
    }

    fn is_real_room(&self) -> bool {
        let frequencies =
            self.name
                .chars()
                .filter(|c| *c != '-')
                .fold(HashMap::new(), |mut map, val| {
                    map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                    map
                });
        let checksum: String = frequencies
            .iter()
            .sorted_by(|(ak, av), (bk, bv)| {
                let o = Ord::cmp(&bv, &av);
                if o.is_eq() {
                    ak.cmp(bk)
                } else {
                    o
                }
            })
            .map(|(k, _)| *k)
            .take(5)
            .collect();
        checksum == self.checksum
    }
}

fn build(input: &str) -> Vec<Room> {
    input.lines().map(Room::build).collect()
}

fn real_rooms_sector_ids_sum(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|r| r.is_real_room())
        .map(|r| r.sector_id)
        .sum()
}

fn part2(rooms: &[Room]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rooms = build(&input);

    println!("Part 1: {}", real_rooms_sector_ids_sum(&rooms));
    println!("Part 2: {}", part2(&rooms));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_real_room() {
        assert_eq!(
            Room::build("aaaaa-bbb-z-y-x-123[abxyz]").is_real_room(),
            true
        );
        assert_eq!(
            Room::build("a-b-c-d-e-f-g-h-987[abcde]").is_real_room(),
            true
        );
        assert_eq!(
            Room::build("not-a-real-room-404[oarel]").is_real_room(),
            true
        );
        assert_eq!(
            Room::build("totally-real-room-200[decoy]").is_real_room(),
            false
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
