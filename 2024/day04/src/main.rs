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
}

// Returns an iterator on the positions around this one, in the directions specified.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn next_positions<'a>(
    grid: &'a Grid,
    pos: usize,
    directions: &'a [(isize, isize)],
) -> impl Iterator<Item = (&'a isize, &'a isize, usize)> + 'a {
    directions
        .iter()
        .map(move |(d_row, d_col)| {
            (
                d_row,
                d_col,
                ((pos / grid.cols) as isize + d_row) as usize,
                ((pos % grid.cols) as isize + d_col) as usize,
            )
        })
        .filter(|&(_, _, row, col)| row < grid.rows && col < grid.cols)
        .map(|(d_row, d_col, row, col)| (d_row, d_col, row * grid.cols + col))
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn xmas_count(grid: &Grid) -> usize {
    grid.values
        .iter()
        .enumerate()
        .map(|(x_pos, x_val)| {
            // Count the XMAS as all the positions where there is an X.
            if *x_val != 'X' {
                return 0;
            }

            // For each X, we look into the 8 directions around.
            next_positions(
                grid,
                x_pos,
                &[
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ],
            )
            .map(|(d_row, d_col, m_pos)| {
                // For a valid XMAS, we need a M next to the X.
                if grid.values[m_pos] != 'M' {
                    return 0;
                }
                // Once we started looking into one direction, we remain in that same direction and check if we have A and a S.
                next_positions(grid, m_pos, &[(*d_row, *d_col)])
                    .map(|(_, _, a_pos)| {
                        if grid.values[a_pos] != 'A' {
                            return 0;
                        }
                        next_positions(grid, a_pos, &[(*d_row, *d_col)])
                            .map(|(_, _, s_pos)| usize::from(grid.values[s_pos] == 'S'))
                            .sum()
                    })
                    .sum()
            })
            .sum()
        })
        .sum()
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn x_shape_mas_count(grid: &Grid) -> usize {
    // Search all 'A' and look if there are 'M' and 'S' in the diagonals.
    grid.values
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == 'A')
        .filter(|(pos, _)| {
            [-1, 1]
                .into_iter()
                .map(move |d_col| {
                    (
                        // Get row and col for the two positions opposite the A.
                        // Top row
                        ((pos / grid.cols) as isize - 1) as usize,
                        ((pos % grid.cols) as isize + d_col) as usize,
                        // Bottom row
                        ((pos / grid.cols) as isize + 1) as usize,
                        ((pos % grid.cols) as isize - d_col) as usize,
                    )
                })
                .filter(|&(top_row, top_col, bottom_row, bottom_col)| {
                    // Filter out if any of the position is outside the grid.
                    top_row < grid.rows
                        && top_col < grid.cols
                        && bottom_row < grid.rows
                        && bottom_col < grid.cols
                })
                .filter(|(top_row, top_col, bottom_row, bottom_col)| {
                    // Convert into actual positions.
                    let pos1 = top_row * grid.cols + top_col;
                    let pos2 = bottom_row * grid.cols + bottom_col;
                    let val1 = grid.values[pos1];
                    let val2 = grid.values[pos2];
                    (val1 == 'M' && val2 == 'S') || (val1 == 'S' && val2 == 'M')
                })
                // Not using 'all', as it returns true for empty iterators. We want true only when we have two real diagonals.
                .count()
                == 2
        })
        // .inspect(|(pos, _)| {
        //     grid.print_with_pos(&[*pos]);
        //     println!();
        // })
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grid = Grid::build(&input);

    println!("Part 1: {}", xmas_count(&grid));
    println!("Part 2: {}", x_shape_mas_count(&grid));
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
        assert_eq!(x_shape_mas_count(&Grid::build(INPUT_TEST)), 9);
    }
}
