// https://adventofcode.com/2023/day/15

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn ascii(c: char) -> u32 {
    c as u32
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| (ascii(c) + acc) * 17 % 256)
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
}

fn build_records<R>(reader: &mut R) -> Vec<String>
where
    R: BufRead,
{
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    s.trim().split(",").map(String::from).collect()
}

fn sum_hashes(steps: &Vec<String>) -> u32 {
    steps.iter().map(|s| hash(s)).sum()
}

fn main() {
    let stdin = io::stdin();
    let records: Vec<String> = build_records(&mut stdin.lock());

    println!("Part 1: {}", sum_hashes(&records));
}

#[test]
fn test_part1() {
    let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
    let records: Vec<String> = build_records(&mut reader);
    assert_eq!(sum_hashes(&records), 1320);
}
