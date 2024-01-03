use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub values: Vec<char>,
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    pub fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("{RED}{}{RESET}", c);
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }

    pub fn print(&self) {
        self.print_with_pos(&[]);
    }
}

fn lights_count<const STEPS: usize>(grid: &Grid) -> i64 {
    0
}

fn part2(grid: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);
    grid.print();

    println!("Part 1: {}", lights_count::<100>(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(lights_count::<4>(&Grid::build(INPUT_TEST)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
