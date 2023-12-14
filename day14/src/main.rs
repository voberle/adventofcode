// https://adventofcode.com/2023/day/14
// Part 1: 113486

use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
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

    fn line(&self, row: usize) -> &[char] {
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
            println!("{}", self.line(row).iter().collect::<String>());
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
        let mut collapsed_columns: Vec<Vec<char>> = (0..self.height)
            .map(|c| collapse_down(&self.col(c)))
            .collect();
        // println!("collapsed_columns {:?}", collapsed_columns);

        let mut result = Platform::empty();
        result.width = self.width;
        result.height = self.height;
        for i in 0..self.height {
            let line = collapsed_columns.iter().map(|l| l[i]).collect::<Vec<_>>();
            // println!("LINE {:?}", line);
            result.arr.extend(line.iter());
        }
        result
    }

    fn total_load_north(&self) -> usize {
        self.collapse_north()
            .arr
            .iter()
            .enumerate()
            .filter(|(i, c)| **c == 'O')
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

fn main() {
    let stdin = io::stdin();
    let platform = Platform::build(&mut stdin.lock());
    // platform.println();

    println!("Part 1: {}", platform.total_load_north());
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_line_row() {
        let p = Platform::new("1234567890ab".chars().collect(), 4, 3);
        assert_eq!(p.line(0), "1234".chars().collect::<Vec<_>>());
        assert_eq!(p.line(1), "5678".chars().collect::<Vec<_>>());
        assert_eq!(p.line(2), "90ab".chars().collect::<Vec<_>>());
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

        assert_eq!(platform.total_load_north(), 136);
    }
}
