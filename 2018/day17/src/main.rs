use std::{
    fmt,
    io::{self, Read},
};

use regex::Regex;

// A line. Coordinates are ordered, [x|y]1 <= [x|y]2
#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn contains(&self, x: usize, y: usize) -> bool {
        (self.x1..=self.x2).contains(&x) && (self.y1..=self.y2).contains(&y)
    }
}

fn build(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(x|y)=(\d+).?.?(\d+)?").unwrap();
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(", ").collect();
            let p1 = re.captures(parts[0]).unwrap();
            let p2 = re.captures(parts[1]).unwrap();

            let x = if &p1[1] == "x" { &p1 } else { &p2 };
            let y = if &p1[1] == "y" { &p1 } else { &p2 };
            let mut x1 = x[2].parse::<usize>().unwrap();
            let mut x2 = x
                .get(3)
                .map_or(x1, |m| m.as_str().parse::<usize>().unwrap());
            let mut y1 = y[2].parse::<usize>().unwrap();
            let mut y2 = y
                .get(3)
                .map_or(y1, |m| m.as_str().parse::<usize>().unwrap());
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

            Line { x1, y1, x2, y2 }
        })
        .collect()
}

// min x, max x and max y (min y is 0).
fn borders(lines: &[Line]) -> (usize, usize, usize) {
    (
        lines.iter().flat_map(|l| [l.x1, l.x2]).min().unwrap(),
        lines.iter().flat_map(|l| [l.x1, l.x2]).max().unwrap(),
        lines.iter().flat_map(|l| [l.y1, l.y2]).max().unwrap(),
    )
}

fn contains(lines: &[Line], x: usize, y: usize) -> bool {
    lines.iter().any(|l| l.contains(x, y))
}

fn print(lines: &[Line]) {
    let (min_x, max_x, max_y) = borders(lines);
    for y in 0..=max_y {
        for x in min_x..=max_x {
            let c = if contains(lines, x, y) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Material {
    Sand,
    Clay,
    Water,
}
use Material::*;

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Sand => '.',
                Clay => '#',
                Water => '+',
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<Material>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(lines: &[Line]) -> Self {
        let (min_x, max_x, max_y) = borders(lines);
        // leave an empty column on each side
        let (min_x, max_x) = (min_x - 1, max_x + 2);
        let cols = max_x - min_x;
        let rows = max_y + 1;
        let mut grid = Self {
            values: vec![Sand; rows * cols],
            rows,
            cols,
        };

        for line in lines {
            if line.x1 == line.x2 {
                let x = line.x1;
                for y in line.y1..=line.y2 {
                    let p = grid.pos(x - min_x, y);
                    grid.values[p] = Clay;
                }
            } else if line.y1 == line.y2 {
                let y = line.y1;
                for x in line.x1..=line.x2 {
                    let p = grid.pos(x - min_x, y);
                    grid.values[p] = Clay;
                }
            }
        }

        let p = grid.pos(500 - min_x, 0);
        grid.values[p] = Water;

        grid
    }

    fn pos(&self, x: usize, y: usize) -> usize {
        y * self.cols + x
    }

    fn print(&self) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                print!("{}", c);
            }
            println!();
        }
    }
}

fn part1(lines: &[Line]) -> i64 {
    0
}

fn part2(lines: &[Line]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);
    // let (min_x, max_x, max_y) = borders(&lines);
    // println!("{}", (max_x - min_x) * max_y);
    // print(&lines);

    let grid = Grid::build(&lines);
    grid.print();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
