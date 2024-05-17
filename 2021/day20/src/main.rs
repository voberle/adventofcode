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

#[derive(Clone)]
struct Image {
    pixels: FxHashSet<Pos>,
    // Indicates if a pixel outside the borders is lit or not.
    default_state: bool,
}

impl Image {
    fn empty(default_state: bool) -> Self {
        Self {
            pixels: FxHashSet::default(),
            default_state,
        }
    }

    fn build(input: &str) -> Self {
        let mut y = 0;
        let pixels = input
            .lines()
            .skip(2) // first two lines are the algo, ignore them.
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
        Self {
            pixels,
            default_state: false,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (min_pos, max_pos) = self.borders();
        for y in min_pos.y..=max_pos.y {
            for x in min_pos.x..=max_pos.x {
                let pos = Pos::new(x, y);
                if self.pixels.contains(&pos) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // Gets the corners of the image
    fn borders(&self) -> (Pos, Pos) {
        let mut min_pos = Pos::new(i32::MAX, i32::MAX);
        let mut max_pos = Pos::new(i32::MIN, i32::MIN);
        for pos in &self.pixels {
            min_pos.x = min_pos.x.min(pos.x);
            max_pos.x = max_pos.x.max(pos.x);
            min_pos.y = min_pos.y.min(pos.y);
            max_pos.y = max_pos.y.max(pos.y);
        }
        (min_pos, max_pos)
    }

    fn is_within_borders(pos: Pos, min_pos: Pos, max_pos: Pos) -> bool {
        (min_pos.x..=max_pos.x).contains(&pos.x) && (min_pos.y..=max_pos.y).contains(&pos.y)
    }

    fn is_pixel_lit(&self, x: i32, y: i32, min_pos: Pos, max_pos: Pos) -> bool {
        let pos = Pos::new(x, y);
        if Self::is_within_borders(pos, min_pos, max_pos) {
            self.pixels.contains(&pos)
        } else {
            self.default_state
        }
    }

    fn get_code(&self, p: Pos) -> usize {
        let (min_p, max_p) = self.borders();

        (usize::from(self.is_pixel_lit(p.x - 1, p.y - 1, min_p, max_p)) << 8)
            + (usize::from(self.is_pixel_lit(p.x, p.y - 1, min_p, max_p)) << 7)
            + (usize::from(self.is_pixel_lit(p.x + 1, p.y - 1, min_p, max_p)) << 6)
            + (usize::from(self.is_pixel_lit(p.x - 1, p.y, min_p, max_p)) << 5)
            + (usize::from(self.is_pixel_lit(p.x, p.y, min_p, max_p)) << 4)
            + (usize::from(self.is_pixel_lit(p.x + 1, p.y, min_p, max_p)) << 3)
            + (usize::from(self.is_pixel_lit(p.x - 1, p.y + 1, min_p, max_p)) << 2)
            + (usize::from(self.is_pixel_lit(p.x, p.y + 1, min_p, max_p)) << 1)
            + (usize::from(self.is_pixel_lit(p.x + 1, p.y + 1, min_p, max_p)))
    }

    fn lit_pixels_count(&self) -> usize {
        assert!(!self.default_state);
        self.pixels.len()
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
    let image = Image::build(input);
    (algo, image)
}

fn convert_image<const INVERT_DEFAULT: bool>(algo: &[bool], image: &Image) -> Image {
    let mut new_image = Image::empty(if INVERT_DEFAULT {
        // The new image will have external pixels inverted.
        !image.default_state
    } else {
        false
    });

    // The pixels to consider are all within the borders + one extra line.
    let (min_pos, max_pos) = image.borders();

    for y in min_pos.y - 1..=max_pos.y + 1 {
        for x in min_pos.x - 1..=max_pos.x + 1 {
            let pos = Pos::new(x, y);
            let code = image.get_code(pos);
            if algo[code] {
                new_image.pixels.insert(pos);
            }
        }
    }
    new_image
}

fn lit_pixels_after<const INVERT_DEFAULT: bool>(
    algo: &[bool],
    image: &Image,
    tranformations: usize,
) -> usize {
    // In the real image, the code 0 is lit, meaning one transformations lits an infinite number of pixel,
    // and next one shuts them off again.
    // It means we can only count even number of transformations.
    assert_eq!(tranformations % 2, 0);

    let mut image = image.clone();
    for _s in 0..tranformations {
        image = convert_image::<INVERT_DEFAULT>(algo, &image);

        // println!("Transformation {}", _s + 1);
        // image.print();
    }
    image.lit_pixels_count()
}

fn part2(algo: &[bool], image: &Image) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (algo, image) = build(&input);

    println!("Part 1: {}", lit_pixels_after::<true>(&algo, &image, 2));
    println!("Part 2: {}", part2(&algo, &image));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (algo, image) = build(INPUT_TEST);
        assert_eq!(lit_pixels_after::<false>(&algo, &image, 2), 35);
    }

    #[test]
    fn test_part2() {
        let (algo, image) = build(INPUT_TEST);
        assert_eq!(part2(&algo, &image), 0);
    }
}
