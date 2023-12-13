// https://adventofcode.com/2023/day/13
// Part 1: 27505

use std::io::{self, BufRead};

#[derive(Debug)]
struct Pattern {
    // String would be better for printing, but harder for index accessing
    arr: Vec<char>,
    width: usize,
    height: usize,
}

impl Pattern {
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
        // println!("{idx} .. {}", idx + self.width);
        &self.arr[idx..idx + self.width]
    }

    fn row(&self, col: usize) -> Vec<char> {
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
            // very ineficient :-(
            println!(
                "{}",
                self.line(row)
                    .iter()
                    .map(char::to_string)
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
    }

    fn find_vertical_reflexion(&self) -> Option<usize> {
        let mut to_check: Vec<usize> = (0..self.width - 1).collect::<Vec<_>>();
        for row in 0..self.height {
            to_check = find_reflexions_for_line(&self.line(row), &to_check);
        }
        assert!(to_check.len() <= 1);
        to_check.first().copied()
    }

    fn find_horizontal_reflexion(&self) -> Option<usize> {
        let mut to_check: Vec<usize> = (0..self.height - 1).collect::<Vec<_>>();
        for col in 0..self.width {
            to_check = find_reflexions_for_line(&self.row(col), &to_check);
        }
        assert!(to_check.len() <= 1);
        to_check.first().copied()
    }
}

// Finds a reflexion point for a line.
// Reflextion position is the item just left of the mirror.
// Not all positions need to be checked, we specify the ones to check in to_check.
fn find_reflexions_for_line(line: &[char], to_check: &[usize]) -> Vec<usize> {
    match line.len() {
        0 | 1 => return Vec::new(),
        2 => {
            if line[0] == line[1] {
                return vec![0];
            } else {
                return Vec::new();
            }
        }
        _ => {}
    }

    let mut positions: Vec<usize> = Vec::new();
    for p in to_check {
        let pos = *p;
        let mut inc = 0;
        loop {
            // if one side has passed the end, we found one point
            if pos < inc || pos + inc + 1 > line.len() - 1 {
                positions.push(pos);
                break;
            }
            let left_idx = pos - inc;
            let right_idx = pos + inc + 1;
            if line[left_idx] == line[right_idx] {
                inc += 1;
            } else {
                // this position doesn't work
                break;
            }
        }
    }
    positions
}

#[test]
fn test_line() {
    let p = Pattern::new("#.##..##...#.##.#.##......#".chars().collect(), 9, 3);
    assert_eq!(p.line(0), "#.##..##.".chars().collect::<Vec<_>>());
    assert_eq!(p.line(1), "..#.##.#.".chars().collect::<Vec<_>>());
    assert_eq!(p.line(2), "##......#".chars().collect::<Vec<_>>());
}

#[test]
fn test_row() {
    let p = Pattern::new("#.##..##...#.##.#.##......#".chars().collect(), 9, 3);
    assert_eq!(p.row(0), "#.#".chars().collect::<Vec<_>>());
    assert_eq!(p.row(1), "..#".chars().collect::<Vec<_>>());
    assert_eq!(p.row(2), "##.".chars().collect::<Vec<_>>());
    assert_eq!(p.row(3), "#..".chars().collect::<Vec<_>>());
    assert_eq!(p.row(4), ".#.".chars().collect::<Vec<_>>());
    assert_eq!(p.row(5), ".#.".chars().collect::<Vec<_>>());
    assert_eq!(p.row(6), "#..".chars().collect::<Vec<_>>());
    assert_eq!(p.row(7), "##.".chars().collect::<Vec<_>>());
    assert_eq!(p.row(8), "..#".chars().collect::<Vec<_>>());
}

#[test]
fn test_find_reflexions_for_line() {
    let line: Vec<char> = "#.##..##.".chars().collect();
    assert_eq!(
        find_reflexions_for_line(&line, &(1..line.len() - 1).collect::<Vec<_>>()),
        vec![4, 6]
    );

    let line2: Vec<char> = "#...#.##.".chars().collect();
    assert_eq!(
        find_reflexions_for_line(&line2, &(1..line2.len() - 1).collect::<Vec<_>>()),
        vec![6]
    );
}

fn build_patterns<R>(reader: &mut R) -> Vec<Pattern>
where
    R: BufRead,
{
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut p = Pattern::empty();
    for l in reader.lines() {
        let line = l.unwrap();
        if line.is_empty() {
            patterns.push(p);
            p = Pattern::empty();
        } else {
            p.arr.extend(line.chars());
            p.width = line.len();
            p.height += 1;
        }
    }
    patterns.push(p); // not forgetting last one
    patterns
}

fn find_summary(patterns: &Vec<Pattern>) -> usize {
    patterns
        .iter()
        .map(|p| {
            // p.println();
            if let Some(c) = p.find_vertical_reflexion() {
                // println!("Found vertical reflection {}", c);
                return c + 1;
            } else if let Some(r) = p.find_horizontal_reflexion() {
                // println!("Found horizontal reflection {}", r);
                return (r + 1) * 100;
            }
            // println!("No reflection found");
            0
        })
        .sum()
}

fn main() {
    let stdin = io::stdin();

    let patterns: Vec<Pattern> = build_patterns(&mut stdin.lock());
    // for p in &patterns {
    //     p.println();
    //     p.find_vertical_reflexion();
    //     println!();
    // }
    // println!("{:?}", patterns);

    println!("Part 1: {}", find_summary(&patterns));
}

#[cfg(test)]
use std::{fs::File, io::BufReader};

#[test]
fn test_part1() {
    let file = File::open("resources/input_test").unwrap();
    let mut reader = BufReader::new(file);
    let records: Vec<Pattern> = build_patterns(&mut reader);
    assert_eq!(find_summary(&records), 405);
}
