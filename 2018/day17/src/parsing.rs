//! Code here is only helping with parsing the input.
//! Might be a bit overkill, but I started with that, so kept it.

use regex::Regex;

// A line. Coordinates are ordered, [x|y]1 <= [x|y]2
#[derive(Debug)]
pub struct Line {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl Line {
    fn contains(&self, x: usize, y: usize) -> bool {
        (self.x1..=self.x2).contains(&x) && (self.y1..=self.y2).contains(&y)
    }
}

pub fn build(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(x|y)=(\d+).?.?(\d+)?").unwrap();
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(", ").collect();
            let p1 = re.captures(parts[0]).unwrap();
            let p2 = re.captures(parts[1]).unwrap();

            let x = if &p1[1] == "x" { &p1 } else { &p2 };
            let y = if &p1[1] == "y" { &p1 } else { &p2 };
            let mut x1 = x[2].parse::<usize>().unwrap();
            let mut x2 = x
                .get(3)
                .map_or(x1, |m| m.as_str().parse::<usize>().unwrap());
            let mut y1 = y[2].parse::<usize>().unwrap();
            let mut y2 = y
                .get(3)
                .map_or(y1, |m| m.as_str().parse::<usize>().unwrap());
            // Sorting the values, easier to manipulate.
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

            Line { x1, y1, x2, y2 }
        })
        .collect()
}

// Returns min x, max x, min y, max y.
pub fn borders(lines: &[Line]) -> (usize, usize, usize, usize) {
    // Not using iterator min / max to keep only one loop.
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;
    for l in lines {
        min_x = min_x.min(l.x1.min(l.x2));
        max_x = max_x.max(l.x1.max(l.x2));
        min_y = min_y.min(l.y1.min(l.y2));
        max_y = max_y.max(l.y1.max(l.y2));
    }
    (min_x, max_x, min_y, max_y)
}

pub fn contains(lines: &[Line], x: usize, y: usize) -> bool {
    lines.iter().any(|l| l.contains(x, y))
}

#[allow(dead_code)]
fn print(lines: &[Line]) {
    let (min_x, max_x, _, max_y) = borders(lines);
    for y in 0..=max_y {
        for x in min_x..=max_x {
            let c = if contains(lines, x, y) { '#' } else { '.' };
            print!("{c}");
        }
        println!();
    }
}
