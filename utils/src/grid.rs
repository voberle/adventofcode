/// A 2D grid backed by a simple Vector
///
/// For a grid looking at the 8 adjacent directions, check 2015/day18/src/main.rs.
///
use crate::direction::Direction::{self, East, North, South, West};

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
                l.chars()
                    // .map(|c| c)
                    .collect::<Vec<_>>()
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
                    print!("{RED}{c}{RESET}");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    pub fn print(&self) {
        self.print_with_pos(&[]);
    }

    // To explore the grid column by column:
    // for col in 0..grid.cols {
    //     for p in (col..(col + grid.cols * grid.rows)).step_by(grid.cols) {
    // To get the next row element in a column:
    //         let p1 = p + grid.cols;

    pub fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    pub fn row(&self, index: usize) -> usize {
        index / self.cols
    }

    pub fn pos_as_str(&self, index: usize) -> String {
        format!("({},{})", self.row(index), self.col(index))
    }

    // Check we don't go outside grid.
    pub fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            North => pos < self.cols,
            East => pos % self.cols == self.cols - 1,
            South => pos / self.cols == self.rows - 1,
            West => pos % self.cols == 0,
        }
    }

    // Returns the index of the next position in that direction.
    // Assumes validity of the move has been checked before with `allowed`.
    pub fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            North => pos - self.cols,
            East => pos + 1,
            South => pos + self.cols,
            West => pos - 1,
        }
    }

    pub fn try_next_pos(&self, pos: usize, direction: Direction) -> Option<usize> {
        if self.allowed(pos, direction) {
            Some(self.next_pos(pos, direction))
        } else {
            None
        }
    }

    // Gives the 8 adjacent positions without all the direction enum stuff.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn neighbors(&self, pos: usize) -> Vec<usize> {
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

    // Same as above, but as an iterator.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn next_positions_iter(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = "123\n456";
        let grid = Grid::build(input);
        assert_eq!(grid.cols, 3);
        assert_eq!(grid.rows, 2);
        assert_eq!(grid.pos(0, 1), 1);
        assert_eq!(grid.pos(1, 2), 5);
        assert_eq!(grid.row(5), 1);
        assert_eq!(grid.col(5), 2);
        assert_eq!(grid.row(1), 0);
        assert_eq!(grid.col(1), 1);

        assert!(grid.allowed(5, North));
        assert_eq!(grid.next_pos(5, North), 2);
        assert!(grid.allowed(5, West));
        assert_eq!(grid.next_pos(5, West), 4);
        assert!(!grid.allowed(5, East));
        assert!(!grid.allowed(5, South));
    }
}
