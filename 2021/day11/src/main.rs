use std::io::{self, Read};

#[derive(Clone)]
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
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    #[allow(dead_code)]
    fn print(&self) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if c == 0 {
                    print!("{RED}{c}{RESET}");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    // This version gives the adjacent positions without all the direction enum code that I used to have.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn neighbors(&self, pos: usize) -> Vec<usize> {
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
        .map(move |(d_row, d_col)| {
            (
                ((pos / self.cols) as isize + d_row) as usize,
                ((pos % self.cols) as isize + d_col) as usize,
            )
        })
        .filter(|&(row, col)| (row < self.rows && col < self.cols))
        .map(|(row, col)| row * self.cols + col)
        .collect()
    }
}

fn run_step(octopuses: &mut Grid) -> usize {
    // println!("---------");
    // octopuses.print();

    // Queue of octopuses that need flashing.
    let mut flashing: Vec<usize> = Vec::new();
    // Octopuses that have flashed.
    let mut flashed: Vec<usize> = Vec::new();

    // Increase energy level of all octopuses by 1.
    octopuses
        .values
        .iter_mut()
        .enumerate()
        .for_each(|(pos, energy_level)| {
            *energy_level += 1;
            if *energy_level > 9 {
                flashing.push(pos);
            }
        });

    // Flash octopuses.
    while let Some(to_flash_pos) = flashing.pop() {
        if flashed.contains(&to_flash_pos) {
            // already flashed, skipping
            continue;
        }

        flashed.push(to_flash_pos);

        let adj_positions: Vec<usize> = octopuses.neighbors(to_flash_pos);
        for adj_pos in adj_positions {
            octopuses.values[adj_pos] += 1;
            if octopuses.values[adj_pos] > 9 {
                flashing.push(adj_pos);
            }
        }
    }

    // Set flashing octopuses level to 0.
    for pos in &flashed {
        octopuses.values[*pos] = 0;
    }

    flashed.len()
}

fn total_flashes(octopuses: &Grid, steps: usize) -> usize {
    let mut octopuses = octopuses.clone();
    (0..steps).map(|_| run_step(&mut octopuses)).sum()
}

#[allow(clippy::maybe_infinite_iter)]
fn step_when_all_flash(octopuses: &Grid) -> usize {
    let mut octopuses = octopuses.clone();
    (1..)
        .find(|_| run_step(&mut octopuses) == octopuses.values.len())
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let octopuses = Grid::build(&input);

    println!("Part 1: {}", total_flashes(&octopuses, 100));
    println!("Part 2: {}", step_when_all_flash(&octopuses));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(total_flashes(&Grid::build(INPUT_TEST), 100), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(step_when_all_flash(&Grid::build(INPUT_TEST)), 195);
    }
}
