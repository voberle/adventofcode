use std::io::{self, Read};

use fxhash::FxHashSet;

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

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn find_start(&self) -> usize {
        self.values.iter().position(|c| *c == 'S').unwrap()
    }

    #[allow(dead_code)]
    fn print_with_beam(&self, positions: &[usize]) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("|");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }
}

fn bean_split_count(manifold: &Grid) -> usize {
    let mut split_count = 0;

    let mut beam_columns: FxHashSet<usize> = FxHashSet::default();
    // let mut beam_pos: Vec<usize> = Vec::new();

    let start_pos = manifold.find_start();
    beam_columns.insert(manifold.col(start_pos));

    // Go down row by row.
    for row in 1..manifold.rows {
        let mut next_beam_columns: FxHashSet<usize> = FxHashSet::default();
        for col in beam_columns {
            let pos = manifold.pos(row, col);
            match manifold.values[pos] {
                '.' => {
                    next_beam_columns.insert(col);
                }
                '^' => {
                    next_beam_columns.insert(col - 1);
                    next_beam_columns.insert(col + 1);
                    split_count += 1;
                }
                _ => panic!("Invalid manifold char"),
            }
        }

        beam_columns = next_beam_columns;

        // beam_pos.extend(beam_columns.iter().map(|c| manifold.pos(row, *c)));
        // manifold.print_with_beam(&beam_pos);
        // println!();
    }

    split_count
}

fn part2(manifold: &Grid) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let manifold = Grid::build(&input);

    println!("Part 1: {}", bean_split_count(&manifold));
    println!("Part 2: {}", part2(&manifold));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(bean_split_count(&Grid::build(INPUT_TEST)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
