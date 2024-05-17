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

// Gets the corners of the image
fn borders(pixels: &FxHashSet<Pos>) -> (Pos, Pos) {
    let mut min_pos = Pos::new(i32::MAX, i32::MAX);
    let mut max_pos = Pos::new(i32::MIN, i32::MIN);
    for pos in pixels {
        min_pos.x = min_pos.x.min(pos.x);
        max_pos.x = max_pos.x.max(pos.x);
        min_pos.y = min_pos.y.min(pos.y);
        max_pos.y = max_pos.y.max(pos.y);
    }
    (min_pos, max_pos)
}

#[derive(Clone)]
struct Image {
    pixels: FxHashSet<Pos>,
    // Borders of the picture.
    borders_min: Pos,
    borders_max: Pos,
    // Indicates if a pixel outside the borders is lit or not.
    default_state: bool,
}

impl Image {
    fn empty(default_state: bool) -> Self {
        Self {
            pixels: FxHashSet::default(),
            borders_min: Pos::new(0, 0),
            borders_max: Pos::new(0, 0),
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
        let (borders_min, borders_max) = borders(&pixels);
        Self {
            pixels,
            borders_min,
            borders_max,
            default_state: false,
        }
    }

    fn update_borders(&mut self) {
        let (borders_min, borders_max) = borders(&self.pixels);
        self.borders_min = borders_min;
        self.borders_max = borders_max;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.borders_min.y..=self.borders_max.y {
            for x in self.borders_min.x..=self.borders_max.x {
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

    fn is_within_borders(&self, pos: Pos) -> bool {
        (self.borders_min.x..=self.borders_max.x).contains(&pos.x)
            && (self.borders_min.y..=self.borders_max.y).contains(&pos.y)
    }

    fn is_pixel_lit(&self, x: i32, y: i32) -> bool {
        let pos = Pos::new(x, y);
        if self.is_within_borders(pos) {
            self.pixels.contains(&pos)
        } else {
            self.default_state
        }
    }

    fn get_code(&self, p: Pos) -> usize {
        (usize::from(self.is_pixel_lit(p.x - 1, p.y - 1)) << 8)
            + (usize::from(self.is_pixel_lit(p.x, p.y - 1)) << 7)
            + (usize::from(self.is_pixel_lit(p.x + 1, p.y - 1)) << 6)
            + (usize::from(self.is_pixel_lit(p.x - 1, p.y)) << 5)
            + (usize::from(self.is_pixel_lit(p.x, p.y)) << 4)
            + (usize::from(self.is_pixel_lit(p.x + 1, p.y)) << 3)
            + (usize::from(self.is_pixel_lit(p.x - 1, p.y + 1)) << 2)
            + (usize::from(self.is_pixel_lit(p.x, p.y + 1)) << 1)
            + (usize::from(self.is_pixel_lit(p.x + 1, p.y + 1)))
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
    for y in image.borders_min.y - 1..=image.borders_max.y + 1 {
        for x in image.borders_min.x - 1..=image.borders_max.x + 1 {
            let pos = Pos::new(x, y);
            let code = image.get_code(pos);
            if algo[code] {
                new_image.pixels.insert(pos);
            }
        }
    }
    // Once the new image is fully set, make sure it has borders properly calculated.
    new_image.update_borders();
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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (algo, image) = build(&input);

    println!("Part 1: {}", lit_pixels_after::<true>(&algo, &image, 2));
    println!("Part 2: {}", lit_pixels_after::<true>(&algo, &image, 50));
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
        assert_eq!(lit_pixels_after::<false>(&algo, &image, 50), 3351);
    }
}
