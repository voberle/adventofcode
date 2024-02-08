use std::io::{self, Read};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Sky {
    // We store the positions and velocities as 4 separate vectors, as it makes
    // it easy to update them and detect lines.
    x_pos: Vec<i32>,
    y_pos: Vec<i32>,
    x_vel: Vec<i32>,
    y_vel: Vec<i32>,
}

impl Sky {
    fn build(input: &str) -> Self {
        let re =
            Regex::new(r"position=< *(-?\d+),  *(-?\d+)> velocity=< *(-?\d+),  *(-?\d+)>").unwrap();
        let mut sky = Sky {
            x_pos: Vec::new(),
            y_pos: Vec::new(),
            x_vel: Vec::new(),
            y_vel: Vec::new(),
        };
        for line in input.lines() {
            let p = re.captures(line).unwrap();
            sky.x_pos.push(p[1].parse().unwrap());
            sky.y_pos.push(p[2].parse().unwrap());
            sky.x_vel.push(p[3].parse().unwrap());
            sky.y_vel.push(p[4].parse().unwrap());
        }
        sky
    }

    fn next(&mut self) {
        self.x_pos
            .iter_mut()
            .zip(self.x_vel.iter())
            .for_each(|(p, v)| *p += v);
        self.y_pos
            .iter_mut()
            .zip(self.y_vel.iter())
            .for_each(|(p, v)| *p += v);
    }

    fn has_possible_message<const BAND_HEIGHT_TO_CHECK: i32>(&self) -> bool {
        // To have a message, all points need to be within a narrow horizontal band.
        if let itertools::MinMaxResult::MinMax(min_y, max_y) = self.y_pos.iter().minmax() {
            if max_y - min_y > BAND_HEIGHT_TO_CHECK {
                return false;
            }
        }
        true

        // To detect if we have a message, we can check if we have lines, meaning several dots aligned.
        // For example, let's look for vertical lines, as they are clearer in the alphabet.
        // But with the input we have, it's not necessary, previous check is enough.
        // let mut x_freq: FxHashMap<i32, usize> = FxHashMap::default();
        // for x in &self.x_pos {
        //     *x_freq.entry(*x).or_default() += 1;
        // }
        // let mut vert_lines_found = 0;
        // for f in x_freq.values() {
        //     if *f > 7 {
        //         vert_lines_found += 1;
        //     }
        // }
        // vert_lines_found > 6
    }

    #[allow(clippy::cast_sign_loss)]
    fn print(&self) {
        if let itertools::MinMaxResult::MinMax(min_x, max_x) = self.x_pos.iter().minmax() {
            if let itertools::MinMaxResult::MinMax(min_y, max_y) = self.y_pos.iter().minmax() {
                // Create a simple 2-d array with the dots to print.
                // Can be seen as wasteful to have this temporary array, but it should be small and makes things simpler.
                let mut v =
                    vec![vec![false; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
                self.y_pos.iter().zip(self.x_pos.iter()).for_each(|(y, x)| {
                    let y = (y - min_y) as usize;
                    let x = (x - min_x) as usize;
                    v[y][x] = true;
                });

                for y in 0..v.len() {
                    for x in 0..v[0].len() {
                        print!("{}", if v[y][x] { '\u{2593}' } else { '\u{2591}' });
                    }
                    println!();
                }
            }
        }
    }
}

fn find_message<const BAND_HEIGHT_TO_CHECK: i32>(sky: &Sky) -> usize {
    let mut sky = sky.clone();
    for seconds in 0..usize::MAX {
        if sky.has_possible_message::<BAND_HEIGHT_TO_CHECK>() {
            // println!("Found message after {} seconds", seconds);
            sky.print();
            return seconds;
        }
        sky.next();
    }
    panic!("Didn't find the message");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sky = Sky::build(input.trim());

    let seconds = find_message::<12>(&sky);
    println!("Part 1: Read above");
    println!("Part 2: {}", seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part2() {
        assert_eq!(find_message::<8>(&Sky::build(INPUT_TEST)), 3);
    }
}
