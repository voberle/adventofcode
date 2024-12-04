use std::io::{self, Read};

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    values: Vec<char>,
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
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    #[allow(dead_code)]
    fn print_with_pos(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("{RED}{c}{RESET}");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn next_positions_into(
        &self,
        pos: usize,
        d_row: isize,
        d_col: isize,
    ) -> impl Iterator<Item = usize> + '_ {
        [(d_row, d_col)]
            .into_iter()
            .map(move |(d_row, d_col)| {
                (
                    ((pos / self.cols) as isize + d_row) as usize,
                    ((pos % self.cols) as isize + d_col) as usize,
                )
            })
            .filter(|&(row, col)| (row < self.rows && col < self.cols))
            .map(|(row, col)| row * self.cols + col)
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn xmas_count(grid: &Grid) -> usize {
    grid.values
        .iter()
        .enumerate()
        .map(|(x_pos, x_val)| {
            if *x_val != 'X' {
                return 0;
            }

            [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .into_iter()
            .map(|(d_row, d_col)| {
                (
                    d_row,
                    d_col,
                    ((x_pos / grid.cols) as isize + d_row) as usize,
                    ((x_pos % grid.cols) as isize + d_col) as usize,
                )
            })
            .filter(|&(_, _, row, col)| (row < grid.rows && col < grid.cols))
            .map(|(d_row, d_col, row, col)| (d_row, d_col, row * grid.cols + col))
            .map(|(d_row, d_col, m_pos)| {
                if grid.values[m_pos] != 'M' {
                    return 0;
                }
                grid.next_positions_into(m_pos, d_row, d_col)
                    .map(|a_pos| {
                        if grid.values[a_pos] != 'A' {
                            return 0;
                        }
                        grid.next_positions_into(a_pos, d_row, d_col)
                            .map(|s_pos| {
                                #[allow(clippy::bool_to_int_with_if)]
                                if grid.values[s_pos] == 'S' {
                                    // grid.print_with_pos(&[x_pos, m_pos, a_pos, s_pos]);
                                    // println!();
                                    1
                                } else {
                                    0
                                }
                            })
                            .sum()
                    })
                    .sum()
            })
            .sum()
        })
        .sum()
}

fn part2(grid: &Grid) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", xmas_count(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(xmas_count(&Grid::build(INPUT_TEST)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
