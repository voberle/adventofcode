// https://adventofcode.com/2023/day/13

use std::io;
use table::{build_tables, Table};

#[derive(Debug)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

impl Reflection {
    fn summary(&self) -> usize {
        match self {
            Reflection::Vertical(col) => col + 1,
            Reflection::Horizontal(row) => (row + 1) * 100,
        }
    }
}

fn find_vertical_reflexion(table: &Table<char>, refl_to_ignore: &Option<Reflection>) -> Option<usize> {
    let mut to_check: Vec<usize> = (0..table.width - 1).collect::<Vec<_>>();
    for row in 0..table.height {
        to_check = find_reflexions_for_line(table.row(row), &to_check);
    }
    if let Some(Reflection::Vertical(refl)) = refl_to_ignore {
        to_check.iter().filter(|val| *val != refl).next().copied()
    } else {
        to_check.first().copied()
    }
}

fn find_horizontal_reflexion(table: &Table<char>, refl_to_ignore: &Option<Reflection>) -> Option<usize> {
    let mut to_check: Vec<usize> = (0..table.height - 1).collect::<Vec<_>>();
    for col in 0..table.width {
        to_check = find_reflexions_for_line(&table.col(col), &to_check);
    }
    if let Some(Reflection::Horizontal(refl)) = refl_to_ignore {
        to_check.iter().filter(|val| *val != refl).next().copied()
    } else {
        to_check.first().copied()
    }
}

fn find_reflection(table: &Table<char>) -> Option<Reflection> {
    find_reflection_with_ignore(table, &None)
}

// In part 2, the original reflection may still be valid, so we need to ignore it
// in order to find the other one always.
fn find_reflection_with_ignore(
    table: &Table<char>,
    refl_to_ignore: &Option<Reflection>,
) -> Option<Reflection> {
    if let Some(c) = find_vertical_reflexion(table, refl_to_ignore) {
        return Some(Reflection::Vertical(c));
    }
    if let Some(r) = find_horizontal_reflexion(table, refl_to_ignore) {
        return Some(Reflection::Horizontal(r));
    }
    None
}

// Finds a reflexion point for a line.
// Reflextion position is the item just left of the mirror.
// Not all positions need to be checked, we specify the ones to check in to_check.
fn find_reflexions_for_line(line: &[char], to_check: &[usize]) -> Vec<usize> {
    match line.len() {
        0 | 1 => return Vec::new(),
        2 => {
            if line[0] == line[1] {
                return vec![0];
            } else {
                return Vec::new();
            }
        }
        _ => {}
    }

    let mut positions: Vec<usize> = Vec::new();
    for p in to_check {
        let pos = *p;
        let mut inc = 0;
        loop {
            // if one side has passed the end, we found one point
            if pos < inc || pos + inc + 1 > line.len() - 1 {
                positions.push(pos);
                break;
            }
            let left_idx = pos - inc;
            let right_idx = pos + inc + 1;
            if line[left_idx] == line[right_idx] {
                inc += 1;
            } else {
                // this position doesn't work
                break;
            }
        }
    }
    positions
}

#[test]
fn test_find_reflexions_for_line() {
    let line: Vec<char> = "#.##..##.".chars().collect();
    assert_eq!(
        find_reflexions_for_line(&line, &(1..line.len() - 1).collect::<Vec<_>>()),
        vec![4, 6]
    );

    let line2: Vec<char> = "#...#.##.".chars().collect();
    assert_eq!(
        find_reflexions_for_line(&line2, &(1..line2.len() - 1).collect::<Vec<_>>()),
        vec![6]
    );
}

fn find_summary(patterns: &[Table<char>]) -> usize {
    patterns
        .iter()
        .map(|p| find_reflection(p))
        .map(|e| e.map_or(0, |r| r.summary()))
        .sum()
}

fn find_summary_with_smudges(patterns: &[Table<char>]) -> usize {
    patterns
        .iter()
        .map(|p| {
            let original_reflection = find_reflection(p);
            p.arr.iter().enumerate().find_map(|(i, smudge)| {
                let mut repaired: Table<char> = p.clone();
                repaired.arr[i] = if *smudge == '.' { '#' } else { '.' };
                find_reflection_with_ignore(&repaired, &original_reflection)
            })
        })
        .map(|o| o.unwrap().summary())
        .sum()
}

fn main() {
    let stdin = io::stdin();

    let patterns: Vec<Table<char>> = build_tables(&mut stdin.lock());
    for p in &patterns {
        println!("{}", p);
    }

    println!("Part 1: {}", find_summary(&patterns));
    println!("Part 2: {}", find_summary_with_smudges(&patterns));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_data() {
        let file = File::open("resources/input_test").unwrap();
        let mut reader = BufReader::new(file);
        let records: Vec<Table<char>> = build_tables(&mut reader);
        assert_eq!(find_summary(&records), 405);
        assert_eq!(find_summary_with_smudges(&records), 400);
    }
}
