use std::io::{self, Read};

enum Instruction {
    Noop,
    AddX(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value == "noop" {
            Self::Noop
        } else {
            Self::AddX(
                value
                    .strip_prefix("addx ")
                    .expect("Not a addx")
                    .parse()
                    .expect("Not a value"),
            )
        }
    }
}

fn build(input: &str) -> Vec<Instruction> {
    input.lines().map(Into::into).collect()
}

fn signal_strengths_sum(instructions: &[Instruction]) -> i32 {
    let checkpoints = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    let mut x = 1;
    let mut cycle = 0;

    for ins in instructions {
        match ins {
            Instruction::Noop => {
                cycle += 1;
                if checkpoints.contains(&cycle) {
                    // println!("{} * {} = {}", cycle, x, cycle * x);
                    sum += cycle * x;
                }
            }
            Instruction::AddX(v) => {
                cycle += 1;
                if checkpoints.contains(&cycle) {
                    // println!("{} * {} = {}", cycle, x, cycle * x);
                    sum += cycle * x;
                }
                cycle += 1;
                if checkpoints.contains(&cycle) {
                    // println!("{} * {} = {}", cycle, x, cycle * x);
                    sum += cycle * x;
                }

                x += v;
            }
        }
    }
    sum
}

fn crt_to_string(crt: &[bool]) -> String {
    const ROWS: usize = 6;
    const COLS: usize = 40;
    let mut s = String::with_capacity(ROWS * COLS);
    for row in 0..ROWS {
        for p in row * COLS..(row + 1) * COLS {
            // s.push(if crt[p] { '\u{2B1B}' } else { '\u{2B1C}' });
            s.push(if crt[p] { '#' } else { '.' });
        }
        if row < ROWS - 1 {
            s.push('\n');
        }
    }
    s
}

fn crt_picture(instructions: &[Instruction]) -> String {
    let mut crt = [false; 40 * 6];

    let mut sprite_center: i32 = 1;
    let mut crt_pixel_pos: usize = 0;

    for ins in instructions {
        match ins {
            Instruction::Noop => {
                let horiz_pos: i32 = (crt_pixel_pos % 40) as i32;
                if (sprite_center - 1..=sprite_center + 1).contains(&horiz_pos) {
                    crt[crt_pixel_pos] = true;
                }
                crt_pixel_pos += 1;
            }
            Instruction::AddX(v) => {
                let horiz_pos: i32 = (crt_pixel_pos % 40) as i32;
                if (sprite_center - 1..=sprite_center + 1).contains(&horiz_pos) {
                    crt[crt_pixel_pos] = true;
                }
                crt_pixel_pos += 1;

                let horiz_pos: i32 = (crt_pixel_pos % 40) as i32;
                if (sprite_center - 1..=sprite_center + 1).contains(&horiz_pos) {
                    crt[crt_pixel_pos] = true;
                }
                crt_pixel_pos += 1;

                sprite_center += v;
            }
        }
    }

    let s = crt_to_string(&crt);
    // println!("{}", s);

    advent_of_code_ocr::parse_string_to_letters(&s)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", signal_strengths_sum(&instructions));
    println!("Part 2: {}", crt_picture(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(signal_strengths_sum(&build(INPUT_TEST)), 13140);
    }
}
