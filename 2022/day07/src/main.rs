use std::{
    collections::VecDeque,
    io::{self, Read},
};

use itertools::Itertools;

#[derive(Debug)]
enum Entry {
    File(String, u32),
    Dir(String, Vec<Entry>),
}

impl Entry {
    fn build(current_dir: &str, lines: &mut VecDeque<String>) -> Self {
        let mut entries = Vec::new();
        while let Some(line) = lines.pop_front() {
            if line == "$ cd /" || line == "$ ls" || line.starts_with("dir ") {
                // Ignoring those, we don't need them.
            } else if line == "$ cd .." {
                break;
            } else if let Some(dir) = line.strip_prefix("$ cd ") {
                let d = Entry::build(dir, lines);
                entries.push(d);
            } else {
                let (size, file) = line.split_ascii_whitespace().collect_tuple().unwrap();
                // println!("Creating file {} ({})", file, size);
                let f = Entry::File(file.to_string(), size.parse().unwrap());
                entries.push(f);
            }
        }
        // println!("Creating dir {} with: {:?}", current_dir, entries);
        Entry::Dir(current_dir.to_string(), entries)
    }
}

fn build(input: &str) -> Entry {
    let mut lines: VecDeque<String> = input.lines().map(ToString::to_string).collect();
    assert_eq!(lines[0], "$ cd /");
    Entry::build("/", &mut lines)
}

fn dir_sizes(entry: &Entry, sizes: &mut Vec<(String, u32)>) -> u32 {
    match entry {
        Entry::File(_, size) => *size,
        Entry::Dir(name, list) => {
            let s = list.iter().map(|e| dir_sizes(e, sizes)).sum();
            sizes.push((name.to_string(), s));
            s
        }
    }
}

fn sum_dir_most_100000(fs: &Entry) -> u32 {
    let mut sizes = Vec::new();
    dir_sizes(fs, &mut sizes);
    sizes
        .iter()
        .filter_map(|(_, s)| if *s <= 100_000 { Some(s) } else { None })
        .sum()
}

fn part2(fs: &Entry) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let fs = build(&input);

    println!("Part 1: {}", sum_dir_most_100000(&fs));
    println!("Part 2: {}", part2(&fs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(sum_dir_most_100000(&build(INPUT_TEST)), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
