use std::io::{self, Read};

fn move_to(pos: u8, dir: char) -> u8 {
    match dir {
        'U' => match pos {
            4..=9 => pos - 3,
            _ => pos,
        },
        'D' => match pos {
            1..=6 => pos + 3,
            _ => pos,
        },
        'L' => match pos {
            2 | 3 | 5 | 6 | 8 | 9 => pos - 1,
            _ => pos,
        },
        'R' => match pos {
            1 | 2 | 4 | 5 | 7 | 8 => pos + 1,
            _ => pos,
        },
        _ => panic!("Invalid direction char"),
    }
}

fn find_code(instructions: &str) -> String {
    let mut code = Vec::new();
    let mut pos: u8 = 5;
    for line in instructions.lines() {
        pos = line.chars().fold(pos, move_to);
        code.push(pos);
    }
    code.iter().map(u8::to_string).collect()
}

fn part2(instructions: &str) -> String {
    "".to_string()
}

fn main() {
    let mut instructions = String::new();
    io::stdin().read_to_string(&mut instructions).unwrap();

    println!("Part 1: {}", find_code(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(find_code(INPUT_TEST), "1985");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEST), "");
    }
}
