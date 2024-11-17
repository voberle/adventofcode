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

    fn decrypt(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    decrypt_char(c, self.sector_id)
                }
            })
            .collect()
    }
}

fn decrypt_char(c: char, sector_id: u32) -> char {
    let offset = (sector_id % 26) as u8;
    let cu8 = c as u8;
    let letter = if cu8 + offset > b'z' {
        b'a' + (cu8 + offset - b'z' - 1)
    } else {
        cu8 + offset
    };
    std::char::from_u32(u32::from(letter)).unwrap()
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

fn north_pole_room_sector_id(rooms: &[Room]) -> u32 {
    const ROOM: &str = "northpole object storage";
    rooms
        .iter()
        .filter(|r| r.is_real_room())
        .find(|r| r.decrypt() == ROOM)
        .unwrap()
        .sector_id
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rooms = build(&input);

    println!("Part 1: {}", real_rooms_sector_ids_sum(&rooms));
    println!("Part 2: {}", north_pole_room_sector_id(&rooms));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_real_room() {
        assert!(Room::build("aaaaa-bbb-z-y-x-123[abxyz]").is_real_room());
        assert!(Room::build("a-b-c-d-e-f-g-h-987[abcde]").is_real_room());
        assert!(Room::build("not-a-real-room-404[oarel]").is_real_room());
        assert!(!Room::build("totally-real-room-200[decoy]").is_real_room());
    }

    #[test]
    fn test_decrypt_char() {
        assert_eq!(decrypt_char('a', 1), 'b');
        assert_eq!(decrypt_char('z', 1), 'a');
        assert_eq!(decrypt_char('q', 343), 'v');
        assert_eq!(decrypt_char('z', 343), 'e');
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            Room::build("qzmt-zixmtkozy-ivhz-343[doesntmatter]").decrypt(),
            "very encrypted name"
        );
    }
}
