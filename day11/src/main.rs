// https://adventofcode.com/2023/day/11
// Part 1: 

use std::{io::{self, BufRead, BufReader}, fs::File};


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
fn expand_universe(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // print_image(image);
    let mut expanded_horizontally: Vec<Vec<char>> = Vec::new();
    // Expand horizontally
    for l in image {
        if l.iter().all(|c| *c == '.') {
            let big_line = vec!['.'; l.len() * 2];
            expanded_horizontally.push(big_line.clone());
            expanded_horizontally.push(big_line);
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
            let big_col = vec!['.'; l.len() * 2];
            expanded_transposed.push(big_col.clone());
            expanded_transposed.push(big_col);
        } else {
            expanded_transposed.push(l.clone());
        }
    }
    let expanded = transpose(expanded_transposed);
    // print_image(&expanded);
    expanded
}

fn sum_of_shortest_paths(image: &Vec<Vec<char>>) -> usize {
    374
}

fn main() {
    let stdin = io::stdin();
    let image: Vec<Vec<char>> = build_image(&mut stdin.lock());

    expand_universe(&image);

    println!("Part 1: {}", sum_of_shortest_paths(&image));
}

#[test]
fn test_expand_universe() {
    let mut reader = BufReader::new(File::open("resources/input_test1").unwrap());
    let image: Vec<Vec<char>> = build_image(&mut reader);

    let mut reader_expanded = BufReader::new(File::open("resources/test1_expanded").unwrap());
    let image_expanded: Vec<Vec<char>> = build_image(&mut reader_expanded);
    
    assert_eq!(expand_universe(&image), image_expanded);
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let image: Vec<Vec<char>> = build_image(&mut reader);
    sum_of_shortest_paths(&image)
}

#[test]
fn test_part1() {
    assert_eq!(part1("resources/input_test1"), 374);
}
