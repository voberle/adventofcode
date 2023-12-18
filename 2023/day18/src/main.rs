// https://adventofcode.com/2023/day/18

use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: char,
    meters: i32,
    color: String,
}

impl Instruction {
    fn new(direction: char, meters: i32, color: String) -> Self {
        Self {
            direction,
            meters,
            color,
        }
    }

    fn build(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        Self {
            direction: parts[0].chars().next().unwrap(),
            meters: parts[1].parse().unwrap(),
            color: parts[2].to_string(),
        }
    }

    // Digs these instructions. The new position is the last item of the result.
    fn dig(&self, start: &Pos) -> Vec<Pos> {
        let range = 1..self.meters + 1;
        match self.direction {
            'U' => range.map(|i| Pos::new(start.row - i, start.col)).collect(),
            'D' => range.map(|i| Pos::new(start.row + i, start.col)).collect(),
            'L' => range.map(|i| Pos::new(start.row, start.col - i)).collect(),
            'R' => range.map(|i| Pos::new(start.row, start.col + i)).collect(),
            _ => panic!("Invalid direction char {}", self.direction),
        }
    }
}

#[test]
fn test_instruction_dig() {
    let start = Pos::new(0, 0);
    let ins = Instruction::build("R 2 (#70c710)");
    assert_eq!(ins.dig(&start), vec![Pos::new(0, 1), Pos::new(0, 2)])
}

fn dig(dig_plan: &Vec<Instruction>) -> Vec<Pos> {
    let mut trench: Vec<Pos> = Vec::new();
    let mut current = &Pos::new(0, 0);
    for ins in dig_plan {
        trench.extend(ins.dig(current));
        current = trench.last().unwrap();
    }
    trench
}

fn min_max_of_trench(trench: &[Pos]) -> (Pos, Pos) {
    assert!(!trench.is_empty());
    (
        Pos::new(
            trench.iter().map(|p| p.row).min().unwrap(),
            trench.iter().map(|p| p.col).min().unwrap()
        ),
        Pos::new(
            trench.iter().map(|p| p.row).max().unwrap(),
            trench.iter().map(|p| p.col).max().unwrap()
        ),
    )
}

fn print_trench(trench: &[Pos]) {
    let (min, max) = min_max_of_trench(trench);
    for row in min.row..max.row + 1 {
        for col in min.col..max.col + 1 {
            let p = Pos::new(row, col);
            print!("{}", if trench.contains(&p) { "#" } else { "." });
        }
        println!();
    }
}

fn line_surface_old(limits: &[i32]) -> i32 {
    // limits is assumed to be ordered
    let mut normalized_limits: Vec<i32> = Vec::new();
    normalized_limits.push(*limits.first().unwrap());
    normalized_limits.extend(limits.windows(3).filter_map(|window| {
        if window[2] - window[0] != 2 {
            return Some(window[1]);
        }
        None
    }));
    normalized_limits.push(*limits.last().unwrap());
    println!("{:?} => {:?}", limits, normalized_limits);
    let add_extra = if normalized_limits.len() % 2 == 1 {
        normalized_limits.pop();
        1
    } else {
        0
    };
    normalized_limits.chunks(2).map(|c| c[1] - c[0] + 1).sum::<i32>() + add_extra
}

fn line_surface(limits: &[i32]) -> i32 {
    // limits is assumed to be ordered and having 2 elements at least
    assert!(limits.len() >= 2);
    let mut surface = 1;
    let mut add = true;
    let mut prev_c = limits[0];
    for c in limits.into_iter().skip(1) {
        if c - prev_c > 1 {
            // We encountered an empty section
            if add {
                surface += c - prev_c;
            }
            add ^= add;
        } else if c - prev_c == 1 {
            surface += 1;
        }
        // println!("c={c}, add={add}, surface={surface}");
        prev_c = *c;
    }
    surface
}

#[test]
fn test_line_surface() {
    // #######
    assert_eq!(line_surface(&[0, 1, 2, 3, 4, 5, 6]), 7);
    // #.....#
    assert_eq!(line_surface(&[0, 6]), 7);
    // ###...#
    assert_eq!(line_surface(&[0, 1, 2, 6]), 7);
    // ..#...#
    assert_eq!(line_surface(&[2, 6]), 5);
    // ###.###
    assert_eq!(line_surface(&[0, 1, 2, 4, 5, 6]), 7);
}

fn trench_surface(trench: &Vec<Pos>) -> u32 {
    // let mut filled_trench: Vec<Pos> = Vec::new();
    let (min, max) = min_max_of_trench(trench);
    let surface = (min.row..max.row + 1).into_iter().map(|row| {
        // Collect the walls on this line
        let mut limits: Vec<i32> = Vec::new();
        for col in min.col..max.col + 1 {
            let p = Pos::new(row, col);
            if trench.contains(&p) {
                limits.push(col);
            }
        }
        let s = line_surface(&limits);
        // println!("{}: {:?} => {}", row, limits, s);
        s
    })
    .sum::<i32>();
    assert!(surface > 0);
    surface as u32
}

fn build_dig_plan<R>(reader: &mut R) -> Vec<Instruction>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            Instruction::build(&line)
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let dig_plan = build_dig_plan(&mut stdin.lock());
    // println!("{:?}", dig_plan);

    let trench = dig(&dig_plan);
    // print_trench(&trench);

    println!("Part 1: {}", trench_surface(&trench));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let dig_plan = build_dig_plan(&mut reader);
        let trench = dig(&dig_plan);
        assert_eq!(trench_surface(&trench), 62);
    }
}
