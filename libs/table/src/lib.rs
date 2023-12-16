//! A simple implementation of a two-dimensional array using a single Vector.

use std::{fmt, io::BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table<T> where T: Clone, T: From<char> {
    pub arr: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Table<T> where T: Clone, T: From<char> {
    pub fn new(arr: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(arr.len(), width * height);
        Self { arr, width, height }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new(), 0, 0)
    }

    pub fn elt(&self, row: usize, col: usize) -> &T {
        &self.arr[row * self.width + col]
    }

    pub fn row(&self, row: usize) -> &[T] {
        let idx = row * self.width;
        &self.arr[idx..idx + self.width]
    }

    pub fn col(&self, col: usize) -> Vec<T> {
        // Much less efficient than line unfortunately
        self.arr
            .iter()
            .skip(col)
            .step_by(self.width)
            .cloned()
            .collect::<Vec<T>>()
    }

    /// Builds a Table with each table line on a separate line.
    pub fn build<R>(reader: &mut R) -> Table<T>
    where
        R: BufRead,
    {
        let mut p = Table::empty();
        for l in reader.lines() {
            let line = l.unwrap();
            p.arr.extend(line.chars().map(|c| c.into()));
            p.width = line.len();
            p.height += 1;
        }
        p
    }
}

impl<T> fmt::Display for Table<T> where T: Clone, T: From<char>, String: for<'a> FromIterator<&'a T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cols={}; Rows={}", self.height, self.width)?;
        for row in 0..self.height {
            writeln!(f, "{}", self.row(row).iter().collect::<String>())?;
        }
        Ok(())
    }
}

/// Builds a vector of Table from an input where tables are separated by a blank line.
///
/// Usage:
///
///     # use table::{build_tables, Table};
///     use std::{fs::File, io::BufReader};
///     let file = File::open("tests/files/input_test").unwrap();
///     let mut reader = BufReader::new(file);
///     let tables: Vec<Table<char>> = build_tables(&mut reader);
pub fn build_tables<R, T>(reader: &mut R) -> Vec<Table<T>>
where
    R: BufRead, T: Clone, T: From<char>
{
    let mut patterns: Vec<Table<T>> = Vec::new();
    let mut p = Table::empty();
    for l in reader.lines() {
        let line = l.unwrap();
        if line.is_empty() {
            patterns.push(p);
            p = Table::empty();
        } else {
            p.arr.extend(line.chars().map(|c| c.into()));
            p.width = line.len();
            p.height += 1;
        }
    }
    patterns.push(p); // not forgetting last one
    patterns
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_elt() {
        let p = Table::new("123456789qwertyuioasdfghjkl".chars().collect(), 9, 3);
        assert_eq!(*p.elt(0, 2), '3');
        assert_eq!(*p.elt(1, 4), 't');
    }

    #[test]
    fn test_line() {
        let p = Table::new("#.##..##...#.##.#.##......#".chars().collect(), 9, 3);
        assert_eq!(p.row(0), "#.##..##.".chars().collect::<Vec<_>>());
        assert_eq!(p.row(1), "..#.##.#.".chars().collect::<Vec<_>>());
        assert_eq!(p.row(2), "##......#".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_row() {
        let p = Table::new("#.##..##...#.##.#.##......#".chars().collect(), 9, 3);
        assert_eq!(p.col(0), "#.#".chars().collect::<Vec<_>>());
        assert_eq!(p.col(1), "..#".chars().collect::<Vec<_>>());
        assert_eq!(p.col(2), "##.".chars().collect::<Vec<_>>());
        assert_eq!(p.col(3), "#..".chars().collect::<Vec<_>>());
        assert_eq!(p.col(4), ".#.".chars().collect::<Vec<_>>());
        assert_eq!(p.col(5), ".#.".chars().collect::<Vec<_>>());
        assert_eq!(p.col(6), "#..".chars().collect::<Vec<_>>());
        assert_eq!(p.col(7), "##.".chars().collect::<Vec<_>>());
        assert_eq!(p.col(8), "..#".chars().collect::<Vec<_>>());
    }
}
