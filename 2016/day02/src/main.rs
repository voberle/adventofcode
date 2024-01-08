use std::{
    fmt::Display,
    io::{self, Read},
};

const UP: char = 'U';
const DOWN: char = 'D';
const LEFT: char = 'L';
const RIGHT: char = 'R';

fn move_normal_keyboard(pos: u8, dir: char) -> u8 {
    match dir {
        UP => match pos {
            4..=9 => pos - 3,
            _ => pos,
        },
        DOWN => match pos {
            1..=6 => pos + 3,
            _ => pos,
        },
        LEFT => match pos {
            2 | 3 | 5 | 6 | 8 | 9 => pos - 1,
            _ => pos,
        },
        RIGHT => match pos {
            1 | 2 | 4 | 5 | 7 | 8 => pos + 1,
            _ => pos,
        },
        _ => panic!("Invalid direction char"),
    }
}

fn move_weird_keyboard(pos: char, dir: char) -> char {
    match pos {
        '1' => match dir {
            DOWN => '3',
            _ => pos,
        },
        '2' => match dir {
            RIGHT => '3',
            DOWN => '6',
            _ => pos,
        },
        '3' => match dir {
            LEFT => '2',
            RIGHT => '4',
            UP => '1',
            DOWN => '7',
            _ => pos,
        },
        '4' => match dir {
            LEFT => '3',
            DOWN => '8',
            _ => pos,
        },
        '5' => match dir {
            RIGHT => '6',
            _ => pos,
        },
        '6' => match dir {
            LEFT => '5',
            RIGHT => '7',
            UP => '2',
            DOWN => 'A',
            _ => pos,
        },
        '7' => match dir {
            LEFT => '6',
            RIGHT => '8',
            UP => '3',
            DOWN => 'B',
            _ => pos,
        },
        '8' => match dir {
            LEFT => '7',
            RIGHT => '9',
            UP => '4',
            DOWN => 'C',
            _ => pos,
        },
        '9' => match dir {
            LEFT => '8',
            _ => pos,
        },
        'A' => match dir {
            RIGHT => 'B',
            UP => '6',
            _ => pos,
        },
        'B' => match dir {
            LEFT => 'A',
            RIGHT => 'C',
            UP => '7',
            DOWN => 'D',
            _ => pos,
        },
        'C' => match dir {
            LEFT => 'B',
            UP => '8',
            _ => pos,
        },
        'D' => match dir {
            UP => 'B',
            _ => pos,
        },
        _ => panic!("Invalid key"),
    }
}

fn find_code<T>(instructions: &str, move_to: fn(T, char) -> T, start: T) -> String
where
    T: Display + Copy,
{
    let mut code = Vec::new();
    let mut pos: T = start;
    for line in instructions.lines() {
        pos = line.chars().fold(pos, move_to);
        code.push(pos);
    }
    code.iter().map(T::to_string).collect()
}

fn find_first_code(instructions: &str) -> String {
    find_code(instructions, move_normal_keyboard, 5)
}

fn find_second_code(instructions: &str) -> String {
    find_code(instructions, move_weird_keyboard, '5')
}

fn main() {
    let mut instructions = String::new();
    io::stdin().read_to_string(&mut instructions).unwrap();

    println!("Part 1: {}", find_first_code(&instructions));
    println!("Part 2: {}", find_second_code(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(find_first_code(INPUT_TEST), "1985");
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_second_code(INPUT_TEST), "5DB3");
    }
}
