use std::io::{self, Read};

use regex::Regex;

const fn wrap(i: usize, len: usize) -> usize {
    (i % len + len) % len
}

struct Disc {
    position_count: usize,
    initial_time: usize,
    initial_position: usize,
}

impl Disc {
    fn position_at(&self, time: usize) -> usize {
        let offset = time - self.initial_time;
        wrap(self.initial_position + offset, self.position_count)
    }
}

fn build(input: &str) -> Vec<Disc> {
    let re =
        Regex::new(r"Disc #(\d+) has (\d+) positions; at time=(\d+), it is at position (\d+).")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let parts = re.captures(line).unwrap();
            Disc {
                position_count: parts[2].parse().unwrap(),
                initial_time: parts[3].parse().unwrap(),
                initial_position: parts[4].parse().unwrap(),
            }
        })
        .collect()
}

fn first_time_to_press_button(discs: &[Disc]) -> usize {
    (0..usize::MAX)
        .position(|time| {
            discs
                .iter()
                .enumerate()
                .all(|(t, d)| d.position_at(time + t + 1) == 0)
        })
        .expect("Didn't find a result")
}

fn part2(discs: &[Disc]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let discs = build(&input);

    println!("Part 1: {}", first_time_to_press_button(&discs));
    println!("Part 2: {}", part2(&discs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_disc_position_at() {
        let disc = Disc {
            position_count: 5,
            initial_time: 0,
            initial_position: 4,
        };
        assert_eq!(disc.position_at(0), 4);
        assert_eq!(disc.position_at(1), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(first_time_to_press_button(&build(INPUT_TEST)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
