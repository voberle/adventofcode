use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

fn build(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Up,
                    'D' => Down,
                    'L' => Left,
                    'R' => Right,
                    _ => panic!("Invalid direction char"),
                })
                .collect()
        })
        .collect()
}

fn move_to(pos: u8, dir: Direction) -> u8 {
    match dir {
        Up => match pos {
            4..=9 => pos - 3,
            _ => pos,
        },
        Down => match pos {
            1..=6 => pos + 3,
            _ => pos,
        },
        Left => match pos {
            2 | 3 | 5 | 6 | 8 | 9 => pos - 1,
            _ => pos,
        },
        Right => match pos {
            1 | 2 | 4 | 5 | 7 | 8 => pos + 1,
            _ => pos,
        },
    }
}

fn find_code(instructions: &[Vec<Direction>]) -> String {
    let mut code = Vec::new();
    let mut pos: u8 = 5;
    for line in instructions {
        pos = line.iter().fold(pos, |current, dir| move_to(current, *dir));
        code.push(pos);
    }
    code.iter().map(u8::to_string).collect()
}

fn part2(instructions: &[Vec<Direction>]) -> String {
    "".to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let instructions = build(&input);

    println!("Part 1: {}", find_code(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(find_code(&build(INPUT_TEST)), "1985");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), "");
    }
}
