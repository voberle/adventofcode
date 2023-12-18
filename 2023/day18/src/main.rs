// https://adventofcode.com/2023/day/18

use std::{io::{self, BufRead}, collections::{VecDeque, HashSet}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    fn left(&self) -> Self {
        // coords can be negative, no boundary check needed
        Pos::new(self.row, self.col - 1)
    }

    fn right(&self) -> Self {
        Pos::new(self.row, self.col + 1)
    }

    fn up(&self) -> Self {
        Pos::new(self.row - 1, self.col)
    }

    fn down(&self) -> Self {
        Pos::new(self.row + 1, self.col)
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
            // if p == Pos::new(0, 100) {
            //     print!("\x1b[92m{}\x1b[0m", "X");
            // } else {
                print!("{}", if trench.contains(&p) { "#" } else { "." });
            // }
        }
        println!();
    }
}

struct FillItem {
    pos: Pos,
    inside: bool,
    visited: bool,
}

impl FillItem {
    fn new(pos: Pos) -> Self {
        Self { pos: pos, inside: false, visited: false }
    }

    fn inside(pos: Pos) -> Self {
        Self { pos: pos, inside: false, visited: false }
    }
}

fn trench_surface(trench: &Vec<Pos>) -> u32 {
    // Flood-fill approach
    let start = Pos::new(1, 1);
    // let start = Pos::new(0, 100);

    let trench_set: HashSet<Pos> = HashSet::from_iter(trench.iter().cloned());
    let mut filled_trench: HashSet<Pos> = HashSet::new();

    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let item = queue.pop_front().unwrap();
        // assert!(!item.visited);

        // println!("Item {:?}", item);
        filled_trench.insert(item.clone());

        for n in [item.left(), item.right(), item.up(), item.down()] {
            if !trench_set.contains(&n) && !filled_trench.contains(&n) {
                if !queue.contains(&n) {
                    // println!("  To queue {:?}", n);
                    queue.push_back(n);
                }
            }
        }
    }

    (filled_trench.len() + trench.len()) as u32
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
    print_trench(&trench);

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
