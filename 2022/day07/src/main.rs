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

fn make_dir_sizes(entry: &Entry, sizes: &mut Vec<(String, u32)>) -> u32 {
    match entry {
        Entry::File(_, size) => *size,
        Entry::Dir(name, list) => {
            let s = list.iter().map(|e| make_dir_sizes(e, sizes)).sum();
            sizes.push((name.to_string(), s));
            s
        }
    }
}

// Returns the list of directory => sizes.
// We don't really need the directory name, but it helps debugging.
fn dir_sizes(fs: &Entry) -> Vec<(String, u32)> {
    let mut sizes = Vec::new();
    make_dir_sizes(fs, &mut sizes);
    // for (n, s) in sizes {
    //     println!("{}: {}", n, s);
    // }
    sizes
}

fn sum_dir_most_100000(sizes: &[(String, u32)]) -> u32 {
    sizes
        .iter()
        .filter_map(|(_, s)| if *s <= 100_000 { Some(s) } else { None })
        .sum()
}

fn dir_size_to_delete(sizes: &[(String, u32)]) -> u32 {
    const FS_SIZE: u32 = 70_000_000;
    const UPDATE_REQUIRED_SIZE: u32 = 30_000_000;

    let last = sizes.last().unwrap();
    assert_eq!(last.0, "/");

    let used_space = last.1;
    let unused_space = FS_SIZE - used_space;
    let extra_needed = UPDATE_REQUIRED_SIZE - unused_space;

    sizes
        .iter()
        .map(|(_, s)| *s)
        .filter(|s| *s >= extra_needed)
        .min()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let fs = build(&input);

    let sizes = dir_sizes(&fs);

    println!("Part 1: {}", sum_dir_most_100000(&sizes));
    println!("Part 2: {}", dir_size_to_delete(&sizes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let sizes = dir_sizes(&build(INPUT_TEST));
        assert_eq!(sum_dir_most_100000(&sizes), 95437);
    }

    #[test]
    fn test_part2() {
        let sizes = dir_sizes(&build(INPUT_TEST));
        assert_eq!(dir_size_to_delete(&sizes), 24933642);
    }
}
