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

impl Instruction {
    fn exec(&self, s: &mut Vec<char>) {
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
                let mut x = position(s, *lx);
                if x >= 4 {
                    x += 1;
                }
                x += 1;
                x %= s.len();
                s.rotate_right(x)
            }
            Instruction::ReversePosition(x, y) => {
                let mut repl = s[*x..=*y].to_owned();
                repl.reverse();
                s.splice(x..=y, repl.to_vec());
            }
            Instruction::MovePosition(x, y) => {
                let letter = s.remove(*x);
                s.insert(*y, letter);
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
        ins.exec(&mut s);
        // println!("{:?}: {}", ins, s.clone().into_iter().collect::<String>());
    }
    s.into_iter().collect()
}

fn part2(instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", scramble(&instructions, "abcdefgh"));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_rotate_position() {
        let mut s = "dhbafegc".chars().collect();
        Instruction::RotatePosition('g').exec(&mut s);
        assert_eq!(s, "dhbafegc".chars().collect::<Vec<_>>());
    }

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(scramble(&build(INPUT_TEST), "abcde"), "decab");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
