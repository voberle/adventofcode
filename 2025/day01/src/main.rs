use std::io::{self, Read};

fn build(input: &str) -> Vec<(char, u32)> {
    input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect()
}

fn actual_password(rotations: &[(char, u32)]) -> u32 {
    let mut pwd = 0;

    let mut pos: i64 = 50;
    for (dir, cnt) in rotations {
        pos = match dir {
            'L' => pos - i64::from(*cnt),
            'R' => pos + i64::from(*cnt),
            _ => panic!("Invalid direction"),
        } % 100;

        if pos == 0 {
            pwd += 1;
        }
    }
    pwd
}

fn part2(rotations: &[(char, u32)]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rotations = build(&input);

    println!("Part 1: {}", actual_password(&rotations));
    println!("Part 2: {}", part2(&rotations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(actual_password(&build(INPUT_TEST)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
