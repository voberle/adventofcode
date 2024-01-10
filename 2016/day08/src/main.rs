use std::io::{self, Read};

use regex::Regex;

enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, amount: usize },
    RotateColumn { col: usize, amount: usize },
}
use Instruction::*;

fn build(input: &str) -> Vec<Instruction> {
    let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_rotate = Regex::new(r"rotate (column x|row y)=(\d+) by (\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            if line.starts_with("rect") {
                let parts = re_rect.captures(line).unwrap();
                Rect {
                    width: parts[1].parse().unwrap(),
                    height: parts[2].parse().unwrap(),
                }
            } else if line.starts_with("rotate") {
                let parts = re_rotate.captures(line).unwrap();
                match &parts[1] {
                    "column x" => RotateColumn {
                        col: parts[2].parse().unwrap(),
                        amount: parts[3].parse().unwrap(),
                    },
                    "row y" => RotateRow {
                        row: parts[2].parse().unwrap(),
                        amount: parts[3].parse().unwrap(),
                    },
                    _ => panic!("Invalid input"),
                }
            } else {
                panic!("Invalid input")
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Screen {
    values: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl Screen {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            values: vec![false; rows * cols],
            rows,
            cols,
        }
    }

    #[cfg(test)]
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().map(|c| c == '#').collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn format(&self) -> String {
        let mut s = String::with_capacity(self.rows * self.cols);
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                s += &format!("{}", if c { '#' } else { '.' });
            }
            s += "\n";
        }
        s.trim_end().to_string()
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

fn draw_rect(screen: &mut Screen, width: usize, height: usize) {
    assert!(width < screen.cols && height < screen.rows);
    for r in 0..height {
        for c in 0..width {
            let i = screen.pos(r, c);
            screen.values[i] = true;
        }
    }
}

// From utils
const fn wrapping_index(i: usize, len: usize) -> usize {
    (i % len + len) % len
}

fn copy_and_clear_row(screen: &mut Screen, row: usize) -> Vec<bool> {
    let mut full_row = Vec::with_capacity(screen.cols);
    for i in 0..screen.cols {
        let i = screen.pos(row, i);
        full_row.push(screen.values[i]);
        screen.values[i] = false;
    }
    full_row
}

fn rotate_row(screen: &mut Screen, row: usize, amount: usize) {
    let existing_row = copy_and_clear_row(screen, row);
    for (i, existing) in existing_row.iter().enumerate().take(screen.cols) {
        let wrapping_idx = screen.pos(row, wrapping_index(i + amount, screen.cols));
        screen.values[wrapping_idx] = *existing;
    }
}

fn copy_and_clear_col(screen: &mut Screen, col: usize) -> Vec<bool> {
    let mut full_col = Vec::with_capacity(screen.rows);
    for i in 0..screen.rows {
        let i = screen.pos(i, col);
        full_col.push(screen.values[i]);
        screen.values[i] = false;
    }
    full_col
}

fn rotate_col(screen: &mut Screen, col: usize, amount: usize) {
    let existing_col = copy_and_clear_col(screen, col);
    for (i, existing) in existing_col.iter().enumerate().take(screen.rows) {
        let wrapping_idx = screen.pos(wrapping_index(i + amount, screen.rows), col);
        screen.values[wrapping_idx] = *existing;
    }
}

fn process_instruction(screen: &mut Screen, instruction: &Instruction) {
    match instruction {
        Rect { width, height } => draw_rect(screen, *width, *height),
        RotateRow { row, amount } => rotate_row(screen, *row, *amount),
        RotateColumn { col, amount } => rotate_col(screen, *col, *amount),
    }
}

fn lit_pixels_count<const SCREEN_WIDTH: usize, const SCREEN_HEIGHT: usize>(
    instructions: &[Instruction],
) -> usize {
    let mut screen = Screen::new(SCREEN_HEIGHT, SCREEN_WIDTH);
    for i in instructions {
        process_instruction(&mut screen, i);
    }
    // println!("{}", screen.format());
    screen.values.iter().filter(|&&v| v).count()
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", lit_pixels_count::<50, 6>(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_draw_rect() {
        let mut screen = Screen::new(3, 7);
        draw_rect(&mut screen, 3, 2);
        assert_eq!(
            screen.format(),
            r"###....
###....
......."
        );
    }

    #[test]
    fn test_rotate_row() {
        let mut screen = Screen::build(
            r"#.#....
###....
.#.....",
        );
        rotate_row(&mut screen, 0, 4);
        assert_eq!(
            screen.format(),
            r"....#.#
###....
.#....."
        );
    }

    #[test]
    fn test_rotate_col() {
        let mut screen = Screen::build(
            r"....#.#
###....
.#.....",
        );
        rotate_col(&mut screen, 1, 1);
        assert_eq!(
            screen.format(),
            r".#..#.#
#.#....
.#....."
        );
    }

    #[test]
    fn test_wrapping_index() {
        assert_eq!(wrapping_index(0, 6), 0);
        assert_eq!(wrapping_index(3, 6), 3);
        assert_eq!(wrapping_index(6, 6), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(lit_pixels_count::<7, 3>(&build(INPUT_TEST)), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
