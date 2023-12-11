// https://adventofcode.com/2023/day/11
// Part 1: 9521550

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

fn build_image<R>(reader: &mut R) -> Vec<Vec<char>>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn print_image(image: &Vec<Vec<char>>) {
    println!("---");
    for (y, row) in image.iter().enumerate() {
        // print!("{y}: ");
        for (x, el) in row.iter().enumerate() {
            print!("{}", *el);
        }
        println!("");
    }
}

// From https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// any rows or columns that contain no galaxies should all actually be twice as big
fn expand_universe(image: &Vec<Vec<char>>, expansion_factor: usize) -> Vec<Vec<char>> {
    // print_image(image);
    let mut expanded_horizontally: Vec<Vec<char>> = Vec::new();
    // Expand horizontally
    for l in image {
        if l.iter().all(|c| *c == '.') {
            let big_line = vec!['.'; l.len() * expansion_factor];
            for _ in 0..expansion_factor {
                expanded_horizontally.push(big_line.clone());
            }
        } else {
            expanded_horizontally.push(l.clone());
        }
    }
    // print_image(&expanded_horizontally);

    let transposed = transpose(expanded_horizontally);
    // print_image(&transposed);

    let mut expanded_transposed: Vec<Vec<char>> = Vec::new();
    for l in transposed {
        if l.iter().all(|c| *c == '.') {
            let big_col = vec!['.'; l.len() * expansion_factor];
            for _ in 0..expansion_factor {
                expanded_transposed.push(big_col.clone());
            }
        } else {
            expanded_transposed.push(l.clone());
        }
    }
    let expanded = transpose(expanded_transposed);
    // print_image(&expanded);
    expanded
}

fn shortest_path(image: &Vec<Vec<char>>, g1: Position, g2: Position) -> usize {
    g1.x.abs_diff(g2.x) + g1.y.abs_diff(g2.y)
}

fn sum_of_shortest_paths(image: &Vec<Vec<char>>) -> usize {
    let mut galaxies: Vec<Position> = Vec::new();
    // How to do this with an iterator?
    for (y, row) in image.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            if *el == '#' {
                galaxies.push(Position::new(y, x));
            }
        }
    }
    // println!("{:#?}", galaxies);

    let mut galaxy_pairs: Vec<(Position, Position)> = Vec::new();
    for g1 in 0..galaxies.len() {
        for g2 in g1 + 1..galaxies.len() {
            galaxy_pairs.push((galaxies[g1], galaxies[g2]));
        }
    }
    // println!("{:#?}", galaxy_pairs);

    galaxy_pairs
        .iter()
        .map(|pair| shortest_path(image, pair.0, pair.1))
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let image: Vec<Vec<char>> = build_image(&mut stdin.lock());

    let expanded = expand_universe(&image, 2);

    println!("Part 1: {}", sum_of_shortest_paths(&expanded));
}

#[test]
fn test_expand_universe() {
    let mut reader = BufReader::new(File::open("resources/input_test1").unwrap());
    let image: Vec<Vec<char>> = build_image(&mut reader);

    let mut reader_expanded = BufReader::new(File::open("resources/test1_expanded").unwrap());
    let image_expanded: Vec<Vec<char>> = build_image(&mut reader_expanded);

    assert_eq!(expand_universe(&image, 2), image_expanded);
}

fn part1(filename: &str, expansion_factor: usize) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let image: Vec<Vec<char>> = build_image(&mut reader);
    let expanded = expand_universe(&image, expansion_factor);
    sum_of_shortest_paths(&expanded)
}

#[test]
fn test_part1() {
    assert_eq!(part1("resources/input_test1", 2), 374);
    assert_eq!(part1("resources/input_test1", 10), 1030);
    assert_eq!(part1("resources/input_test1", 100), 8410);
}
