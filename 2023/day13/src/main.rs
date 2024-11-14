use std::{
    fmt,
    io::{self, Read},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Table<T>
where
    T: Clone,
    T: From<char>,
{
    arr: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Table<T>
where
    T: Clone,
    T: From<char>,
{
    fn new(arr: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(arr.len(), width * height);
        Self { arr, width, height }
    }

    fn empty() -> Self {
        Self::new(Vec::new(), 0, 0)
    }

    #[allow(dead_code)]
    fn elt(&self, row: usize, col: usize) -> &T {
        &self.arr[row * self.width + col]
    }

    fn row(&self, row: usize) -> &[T] {
        let idx = row * self.width;
        &self.arr[idx..idx + self.width]
    }

    fn col(&self, col: usize) -> Vec<T> {
        // Much less efficient than line unfortunately
        self.arr
            .iter()
            .skip(col)
            .step_by(self.width)
            .cloned()
            .collect::<Vec<T>>()
    }

    /// Builds a Table with each table line on a separate line.
    #[allow(dead_code)]
    fn build(input: &str) -> Table<T> {
        let mut p = Table::empty();
        for line in input.lines() {
            p.arr.extend(line.chars().map(std::convert::Into::into));
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

fn build_tables<T>(input: &str) -> Vec<Table<T>>
where
    T: Clone,
    T: From<char>,
{
    let mut patterns: Vec<Table<T>> = Vec::new();
    let mut p = Table::empty();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(p);
            p = Table::empty();
        } else {
            p.arr.extend(line.chars().map(std::convert::Into::into));
            p.width = line.len();
            p.height += 1;
        }
    }
    patterns.push(p); // not forgetting last one
    patterns
}

#[cfg(test)]
mod tests_table {
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

fn find_vertical_reflexion(
    table: &Table<char>,
    refl_to_ignore: &Option<Reflection>,
) -> Option<usize> {
    let mut to_check: Vec<usize> = (0..table.width - 1).collect::<Vec<_>>();
    for row in 0..table.height {
        to_check = find_reflexions_for_line(table.row(row), &to_check);
    }
    if let Some(Reflection::Vertical(refl)) = refl_to_ignore {
        to_check.iter().find(|val| *val != refl).copied()
    } else {
        to_check.first().copied()
    }
}

fn find_horizontal_reflexion(
    table: &Table<char>,
    refl_to_ignore: &Option<Reflection>,
) -> Option<usize> {
    let mut to_check: Vec<usize> = (0..table.height - 1).collect::<Vec<_>>();
    for col in 0..table.width {
        to_check = find_reflexions_for_line(&table.col(col), &to_check);
    }
    if let Some(Reflection::Horizontal(refl)) = refl_to_ignore {
        to_check.iter().find(|val| *val != refl).copied()
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
            }
            return Vec::new();
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
        .map(find_reflection)
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let patterns: Vec<Table<char>> = build_tables(&input);
    // for p in &patterns {
    //     println!("{}", p);
    // }

    println!("Part 1: {}", find_summary(&patterns));
    println!("Part 2: {}", find_summary_with_smudges(&patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_data() {
        let records: Vec<Table<char>> = build_tables(INPUT_TEST);
        assert_eq!(find_summary(&records), 405);
        assert_eq!(find_summary_with_smudges(&records), 400);
    }
}
