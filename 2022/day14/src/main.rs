use std::io::{self, Read};

use itertools::Itertools;

// Helper method to parse the input, before building the grid.
fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|c| {
                    c.split(',')
                        .map(|v| v.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn borders(coords: &[Vec<(usize, usize)>]) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;
    for &(x, y) in coords.iter().flatten() {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    (min_x, max_x, min_y, max_y)
}

struct Cave {
    rocks: Vec<bool>,
    rows: usize,
    cols: usize,
    min_x: usize,
}

impl Cave {
    fn build(input: &str) -> Self {
        let coords = parse_input(input);
        let (min_x, max_x, _, max_y) = borders(&coords);
        // Top starts at 0.
        let rows = max_y + 1;
        let cols = max_x - min_x + 1;
        let mut rocks = vec![false; rows * cols];

        for line in coords {
            for start_end in line.windows(2) {
                if start_end[0].0 == start_end[1].0 {
                    // x is same, so vertical line.
                    let x = start_end[0].0 - min_x;
                    let (y1, y2) = if start_end[0].1 < start_end[1].1 {
                        (start_end[0].1, start_end[1].1)
                    } else {
                        (start_end[1].1, start_end[0].1)
                    };
                    for y in y1..=y2 {
                        rocks[y * cols + x] = true;
                    }
                } else if start_end[0].1 == start_end[1].1 {
                    // y is same, so horizontal line.
                    let y = start_end[0].1;
                    let (x1, x2) = if start_end[0].0 < start_end[1].0 {
                        (start_end[0].0, start_end[1].0)
                    } else {
                        (start_end[1].0, start_end[0].0)
                    };
                    for x in x1..=x2 {
                        let x = x - min_x;
                        rocks[y * cols + x] = true;
                    }
                } else {
                    panic!("Diagonal lines not supported")
                }
            }
        }
        Self {
            rocks,
            rows,
            cols,
            min_x,
        }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn get_entry(&self) -> usize {
        // 500,0
        500 - self.min_x
    }

    fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const BLUE: &str = "\x1b[94m";
        const RESET: &str = "\x1b[0m";
        let entry = self.get_entry();
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                if p == entry {
                    print!("{BLUE}+{RESET}");
                } else if positions.contains(&p) {
                    print!("{RED}O{RESET}");
                } else {
                    let c = self.rocks[p];
                    print!("{}", if c { '#' } else { '.' });
                }
            }
            println!();
        }
    }

    fn print(&self) {
        self.print_with_pos(&[]);
    }
}

fn sand_count_before_abyss(cave: &Cave) -> usize {
    0
}

fn part2(cave: &Cave) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cave = Cave::build(&input);
    cave.print();

    println!("Part 1: {}", sand_count_before_abyss(&cave));
    println!("Part 2: {}", part2(&cave));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(sand_count_before_abyss(&Cave::build(INPUT_TEST)), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Cave::build(INPUT_TEST)), 0);
    }
}
