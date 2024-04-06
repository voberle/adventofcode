use std::{
    io::{self, Read},
    string::ToString,
};

use fxhash::FxHashMap;
use itertools::Itertools;

type Passport = FxHashMap<String, String>;

fn build(input: &str) -> Vec<Passport> {
    let mut passports = vec![FxHashMap::default()];
    for line in input.lines() {
        // Passport data can be on several lines.
        if line.is_empty() {
            passports.push(FxHashMap::default());
        }
        passports
            .last_mut()
            .unwrap()
            .extend(line.split_whitespace().map(|f| {
                f.split(':')
                    .map(ToString::to_string)
                    .collect_tuple()
                    .unwrap()
            }));
    }
    passports
}

fn check_number(data: &str, from: u32, to: u32) -> bool {
    if let Ok(y) = data.parse::<u32>() {
        (from..=to).contains(&y)
    } else {
        false
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn check_byr(data: &str) -> bool {
    check_number(data, 1920, 2002)
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn check_iyr(data: &str) -> bool {
    check_number(data, 2010, 2020)
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn check_eyr(data: &str) -> bool {
    check_number(data, 2020, 2030)
}

// hgt (Height) - a number followed by either cm or in:
// - If cm, the number must be at least 150 and at most 193.
// - If in, the number must be at least 59 and at most 76.
fn check_hgt(data: &str) -> bool {
    if data.len() >= 3 {
        let nb = &data[0..data.len() - 2];
        let pf = &data[data.len() - 2..data.len()];
        if pf == "cm" {
            return check_number(nb, 150, 193);
        } else if pf == "in" {
            return check_number(nb, 59, 76);
        }
    }
    false
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn check_hcl(data: &str) -> bool {
    data.len() == 7
        && data.starts_with('#')
        && data
            .chars()
            .skip(1)
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn check_ecl(data: &str) -> bool {
    const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    EYE_COLORS.contains(&data)
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn check_pid(data: &str) -> bool {
    data.len() == 9 && data.chars().all(|c| c.is_ascii_digit())
}

type ValidationFn = fn(&str) -> bool;

const MANDATORY_FIELDS: [(&str, ValidationFn); 7] = [
    ("byr", check_byr),
    ("iyr", check_iyr),
    ("eyr", check_eyr),
    ("hgt", check_hgt),
    ("hcl", check_hcl),
    ("ecl", check_ecl),
    ("pid", check_pid),
    // cid (Country ID) - ignored, missing or not.
];

fn valid_passports_count<const STRICT: bool>(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|p| {
            MANDATORY_FIELDS.iter().all(|(name, check_fn)| {
                if let Some(n) = p.get(*name) {
                    if STRICT {
                        check_fn(n)
                    } else {
                        // present is enough
                        true
                    }
                } else {
                    false
                }
            })
        })
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let passports = build(&input);

    println!("Part 1: {}", valid_passports_count::<false>(&passports));
    println!("Part 2: {}", valid_passports_count::<true>(&passports));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(valid_passports_count::<false>(&build(INPUT_TEST_1)), 2);
    }

    #[test]
    fn test_check_fields() {
        assert!(check_byr("2002"));
        assert!(!check_byr("2003"));

        assert!(check_hgt("60in"));
        assert!(check_hgt("190cm"));
        assert!(!check_hgt("190in"));
        assert!(!check_hgt("190"));

        assert!(check_hcl("#123abc"));
        assert!(!check_hcl("#123abz"));
        assert!(!check_hcl("123abc"));

        assert!(check_ecl("brn"));
        assert!(!check_ecl("wat"));

        assert!(check_pid("000000001"));
        assert!(!check_pid("0123456789"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(valid_passports_count::<true>(&build(INPUT_TEST_2)), 4);
    }
}
