use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars()
                    .filter(|c| !c.is_ascii_whitespace())
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

fn safest_danger_level(grid: &Grid) -> u32 {
    let min_horizontal_danger: u32 = (0..grid.rows)
        .map(|row| {
            (0..grid.cols)
                .map(|col| grid.values[grid.pos(row, col)])
                .sum()
        })
        .min()
        .unwrap();
    let min_vertical_danger: u32 = (0..grid.cols)
        .map(|col| {
            (0..grid.rows)
                .map(|row| grid.values[grid.pos(row, col)])
                .sum()
        })
        .min()
        .unwrap();
    min_horizontal_danger.min(min_vertical_danger)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", safest_danger_level(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let grid = Grid::build(&INPUT_TEST);
        assert_eq!(safest_danger_level(&grid), 73);
    }
}
