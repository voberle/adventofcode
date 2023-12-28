// https://adventofcode.com/2023/day/15

use std::io::{self, BufRead};

fn ascii(c: char) -> u32 {
    c as u32
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| (ascii(c) + acc) * 17 % 256)
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

// Part 2 stuff

// First value is the box number, second is the label
#[derive(Debug, PartialEq)]
enum Operation {
    Remove(usize, String),
    Add(usize, String, u32), // Last value is the focal length
}

impl Operation {
    fn new(c: &str) -> Self {
        if c.ends_with("-") {
            let label = &c[..c.len() - 1];
            return Self::Remove(hash(&label) as usize, label.to_string());
        } else if c.contains("=") {
            let v: Vec<&str> = c.split("=").collect();
            return Self::Add(
                hash(&v[0]) as usize,
                v[0].to_string(),
                v[1].parse().unwrap(),
            );
        }
        panic!("Cannot build Operation with {}", c);
    }

    fn box_nb(&self) -> usize {
        match self {
            Operation::Remove(i, _) => *i,
            Operation::Add(i, _, _) => *i,
        }
    }
}

#[test]
fn test_operation_new() {
    assert_eq!(
        Operation::new("rn=1"),
        Operation::Add(0, "rn".to_string(), 1)
    );
    assert_eq!(
        Operation::new("pc-"),
        Operation::Remove(3, "pc".to_string())
    );
}

fn build_operations(records: &Vec<String>) -> Vec<Operation> {
    records.iter().map(|s| Operation::new(&s)).collect()
}

#[derive(Debug, Default)]
struct Lens {
    label: String,
    focal_len: u32,
}

#[derive(Debug, Default)]
struct LightBox {
    lenses: Vec<Lens>,
}

impl LightBox {
    fn apply(&mut self, operation: Operation) {
        match operation {
            Operation::Remove(_, label) => {
                if let Some(index) = self.lenses.iter().position(|lens| lens.label == label) {
                    self.lenses.remove(index);
                }
            }
            Operation::Add(_, label, focal_len) => {
                let new_lens = Lens {
                    label: label.clone(),
                    focal_len,
                };
                if let Some(index) = self.lenses.iter().position(|lens| lens.label == label) {
                    if let Some(lens) = self.lenses.get_mut(index) {
                        *lens = new_lens;
                    }
                } else {
                    self.lenses.push(new_lens);
                }
            }
        }
    }

    fn focusing_power(&self, box_idx: usize) -> u64 {
        let box_param: u64 = box_idx as u64 + 1;
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| box_param * (i as u64 + 1) * lens.focal_len as u64)
            .sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let records: Vec<String> = build_records(&mut stdin.lock());
    println!("Part 1: {}", sum_hashes(&records));

    let operations = build_operations(&records);
    let mut boxes: [LightBox; 256] = std::array::from_fn(|_| Default::default());
    for op in operations {
        let idx = op.box_nb();
        boxes[idx].apply(op);
    }

    println!(
        "Part 2: {}",
        boxes
            .iter()
            .enumerate()
            .map(|(i, b)| b.focusing_power(i))
            .sum::<u64>()
    );
}

#[cfg(test)]
pub mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("cm"), 0);
        assert_eq!(hash("qp"), 1);
    }

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let records: Vec<String> = build_records(&mut reader);
        assert_eq!(sum_hashes(&records), 1320);
    }
}
