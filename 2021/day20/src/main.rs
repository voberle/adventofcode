use std::io::{self, Read};

use fxhash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32, // from west to east
    y: i32, // from north to south
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

type Image = FxHashSet<Pos>;

// Gets the corners of the image
fn borders(image: &Image) -> (Pos, Pos) {
    let mut min_pos = Pos::new(i32::MAX, i32::MAX);
    let mut max_pos = Pos::new(i32::MIN, i32::MIN);
    for pos in image {
        min_pos.x = min_pos.x.min(pos.x);
        max_pos.x = max_pos.x.max(pos.x);
        min_pos.y = min_pos.y.min(pos.y);
        max_pos.y = max_pos.y.max(pos.y);
    }
    (min_pos, max_pos)
}

#[allow(dead_code)]
fn print(image: &Image) {
    let (min_pos, max_pos) = borders(image);
    for y in min_pos.y..=max_pos.y {
        for x in min_pos.x..=max_pos.x {
            let pos = Pos::new(x, y);
            if image.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn build(input: &str) -> (Vec<bool>, Image) {
    let algo: Vec<bool> = input
        .lines()
        .next()
        .unwrap()
        .bytes()
        .map(|b| b == b'#')
        .collect();

    let mut y = 0;
    let image: Image = input
        .lines()
        .skip(2)
        .flat_map(|l| {
            y += 1;
            l.bytes().enumerate().filter_map(move |(x, b)| {
                if b == b'#' {
                    Some(Pos::new(i32::try_from(x).unwrap(), y))
                } else {
                    None
                }
            })
        })
        .collect();

    (algo, image)
}

fn lit_pixels_after_two(algo: &[bool], image: &Image) -> usize {
    0
}

fn part2(algo: &[bool], image: &Image) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (algo, image) = build(&input);

    print(&image);

    println!("Part 1: {}", lit_pixels_after_two(&algo, &image));
    println!("Part 2: {}", part2(&algo, &image));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (algo, image) = build(INPUT_TEST);
        assert_eq!(lit_pixels_after_two(&algo, &image), 35);
    }

    #[test]
    fn test_part2() {
        let (algo, image) = build(INPUT_TEST);
        assert_eq!(part2(&algo, &image), 0);
    }
}
