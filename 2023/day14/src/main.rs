// https://adventofcode.com/2023/day/14

use std::io;
use std::{fmt, io::BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table<T>
where
    T: Clone,
    T: From<char>,
{
    pub arr: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Table<T>
where
    T: Clone,
    T: From<char>,
{
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

impl<T> fmt::Display for Table<T>
where
    T: Clone,
    T: From<char>,
    String: for<'a> FromIterator<&'a T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cols={}; Rows={}", self.height, self.width)?;
        for row in 0..self.height {
            writeln!(f, "{}", self.row(row).iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn build_tables<R, T>(reader: &mut R) -> Vec<Table<T>>
where
    R: BufRead,
    T: Clone,
    T: From<char>,
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

fn collapse_north(table: &Table<char>) -> Table<char> {
    let collapsed_columns: Vec<Vec<char>> = (0..table.height)
        .map(|c| collapse_down(&table.col(c)))
        .collect();

    let mut result = Table::empty();
    result.width = table.width;
    result.height = table.height;
    for i in 0..table.height {
        let line = collapsed_columns.iter().map(|l| l[i]).collect::<Vec<_>>();
        result.arr.extend(line.iter());
    }
    result
}

fn collapse_south(table: &Table<char>) -> Table<char> {
    let collapsed_columns: Vec<Vec<char>> = (0..table.height)
        .map(|c| {
            let mut col = table.col(c);
            col.reverse();
            let mut collapsed = collapse_down(&col);
            collapsed.reverse();
            collapsed
        })
        .collect();

    let mut result = Table::empty();
    result.width = table.width;
    result.height = table.height;
    for i in 0..table.height {
        let line = collapsed_columns
            .iter()
            .map(|l| l[table.height - i - 1])
            .collect::<Vec<_>>();
        result.arr.extend(line.iter());
    }
    result
}

fn collapse_west(table: &Table<char>) -> Table<char> {
    let collapsed_columns: Vec<Vec<char>> = (0..table.width)
        .map(|r| collapse_down(table.row(r)))
        .collect();

    let mut result = Table::empty();
    result.width = table.width;
    result.height = table.height;
    for item in collapsed_columns.iter().take(table.width) {
        result.arr.extend(item);
    }
    result
}

fn collapse_east(table: &Table<char>) -> Table<char> {
    let collapsed_columns: Vec<Vec<char>> = (0..table.width)
        .map(|r| {
            let row = &mut table.row(r).to_vec();
            row.reverse();
            let mut collapsed = collapse_down(row);
            collapsed.reverse();
            collapsed
        })
        .collect();

    let mut result = Table::empty();
    result.width = table.width;
    result.height = table.height;
    for i in 0..table.width {
        result
            .arr
            .extend(collapsed_columns[table.width - i - 1].iter());
    }
    result
}

fn cycle(table: &Table<char>) -> Table<char> {
    let mut t = collapse_north(table);
    t = collapse_west(&t);
    t = collapse_south(&t);
    collapse_east(&t)
}

fn total_load_north(table: &Table<char>) -> usize {
    table
        .arr
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'O')
        .map(|(i, _)| table.height - i / table.width)
        .sum()
}

// Collapsing one line of rocks
fn collapse_down(s: &[char]) -> Vec<char> {
    let mut res: Vec<char> = Vec::with_capacity(s.len());

    let mut dot_cnt = 0;
    let mut o_cnt = 0;
    for c in s {
        match c {
            '#' => {
                for _ in 0..o_cnt {
                    res.push('O')
                }
                o_cnt = 0;
                for _ in 0..dot_cnt {
                    res.push('.')
                }
                dot_cnt = 0;
                res.push(*c);
            }
            '.' => {
                dot_cnt += 1;
            }
            'O' => {
                o_cnt += 1;
            }
            _ => {}
        }
    }
    for _ in 0..o_cnt {
        res.push('O')
    }
    for _ in 0..dot_cnt {
        res.push('.')
    }
    res
}

fn cycle_nth(platform: Table<char>, count: usize) -> Table<char> {
    let mut p = platform;
    for _ in 0..count {
        p = cycle(&p);
    }
    p
}

// The cycle repeats at some point, finding when.
// Returns the value and its period.
fn find_period(platform: Table<char>, warmup: usize) -> (usize, Table<char>) {
    let initial_pattern = cycle_nth(platform, warmup);
    println!("initial_pattern: {:?}", initial_pattern);
    let mut p = initial_pattern.clone();
    let mut i = 0;
    loop {
        i += 1;
        p = cycle(&p);
        println!("{} cycles: {}", i, total_load_north(&p));
        if p == initial_pattern {
            break;
        }
    }
    println!("Period is {} cycles: {:?}", i, p);
    (i, p)
}

fn total_load_north_after_n_cycles(platform: Table<char>, cycles: usize) -> usize {
    const WARMUP: usize = 100;

    let (period, mut p) = find_period(platform, WARMUP);
    // Future jump!
    // p corresponds to WARMUP, WARMUP + P, WARMUP + 2*P, etc
    let c = WARMUP + ((cycles - WARMUP) / period) * period;
    // println!("period={}, c={}", period, c);
    for _ in c..cycles {
        p = cycle(&p);
    }
    total_load_north(&p)
}

fn main() {
    let stdin = io::stdin();
    let platform = Table::build(&mut stdin.lock());

    println!("Part 1: {}", total_load_north(&collapse_north(&platform)));

    println!(
        "Part 2: {}",
        total_load_north_after_n_cycles(platform, 1_000_000_000)
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_collapse_down() {
        assert_eq!(
            collapse_down(&"..#...O.#.".chars().collect::<Vec<_>>()),
            "..#O....#.".chars().collect::<Vec<_>>()
        );
        assert_eq!(
            collapse_down(&"..O...O.".chars().collect::<Vec<_>>()),
            "OO......".chars().collect::<Vec<_>>()
        );
        assert_eq!(
            collapse_down(&"OO.O.O..##".chars().collect::<Vec<_>>()),
            "OOOO....##".chars().collect::<Vec<_>>()
        );
        assert_eq!(
            collapse_down(&".#.O.#O...".chars().collect::<Vec<_>>()),
            ".#O..#O...".chars().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let platform = Table::build(&mut reader);
        println!("{}", platform);

        let platform_collapsed = collapse_north(&platform);
        println!("{}", platform_collapsed);

        let mut reader_res = BufReader::new(File::open("resources/result_test").unwrap());
        let platform_res = Table::build(&mut reader_res);
        println!("{}", platform_res);

        assert_eq!(platform_collapsed, platform_res);

        assert_eq!(total_load_north(&platform_collapsed), 136);
    }

    #[test]
    fn test_part2() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let platform = Table::build(&mut reader);
        println!("{}", platform);

        let platform_1cycle = cycle(&platform);
        println!("{}", platform_1cycle);

        let mut reader_res = BufReader::new(File::open("resources/result_test_1cycle").unwrap());
        let platform_res = Table::build(&mut reader_res);
        println!("{}", platform_res);

        assert_eq!(platform_1cycle, platform_res);

        let mut platform_3cycle = cycle(&platform);
        platform_3cycle = cycle(&platform_3cycle);
        platform_3cycle = cycle(&platform_3cycle);
        println!("{}", platform_3cycle);

        let mut reader_res3 = BufReader::new(File::open("resources/result_test_3cycle").unwrap());
        let platform_res3 = Table::build(&mut reader_res3);
        println!("{}", platform_res);

        assert_eq!(platform_3cycle, platform_res3);

        assert_eq!(total_load_north_after_n_cycles(platform, 1_000_000_000), 64);
    }
}
