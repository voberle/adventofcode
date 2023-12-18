// https://adventofcode.com/2023/day/18

use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: char,
    meters: i64,
    color: String,
}

impl Instruction {
    fn new(direction: char, meters: i64, color: String) -> Self {
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
            color: parts[2][2..parts[2].len() - 1].to_string(),
        }
    }

    // Digs these instructions and return the new position.
    fn dig(&self, start: &Pos) -> Pos {
        match self.direction {
            'U' => Pos::new(start.row - self.meters, start.col),
            'D' => Pos::new(start.row + self.meters, start.col),
            'L' => Pos::new(start.row, start.col - self.meters),
            'R' => Pos::new(start.row, start.col + self.meters),
            _ => panic!("Invalid direction char {}", self.direction),
        }
    }

    fn invert(&self) -> Self {
        let meters = i64::from_str_radix(&self.color[0..self.color.len() - 1], 16).unwrap();
        let dir_char = &self.color.chars().last().unwrap(); //[self.color.len()-2..self.color.len()-2];
        let direction = match dir_char {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!(
                "Invalid direction char in color {}: '{}'",
                self.color, dir_char
            ),
        };
        Self::new(direction, meters, String::new())
    }
}

#[test]
fn test_instruction_dig() {
    let start = Pos::new(0, 0);
    let ins = Instruction::build("R 2 (#70c710)");
    assert_eq!(ins.dig(&start), Pos::new(0, 2))
}

#[test]
fn test_instruction_invert() {
    let ins = Instruction::build("R 2 (#70c710)");
    assert_eq!(ins.invert(), Instruction::new('R', 461937, "".to_string()));
}

// Digs the trench, and return a list of vertices.
// A vertice is a pair of coordinates representing a side of the polygon.
fn dig(dig_plan: &Vec<Instruction>) -> Vec<(Pos, Pos)> {
    let mut trench: Vec<(Pos, Pos)> = Vec::new();
    let mut current = Pos::new(0, 0);
    for ins in dig_plan {
        let next_corner = ins.dig(&current);
        trench.push((current, next_corner.clone()));
        current = next_corner;
    }
    trench
}

fn vertice_len(v: &(Pos, Pos)) -> u64 {
    assert!(v.0 != v.1);
    if v.0.col == v.1.col {
        return v.0.row.abs_diff(v.1.row);
    }
    if v.0.row == v.1.row {
        return v.0.col.abs_diff(v.1.col);
    }
    panic!("Invalid pair for vertice_len(): {:?}", v);
}

fn trench_len(trench: &[(Pos, Pos)]) -> u64 {
    trench.iter().map(vertice_len).sum()
}

fn trench_internal_surface(trench: &[(Pos, Pos)]) -> u64 {
    // Flatten the trench
    let vertices: Vec<Pos> = trench.iter().map(|pair| pair.0.clone()).collect();

    // Shoelace algorithm
    // Followed instructions at https://www.101computing.net/the-shoelace-algorithm/
    // They say that the vertices should be in anti-clockwise order, but it doesn't seem to matter.
    let nb_of_vertices = vertices.len();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..nb_of_vertices - 1 {
        sum1 += vertices[i].row * vertices[i + 1].col;
        sum2 += vertices[i].col * vertices[i + 1].row;
    }
    sum1 += vertices[nb_of_vertices - 1].row * vertices[0].col;
    sum2 += vertices[0].row * vertices[nb_of_vertices - 1].col;
    sum1.abs_diff(sum2) / 2
}

fn trench_surface(trench: &[(Pos, Pos)]) -> u64 {
    let interior_area = trench_internal_surface(trench);
    // println!("Interior area {}", interior_area);

    let boundary = trench_len(trench);
    // println!("Boundary {}", boundary);

    // Pick's theorem to add the boundaty, with small adjustment (+1 instead of -1)
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    interior_area + boundary / 2 + 1
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

    let trench: Vec<(Pos, Pos)> = dig(&dig_plan);
    // print::print_trench(&trench);
    println!("Part 1: {}", trench_surface(&trench));

    let inverted_plan: Vec<_> = dig_plan.iter().map(|i| i.invert()).collect();
    // println!("{:?}", inverted_plan);
    let trench_inverted = dig(&inverted_plan);
    println!("Part 2: {}", trench_surface(&trench_inverted));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1_and_2() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let dig_plan = build_dig_plan(&mut reader);
        let trench = dig(&dig_plan);

        assert_eq!(trench_len(&trench), 38);
        assert_eq!(trench_surface(&trench), 62);

        let inverted_plan: Vec<_> = dig_plan.iter().map(|i| i.invert()).collect();
        let trench_inverted = dig(&inverted_plan);

        assert_eq!(trench_surface(&trench_inverted), 952408144115);
    }
}

// Code used to print the trench.
// Only works for small ones of course.
mod print {
    use super::*;

    fn min_max_of_trench(trench: &[(Pos, Pos)]) -> (Pos, Pos) {
        assert!(!trench.is_empty());
        (
            Pos::new(
                trench
                    .iter()
                    .map(|(p1, p2)| i64::min(p1.row, p2.row))
                    .min()
                    .unwrap(),
                trench
                    .iter()
                    .map(|(p1, p2)| i64::min(p1.col, p2.col))
                    .min()
                    .unwrap(),
            ),
            Pos::new(
                trench
                    .iter()
                    .map(|(p1, p2)| i64::min(p1.row, p2.row))
                    .max()
                    .unwrap(),
                trench
                    .iter()
                    .map(|(p1, p2)| i64::min(p1.col, p2.col))
                    .max()
                    .unwrap(),
            ),
        )
    }

    fn is_on_vertice(pos: &Pos, v: &(Pos, Pos)) -> bool {
        assert!(v.0 != v.1);
        if v.0.col == v.1.col {
            // Vertical line
            let (a, b) = if v.0.row < v.1.row {
                (v.0.row, v.1.row)
            } else {
                (v.1.row, v.0.row)
            };
            return pos.col == v.0.col && pos.row >= a && pos.row <= b;
        }
        if v.0.row == v.1.row {
            // Horizontal line
            let (a, b) = if v.0.col < v.1.col {
                (v.0.col, v.1.col)
            } else {
                (v.1.col, v.0.col)
            };
            return pos.row == v.0.row && pos.col >= a && pos.col <= b;
        }
        false
    }

    fn trench_contains(trench: &[(Pos, Pos)], pos: &Pos) -> bool {
        trench.iter().any(|v| is_on_vertice(pos, v))
    }

    pub fn print_trench(trench: &[(Pos, Pos)]) {
        let (min, max) = min_max_of_trench(trench);
        for row in min.row..max.row + 1 {
            for col in min.col..max.col + 1 {
                let p = Pos::new(row, col);
                print!(
                    "{}",
                    if trench_contains(trench, &p) {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            println!();
        }
    }
}
