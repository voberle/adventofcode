use std::io::{self, Read};

use regex::Regex;

#[derive(Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePosition(char),
    ReversePosition(usize, usize),
    MovePosition(usize, usize),
}

fn position(s: &[char], x: char) -> usize {
    s.iter().position(|&c| c == x).unwrap()
}

// Calculates the offset to rotate for the RotatePosition instruction
fn rotate_pos_offset(pos: usize, len: usize) -> usize {
    let mut x = pos;
    if x >= 4 {
        x += 1;
    }
    x += 1;
    x %= len;
    x
}

// Calculates the offset needed to reverse the RotatePosition instruction.
fn reverse_rotate_pos_offset(pos: usize, len: usize) -> usize {
    // Reversed table to find how much to rotate back
    // This only works for real input. On tests, we have a double mapping for index 0.
    (0..len)
        .map(|i| {
            let r = rotate_pos_offset(i, len);
            ((i + r) % len, (len + r) % len)
        })
        .find(|(s, _)| *s == pos)
        .unwrap()
        .1
}

impl Instruction {
    fn scramble(&self, s: &mut Vec<char>) {
        match self {
            Instruction::SwapPosition(x, y) => {
                s.swap(*x, *y);
            }
            Instruction::SwapLetter(lx, ly) => {
                let x = position(s, *lx);
                let y = position(s, *ly);
                s.swap(x, y);
            }
            Instruction::RotateLeft(x) => {
                s.rotate_left(*x);
            }
            Instruction::RotateRight(x) => {
                s.rotate_right(*x);
            }
            Instruction::RotatePosition(lx) => {
                let x = rotate_pos_offset(position(s, *lx), s.len());
                s.rotate_right(x);
            }
            Instruction::ReversePosition(x, y) => {
                let mut repl = s[*x..=*y].to_owned();
                repl.reverse();
                s.splice(x..=y, repl.clone());
            }
            Instruction::MovePosition(x, y) => {
                let letter = s.remove(*x);
                s.insert(*y, letter);
            }
        }
    }

    fn unscramble(&self, s: &mut Vec<char>) {
        match self {
            Instruction::SwapPosition(_, _)
            | Instruction::SwapLetter(_, _)
            | Instruction::ReversePosition(_, _) => self.scramble(s),
            Instruction::RotateLeft(x) => {
                s.rotate_right(*x); // right instead of left
            }
            Instruction::RotateRight(x) => {
                s.rotate_left(*x); // left instead of right
            }
            Instruction::RotatePosition(lx) => {
                let x = reverse_rotate_pos_offset(position(s, *lx), s.len());
                s.rotate_left(x);
            }
            Instruction::MovePosition(x, y) => {
                let letter = s.remove(*y); // swap the indexes
                s.insert(*x, letter);
            }
        }
    }
}

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[inline]
fn usize(s: &str) -> usize {
    s.parse().unwrap()
}

fn build(input: &str) -> Vec<Instruction> {
    let re_swap_position = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let re_swap_letter = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
    let re_rotate_left = Regex::new(r"rotate left (\d+) step").unwrap();
    let re_rotate_right = Regex::new(r"rotate right (\d+) step").unwrap();
    let re_rotate_position = Regex::new(r"rotate based on position of letter (\w)").unwrap();
    let re_reverse_position = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let re_move_position = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            if let Some(p) = re_swap_position.captures(line) {
                Instruction::SwapPosition(usize(&p[1]), usize(&p[2]))
            } else if let Some(p) = re_swap_letter.captures(line) {
                Instruction::SwapLetter(char(&p[1]), char(&p[2]))
            } else if let Some(p) = re_rotate_left.captures(line) {
                Instruction::RotateLeft(usize(&p[1]))
            } else if let Some(p) = re_rotate_right.captures(line) {
                Instruction::RotateRight(usize(&p[1]))
            } else if let Some(p) = re_rotate_position.captures(line) {
                Instruction::RotatePosition(char(&p[1]))
            } else if let Some(p) = re_reverse_position.captures(line) {
                Instruction::ReversePosition(usize(&p[1]), usize(&p[2]))
            } else if let Some(p) = re_move_position.captures(line) {
                Instruction::MovePosition(usize(&p[1]), usize(&p[2]))
            } else {
                panic!("Invalid instruction {}", line)
            }
        })
        .collect()
}

fn scramble(instructions: &[Instruction], input: &str) -> String {
    let mut s: Vec<char> = input.chars().collect();
    for ins in instructions {
        ins.scramble(&mut s);
    }
    s.into_iter().collect()
}

fn unscramble(instructions: &[Instruction], input: &str) -> String {
    let mut s: Vec<char> = input.chars().collect();
    for ins in instructions.iter().rev() {
        ins.unscramble(&mut s);
    }
    s.into_iter().collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", scramble(&instructions, "abcdefgh"));
    println!("Part 2: {}", unscramble(&instructions, "fbgdceah"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_position() {
        let mut s = "dhbafegc".chars().collect();
        Instruction::RotatePosition('g').scramble(&mut s);
        assert_eq!(s, "dhbafegc".chars().collect::<Vec<_>>());
    }

    #[rustfmt::skip]
    #[test]
    fn test_scramble() {
        assert_eq!(scramble(&build("swap position 4 with position 0"), "abcde"), "ebcda");
        assert_eq!(scramble(&build("swap letter d with letter b"), "ebcda"), "edcba");
        assert_eq!(scramble(&build("reverse positions 0 through 4"), "edcba"), "abcde");
        assert_eq!(scramble(&build("rotate left 1 step"), "abcde"), "bcdea");
        assert_eq!(scramble(&build("move position 1 to position 4"), "bcdea"), "bdeac");
        assert_eq!(scramble(&build("move position 3 to position 0"), "bdeac"), "abdec");
        assert_eq!(scramble(&build("rotate based on position of letter b"), "abdec"), "ecabd");
        assert_eq!(scramble(&build("rotate based on position of letter d"), "ecabd"), "decab");
    }

    #[rustfmt::skip]
    #[test]
    fn test_unscramble() {
        // The method we use to reverse the rotation command doesn't work on the test input for that command.car
        // assert_eq!(unscramble(&build("rotate based on position of letter d"), "decab"), "ecabd");
        assert_eq!(unscramble(&build("rotate based on position of letter b"), "ecabd"), "abdec");
        assert_eq!(unscramble(&build("move position 3 to position 0"), "abdec"), "bdeac");
        assert_eq!(unscramble(&build("move position 1 to position 4"), "bdeac"), "bcdea");
        assert_eq!(unscramble(&build("rotate left 1 step"), "bcdea"), "abcde");
        assert_eq!(unscramble(&build("reverse positions 0 through 4"), "abcde"), "edcba");
        assert_eq!(unscramble(&build("swap letter d with letter b"), "edcba"), "ebcda");
        assert_eq!(unscramble(&build("swap position 4 with position 0"), "ebcda"), "abcde");
    }

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(scramble(&build(INPUT_TEST), "abcde"), "decab");
    }
}
