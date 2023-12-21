// https://adventofcode.com/2023/day/21

use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build<R>(reader: &mut R) -> Self where R: BufRead {
        let mut rows = 0;
        let values: Vec<_> = reader
            .lines()
            .filter_map(|result| result.ok())
            .map(|l| {
                rows += 1;
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        println!("{rows} {cols}");
        Self { values, rows, cols }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
       }
    }
}

#[test]
fn test_grid() {
    let input = "123\n456";
    let grid = Grid::build(&mut input.as_bytes());
    assert_eq!(grid.cols, 3);
    assert_eq!(grid.rows, 2);
    assert_eq!(grid.pos(0, 1), 1);
    assert_eq!(grid.pos(1, 2), 5);

    assert_eq!(grid.next_pos(5, North), 2);
    assert_eq!(grid.next_pos(5, West), 4);
}

fn garden_plots_count(map: &Grid, target_step_count: u32) -> u32 {
    0
}




const STEPS_COUNT_TEST: u32 = 6;

fn main() {
    let stdin = io::stdin();

    let grid = Grid::build(&mut stdin.lock());
    // println!("{:#?}", grid);

    println!("Part 1: {}", garden_plots_count(&grid, STEPS_COUNT_TEST));

}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let grid = Grid::build(&mut reader);

        assert_eq!(garden_plots_count(&grid, STEPS_COUNT_TEST), 16);
    }
}