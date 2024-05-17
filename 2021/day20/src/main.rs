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

#[allow(dead_code)]
fn algo_to_string(algo: &[bool]) -> String {
    algo.iter().map(|v| if *v { '#' } else { '.' }).collect()
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

fn get_code(image: &Image, pos: Pos) -> usize {
    (usize::from(image.contains(&Pos::new(pos.x - 1, pos.y - 1))) << 8)
        + (usize::from(image.contains(&Pos::new(pos.x, pos.y - 1))) << 7)
        + (usize::from(image.contains(&Pos::new(pos.x + 1, pos.y - 1))) << 6)
        + (usize::from(image.contains(&Pos::new(pos.x - 1, pos.y))) << 5)
        + (usize::from(image.contains(&Pos::new(pos.x, pos.y))) << 4)
        + (usize::from(image.contains(&Pos::new(pos.x + 1, pos.y))) << 3)
        + (usize::from(image.contains(&Pos::new(pos.x - 1, pos.y + 1))) << 2)
        + (usize::from(image.contains(&Pos::new(pos.x, pos.y + 1))) << 1)
        + (usize::from(image.contains(&Pos::new(pos.x + 1, pos.y + 1))))
}

fn convert_image(algo: &[bool], image: &Image) -> Image {
    const EXTRA: i32 = 15;
    let mut new_image = FxHashSet::default();
    // The pixels to consider are all within the borders + some extra lines.
    let (min_pos, max_pos) = borders(image);
    for y in min_pos.y - EXTRA..=max_pos.y + EXTRA {
        for x in min_pos.x - EXTRA..=max_pos.x + EXTRA {
            let pos = Pos::new(x, y);
            let code = get_code(image, pos);
            if algo[code] {
                new_image.insert(pos);
            }
        }
    }
    new_image
}

fn lit_pixels_after(algo: &[bool], image: &Image, tranformations: usize) -> usize {
    // In the real image, the code 0 is lit, meaning one transformations lits an infinite number of pixel,
    // and next one shuts them off again.
    // It means we can only count even number of transformations.
    assert_eq!(tranformations % 2, 0);

    println!("Initial");
    print(image);

    let image1 = convert_image(algo, image);
    println!("1 transformation");
    print(&image1);

    let image2 = convert_image(algo, &image1);
    println!("2 transformation");
    print(&image2);

    image2.len()
}

fn part2(algo: &[bool], image: &Image) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (algo, image) = build(&input);

    // print(&image);

    println!("Part 1: {}", lit_pixels_after(&algo, &image, 2));
    println!("Part 2: {}", part2(&algo, &image));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (algo, image) = build(INPUT_TEST);
        assert_eq!(lit_pixels_after(&algo, &image, 2), 35);
    }

    #[test]
    fn test_part2() {
        let (algo, image) = build(INPUT_TEST);
        assert_eq!(part2(&algo, &image), 0);
    }
}
