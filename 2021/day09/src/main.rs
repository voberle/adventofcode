use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

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
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print_with_pos(&self, positions: &[usize]) {
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

    fn print(&self) {
        self.print_with_pos(&[]);
    }

    // To explore the grid column by column:
    // for col in 0..grid.cols {
    //     for p in (col..(col + grid.cols * grid.rows)).step_by(grid.cols) {
    // To get the next row element in a column:
    //         let p1 = p + grid.cols;

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    fn pos_as_str(&self, index: usize) -> String {
        format!("({},{})", self.row(index), self.col(index))
    }

    // Check we don't go outside grid.
    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `allowed`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    // Returns the up to 4 adjacent positions.
    fn adjacent_pos(&self, pos: usize) -> Vec<usize> {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|d| {
                if self.allowed(pos, *d) {
                    Some(self.next_pos(pos, *d))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn sum_risk_levels(heightmap: &Grid) -> u32 {
    heightmap
        .values
        .iter()
        .enumerate()
        .filter(|(pos, value)| {
            let adjacent_pos = heightmap.adjacent_pos(*pos);
            adjacent_pos.iter().all(|p| **value < heightmap.values[*p])
        })
        .map(|(_, value)| 1 + value)
        .sum()
}

fn part2(heightmap: &Grid) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let heightmap = Grid::build(&input);

    println!("Part 1: {}", sum_risk_levels(&heightmap));
    println!("Part 2: {}", part2(&heightmap));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(sum_risk_levels(&Grid::build(INPUT_TEST)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
