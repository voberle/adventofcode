use std::io::{self, Read};

fn ascii(c: char) -> u32 {
    c as u32
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| (ascii(c) + acc) * 17 % 256)
}

fn build_records(input: &str) -> Vec<String> {
    input.trim().split(',').map(String::from).collect()
}

fn sum_hashes(steps: &[String]) -> u32 {
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
        if let Some(label) = c.strip_suffix('-') {
            return Self::Remove(hash(label) as usize, label.to_string());
        } else if c.contains('=') {
            let v: Vec<&str> = c.split('=').collect();
            return Self::Add(hash(v[0]) as usize, v[0].to_string(), v[1].parse().unwrap());
        }
        panic!("Cannot build Operation with {}", c);
    }

    fn box_nb(&self) -> usize {
        match self {
            Operation::Remove(i, _) | Operation::Add(i, _, _) => *i,
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

fn build_operations(records: &[String]) -> Vec<Operation> {
    records.iter().map(|s| Operation::new(s)).collect()
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
            .map(|(i, lens)| box_param * (i as u64 + 1) * u64::from(lens.focal_len))
            .sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let records: Vec<String> = build_records(&input);
    println!("Part 1: {}", sum_hashes(&records));

    let operations = build_operations(&records);
    let mut boxes: [LightBox; 256] = std::array::from_fn(|_| LightBox::default());
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
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("cm"), 0);
        assert_eq!(hash("qp"), 1);
    }

    #[test]
    fn test_part1() {
        let records: Vec<String> = build_records(INPUT_TEST);
        assert_eq!(sum_hashes(&records), 1320);
    }
}
