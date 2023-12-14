// https://adventofcode.com/2023/day/14

use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Platform {
    arr: Vec<char>,
    width: usize,
    height: usize,
}

// TODO all the initial code below is generic, make it reusable
impl Platform {
    fn new(arr: Vec<char>, width: usize, height: usize) -> Self {
        Self { arr, width, height }
    }

    fn empty() -> Self {
        Self {
            arr: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn line(&self, row: usize) -> &[char] {
        let idx = row * self.width;
        &self.arr[idx..idx + self.width]
    }

    fn col(&self, col: usize) -> Vec<char> {
        // Much less efficient than line unfortunately
        self.arr
            .iter()
            .skip(col)
            .step_by(self.width)
            .cloned()
            .collect::<Vec<_>>()
    }

    fn println(&self) {
        println!("Cols={}; Rows={}", self.height, self.width);
        for row in 0..self.height {
            println!("{}", self.line(row).iter().collect::<String>());
        }
    }

    fn build<R>(reader: &mut R) -> Self
    where
        R: BufRead,
    {
        let mut p = Platform::empty();
        for l in reader.lines() {
            let line = l.unwrap();
            p.arr.extend(line.chars());
            p.width = line.len();
            p.height += 1;
        }
        p
    }

    fn total_load_north(&self) -> u32 {
        0
    }
}

#[test]
fn test_line_row() {
    let p = Platform::new("1234567890ab".chars().collect(), 4, 3);
    assert_eq!(p.line(0), "1234".chars().collect::<Vec<_>>());
    assert_eq!(p.line(1), "5678".chars().collect::<Vec<_>>());
    assert_eq!(p.line(2), "90ab".chars().collect::<Vec<_>>());
    assert_eq!(p.col(0), "159".chars().collect::<Vec<_>>());
    assert_eq!(p.col(1), "260".chars().collect::<Vec<_>>());
    assert_eq!(p.col(2), "37a".chars().collect::<Vec<_>>());
    assert_eq!(p.col(3), "48b".chars().collect::<Vec<_>>());
}

fn main() {
    let stdin = io::stdin();
    let platform = Platform::build(&mut stdin.lock());
    platform.println();

    println!("Part 1: {}", platform.total_load_north());
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    fn part1(filename: &str) -> u32 {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let platform = Platform::build(&mut reader);
        platform.total_load_north()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("resources/input_test"), 136);
    }
}
