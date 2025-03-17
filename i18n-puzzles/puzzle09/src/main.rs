use std::{
    fmt::Display,
    io::{self, Read},
};

use fxhash::FxHashMap;
use itertools::Itertools;

// Possible formats that the dates can be in.
// We use a bitmask, as bit manipulations allow us to easily include/exclude cases.
type Formats = u8;

// Year can never be in the middle.
const DMY: u8 = 0b0001;
const MDY: u8 = 0b0010;
const YMD: u8 = 0b0100;
const YDM: u8 = 0b1000;

fn is_format(mask: Formats, format: Formats) -> bool {
    mask & format != 0
}

fn is_leap_year(year: u8) -> bool {
    // years are between 1920 and 2020
    year % 4 == 0
}

#[allow(clippy::match_same_arms)]
fn get_month_length(month: u8, year: u8) -> u8 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => panic!("Invalid month value"),
    }
}

// Checks if these 3 values would make a valid date.
fn is_valid_date(day: u8, month: u8, year: u8) -> bool {
    day > 0 && month > 0 && year < 100 && month <= 12 && day <= get_month_length(month, year)
}

#[allow(dead_code)]
fn formats_to_str(formats: Formats) -> String {
    let mut s = Vec::new();
    if is_format(formats, DMY) {
        s.push("DMY");
    }
    if is_format(formats, MDY) {
        s.push("MDY");
    }
    if is_format(formats, YMD) {
        s.push("YMD");
    }
    if is_format(formats, YDM) {
        s.push("YDM");
    }
    s.join(", ")
}

struct Date(Vec<u8>);

impl Date {
    fn possible_formats(&self) -> Formats {
        let mut formats = 0;

        let f1 = self.0[0];
        let f2 = self.0[1];
        let f3 = self.0[2];

        if is_valid_date(f1, f2, f3) {
            formats |= DMY;
        }
        if is_valid_date(f2, f1, f3) {
            formats |= MDY;
        }
        if is_valid_date(f3, f2, f1) {
            formats |= YMD;
        }
        if is_valid_date(f2, f3, f1) {
            formats |= YDM;
        }

        formats
    }

    fn is_nine_eleven(&self, formats: Formats) -> bool {
        if is_format(formats, DMY) && (self.0 == vec![11, 9, 1]) {
            return true;
        }
        if is_format(formats, MDY) && (self.0 == vec![9, 11, 1]) {
            return true;
        }
        if is_format(formats, YMD) && (self.0 == vec![1, 9, 11]) {
            return true;
        }
        if is_format(formats, YDM) && (self.0 == vec![1, 11, 9]) {
            return true;
        }
        false
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.0[0], self.0[1], self.0[2])
    }
}

fn build(input: &str) -> Vec<(Date, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let (date_str, names_str) = line.split(':').collect_tuple().unwrap();
            let date_fields = date_str.split('-').map(|f| f.parse().unwrap()).collect();
            let names = names_str.split(',').map(|n| n.trim().to_string()).collect();
            (Date(date_fields), names)
        })
        .collect()
}

fn nine_eleven_entries_names(entries: &[(Date, Vec<String>)]) -> String {
    // Associate each name with the possible formats it may use.
    let mut names_format: FxHashMap<String, Formats> = FxHashMap::default();
    for (date, names) in entries {
        let possible_formats = date.possible_formats();
        // println!("{date}: {}", formats_to_str(possible_formats));

        for name in names {
            names_format
                .entry(name.clone())
                .and_modify(|formats| *formats &= possible_formats)
                .or_insert(possible_formats);
        }
    }

    // for (name, formats) in &names_format {
    //     println!("{name}: {}", formats_to_str(*formats));
    // }

    // Find the names that are written on 9/11.
    let mut nine_eleven_writers = Vec::new();
    for (date, names) in entries {
        for name in names {
            if let Some(formats) = names_format.get(name) {
                if date.is_nine_eleven(*formats) {
                    nine_eleven_writers.push(name.clone());
                }
            }
        }
    }

    nine_eleven_writers.sort();
    nine_eleven_writers.join(" ")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let entries = build(&input);

    println!("Answer: {}", nine_eleven_entries_names(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(
            nine_eleven_entries_names(&build(INPUT_TEST)),
            "Margot Peter"
        );
    }
}
