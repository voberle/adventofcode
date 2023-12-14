// https://adventofcode.com/2023/day/14
// Part 1: 113486
// Part 2: 104409

use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    arr: Vec<char>,
    width: usize,
    height: usize,
}

// TODO all the initial code below is generic, make it reusable
impl Platform {
    fn new(arr: Vec<char>, width: usize, height: usize) -> Self {
        Self { arr, width, height }
    }

    fn empty() -> Self {
        Self {
            arr: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn row(&self, row: usize) -> &[char] {
        let idx = row * self.width;
        &self.arr[idx..idx + self.width]
    }

    fn col(&self, col: usize) -> Vec<char> {
        // Much less efficient than line unfortunately
        self.arr
            .iter()
            .skip(col)
            .step_by(self.width)
            .cloned()
            .collect::<Vec<_>>()
    }

    fn println(&self) {
        println!("Cols={}; Rows={}", self.height, self.width);
        for row in 0..self.height {
            println!("{}", self.row(row).iter().collect::<String>());
        }
    }

    fn build<R>(reader: &mut R) -> Self
    where
        R: BufRead,
    {
        let mut p = Platform::empty();
        for l in reader.lines() {
            let line = l.unwrap();
            p.arr.extend(line.chars());
            p.width = line.len();
            p.height += 1;
        }
        p
    }

    fn collapse_north(&self) -> Self {
        let collapsed_columns: Vec<Vec<char>> = (0..self.height)
            .map(|c| collapse_down(&self.col(c)))
            .collect();

        let mut result = Platform::empty();
        result.width = self.width;
        result.height = self.height;
        for i in 0..self.height {
            let line = collapsed_columns.iter().map(|l| l[i]).collect::<Vec<_>>();
            result.arr.extend(line.iter());
        }
        result
    }

    fn collapse_south(&self) -> Self {
        let collapsed_columns: Vec<Vec<char>> = (0..self.height)
            .map(|c| {
                let mut col = self.col(c);
                col.reverse();
                let mut collapsed = collapse_down(&col);
                collapsed.reverse();
                collapsed
            })
            .collect();

        let mut result = Platform::empty();
        result.width = self.width;
        result.height = self.height;
        for i in 0..self.height {
            let line = collapsed_columns
                .iter()
                .map(|l| l[self.height - i - 1])
                .collect::<Vec<_>>();
            result.arr.extend(line.iter());
        }
        result
    }

    fn collapse_west(&self) -> Self {
        let collapsed_columns: Vec<Vec<char>> = (0..self.width)
            .map(|r| collapse_down(&self.row(r)))
            .collect();

        let mut result = Platform::empty();
        result.width = self.width;
        result.height = self.height;
        for i in 0..self.width {
            result.arr.extend(collapsed_columns[i].iter());
        }
        result
    }

    fn collapse_east(&self) -> Self {
        let collapsed_columns: Vec<Vec<char>> = (0..self.width)
            .map(|r| {
                let row = &mut self.row(r).to_vec();
                row.reverse();
                let mut collapsed = collapse_down(&row);
                collapsed.reverse();
                collapsed
            })
            .collect();

        let mut result = Platform::empty();
        result.width = self.width;
        result.height = self.height;
        for i in 0..self.width {
            result
                .arr
                .extend(collapsed_columns[self.width - i - 1].iter());
        }
        result
    }

    fn cycle(&self) -> Self {
        self.collapse_north()
            .collapse_west()
            .collapse_south()
            .collapse_east()
    }

    fn total_load_north(&self) -> usize {
        self.arr
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 'O')
            .map(|(i, _)| self.height - i / self.width)
            .sum()
    }
}

// Collapsing one line of rocks
fn collapse_down(s: &[char]) -> Vec<char> {
    let mut res: Vec<char> = Vec::with_capacity(s.len());

    let mut dot_cnt = 0;
    let mut O_cnt = 0;
    for c in s {
        match c {
            '#' => {
                for _ in 0..O_cnt {
                    res.push('O')
                }
                O_cnt = 0;
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
                O_cnt += 1;
            }
            _ => {}
        }
    }
    for _ in 0..O_cnt {
        res.push('O')
    }
    for _ in 0..dot_cnt {
        res.push('.')
    }
    res
}

fn cycle_nth(platform: Platform, count: usize) -> Platform {
    let mut p = platform;
    for _ in 0..count {
        p = p.cycle();
    }
    p
}

// The cycle repeats at some point, finding when.
// Returns the value and its period.
fn find_period(platform: Platform, warmup: usize) -> (usize, Platform) {
    let initial_pattern = cycle_nth(platform, warmup);
    println!("initial_pattern: {:?}", initial_pattern);
    let mut p = initial_pattern.clone();
    let mut i = 0;
    loop {
        i += 1;
        p = p.cycle();
        println!("{} cycles: {}", i, p.total_load_north());
        if p == initial_pattern {
            break;
        }
    }
    println!("Period is {} cycles: {:?}", i, p);
    (i, p)
}

fn total_load_north_after_n_cycles(platform: Platform, cycles: usize) -> usize {
    const WARMUP: usize = 100;

    let (period, mut p) = find_period(platform, WARMUP);
    // Future jump!
    // p corresponds to WARMUP, WARMUP + P, WARMUP + 2*P, etc
    let c = WARMUP + ((cycles - WARMUP) / period) * period;
    // println!("period={}, c={}", period, c);
    for _ in c..cycles {
        p = p.cycle();
    }
    p.total_load_north()
}

fn main() {
    let stdin = io::stdin();
    let platform = Platform::build(&mut stdin.lock());

    println!("Part 1: {}", platform.collapse_north().total_load_north());

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
    fn test_line_row() {
        let p = Platform::new("1234567890ab".chars().collect(), 4, 3);
        assert_eq!(p.row(0), "1234".chars().collect::<Vec<_>>());
        assert_eq!(p.row(1), "5678".chars().collect::<Vec<_>>());
        assert_eq!(p.row(2), "90ab".chars().collect::<Vec<_>>());
        assert_eq!(p.col(0), "159".chars().collect::<Vec<_>>());
        assert_eq!(p.col(1), "260".chars().collect::<Vec<_>>());
        assert_eq!(p.col(2), "37a".chars().collect::<Vec<_>>());
        assert_eq!(p.col(3), "48b".chars().collect::<Vec<_>>());
    }

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
        let platform = Platform::build(&mut reader);
        platform.println();

        let platform_collapsed = platform.collapse_north();
        platform_collapsed.println();

        let mut reader_res = BufReader::new(File::open("resources/result_test").unwrap());
        let platform_res = Platform::build(&mut reader_res);
        platform_res.println();

        assert_eq!(platform_collapsed, platform_res);

        assert_eq!(platform_collapsed.total_load_north(), 136);
    }

    #[test]
    fn test_part2() {
        let mut reader = BufReader::new(File::open("resources/input_test").unwrap());
        let platform = Platform::build(&mut reader);
        platform.println();

        let platform_1cycle = platform.cycle();
        platform_1cycle.println();

        let mut reader_res = BufReader::new(File::open("resources/result_test_1cycle").unwrap());
        let platform_res = Platform::build(&mut reader_res);
        platform_res.println();

        assert_eq!(platform_1cycle, platform_res);

        let platform_3cycle = platform.cycle().cycle().cycle();
        platform_3cycle.println();

        let mut reader_res3 = BufReader::new(File::open("resources/result_test_3cycle").unwrap());
        let platform_res3 = Platform::build(&mut reader_res3);
        platform_res.println();

        assert_eq!(platform_3cycle, platform_res3);

        assert_eq!(total_load_north_after_n_cycles(platform, 1_000_000_000), 64);
    }
}
