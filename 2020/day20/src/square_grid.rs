//! A squary grid backed by a 1-d vector, with support for rotation and flipping.
//!
//! Improvement: Could made generic.

pub fn pos(cols: usize, row: usize, col: usize) -> usize {
    row * cols + col
}

fn transpose(from: &[bool], size: usize) -> Vec<bool> {
    let mut to = vec![false; from.len()];
    for i in 0..size {
        for j in 0..size {
            to[pos(size, i, j)] = from[pos(size, j, i)];
        }
    }
    to
}

fn reverse_rows(from: &[bool], size: usize) -> Vec<bool> {
    let mut to = vec![false; from.len()];
    for i in 0..size {
        for j in 0..size {
            to[pos(size, i, j)] = from[pos(size, i, size - 1 - j)];
        }
    }
    to
}

fn reverse_columns(from: &[bool], size: usize) -> Vec<bool> {
    let mut to = vec![false; from.len()];
    for j in 0..size {
        for i in 0..size {
            to[pos(size, i, j)] = from[pos(size, size - 1 - i, j)];
        }
    }
    to
}

#[derive(Clone)]
pub struct SquareGrid {
    pub values: Vec<bool>,
    pub size: usize,
}

impl SquareGrid {
    pub fn new(values: Vec<bool>, size: usize) -> Self {
        Self { values, size }
    }

    pub fn print_with_position(&self, positions: &[(usize, usize)]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.size {
            for p in row * self.size..(row + 1) * self.size {
                let c = self.values[p];
                let col = p - row * self.size;
                if positions.contains(&(row, col)) {
                    assert!(c);
                    print!("{RED}O{RESET}");
                } else {
                    print!("{}", if c { '#' } else { '.' });
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_with_position(&[]);
    }

    // Rotate by +90 degres
    fn rotate(&self) -> Self {
        let g = transpose(&self.values, self.size);
        let result = reverse_rows(&g, self.size);
        Self {
            values: result,
            size: self.size,
        }
    }

    fn flip_horizontally(&self) -> Self {
        let result = reverse_columns(&self.values, self.size);
        Self {
            values: result,
            size: self.size,
        }
    }

    fn flip_vertically(&self) -> Self {
        let result = reverse_rows(&self.values, self.size);
        Self {
            values: result,
            size: self.size,
        }
    }

    // Go through each orientation of the tile.
    // Usage: Call with `orientation` from 0 to 11.
    pub fn next_orientation(&mut self, orientation: usize) {
        match orientation % 3 {
            0 => {
                *self = self.flip_horizontally();
            }
            1 => {
                *self = self.flip_horizontally();
                *self = self.flip_vertically();
            }
            2 => {
                *self = self.flip_vertically();
                *self = self.rotate();
            }
            _ => panic!("Invalid orientation"),
        }
    }
}
