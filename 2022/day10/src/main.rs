use std::{
    fmt,
    io::{self, Read},
};

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
    const CHECKPOINTS: [i32; 6] = [20, 60, 100, 140, 180, 220];

    let mut sum = 0;
    let mut cycle = 0;

    let mut x = 1;

    for ins in instructions {
        match ins {
            Instruction::Noop => {
                cycle += 1;
                if CHECKPOINTS.contains(&cycle) {
                    sum += cycle * x;
                }
            }
            Instruction::AddX(v) => {
                cycle += 1;
                if CHECKPOINTS.contains(&cycle) {
                    sum += cycle * x;
                }
                cycle += 1;
                if CHECKPOINTS.contains(&cycle) {
                    sum += cycle * x;
                }

                x += v;
            }
        }
    }
    sum
}

struct Crt {
    screen: [bool; 40 * 6],
    pos: usize,
}

impl Crt {
    fn new() -> Self {
        Self {
            screen: [false; 40 * 6],
            pos: 0,
        }
    }

    // Draws the pixel if the sprite is currently on the CRT pixel position.
    fn draw_pixel(&mut self, sprite_center: i32) {
        let horiz_pos = i32::try_from(self.pos % 40).unwrap();

        if (sprite_center - 1..=sprite_center + 1).contains(&horiz_pos) {
            self.screen[self.pos] = true;
        }

        self.pos += 1;
    }
}

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const ROWS: usize = 6;
        const COLS: usize = 40;
        let mut s = String::with_capacity(ROWS * COLS);
        for row in 0..ROWS {
            for e in self.screen.iter().take((row + 1) * COLS).skip(row * COLS) {
                // These unicode chars are easier to read.
                // let c = if *e { '\u{2B1B}' } else { '\u{2B1C}' };
                // but we use #. as our OCR crate is using them.
                let c = if *e { '#' } else { '.' };
                s.push(c);
            }
            if row < ROWS - 1 {
                s.push('\n');
            }
        }
        write!(f, "{s}")
    }
}

fn crt_picture(instructions: &[Instruction]) -> String {
    let mut crt = Crt::new();

    let mut sprite_center: i32 = 1;

    for ins in instructions {
        match ins {
            Instruction::Noop => {
                crt.draw_pixel(sprite_center);
            }
            Instruction::AddX(v) => {
                crt.draw_pixel(sprite_center);
                crt.draw_pixel(sprite_center);

                sprite_center += v;
            }
        }
    }

    let s = crt.to_string();
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
