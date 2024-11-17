use std::io::{self, Read};

use itertools::Itertools;

fn hash(door_id: &str, index: i32) -> String {
    let digest = md5::compute(format!("{door_id}{index}").as_bytes());
    format!("{digest:x}")
}

const START: &str = "00000";

fn find_password(door_id: &str) -> String {
    let mut password = String::with_capacity(8);
    let mut index = 0;
    while password.len() < 8 {
        let s = hash(door_id, index);
        if s.starts_with(START) {
            password.push(s.trim_start_matches(START).chars().next().unwrap());
        }
        index += 1;
    }
    password
}

fn find_second_password(door_id: &str) -> String {
    let mut password: [Option<char>; 8] = [None; 8];
    let mut index = 0;
    let mut password_len = 0;
    while password_len < 8 {
        let s = hash(door_id, index);
        if s.starts_with(START) {
            let mut it = s.trim_start_matches(START).chars();
            if let Some(pos) = it.next().unwrap().to_digit(10) {
                if pos < 8 {
                    let i = pos as usize;
                    let letter = it.next().unwrap();
                    if password[i].is_none() {
                        password[i] = Some(letter);
                        password_len += 1;
                    }
                }
            }
        }
        index += 1;
    }
    password.iter().map(|c| c.unwrap()).join("")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", find_password(&input));
    println!("Part 2: {}", find_second_password(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(find_password("abc"), "18f47a30");
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_second_password("abc"), "05ace8e3");
    }
}
