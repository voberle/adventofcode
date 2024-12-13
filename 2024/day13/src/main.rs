use std::io::{self, Read};

use itertools::Itertools;

#[derive(Debug)]
struct Machine {
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
    prize_x: u32,
    prize_y: u32,
}

impl Machine {
    // Press the buttons the specified amount of times.
    // Returns the cost if we got the prize.
    fn press(&self, press_a: u32, press_b: u32) -> Option<u32> {
        let p_x = press_a * self.a_x + press_b * self.b_x;
        let p_y = press_a * self.a_y + press_b * self.b_y;
        if p_x == self.prize_x && p_y == self.prize_y {
            Some(press_a * 3 + press_b)
        } else {
            None
        }
    }
}

fn parse_button(line: &str, name: &str) -> (u32, u32) {
    line.trim_start_matches(name)
        .split(", ")
        .map(|u| u[2..].parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn parse_prize(line: &str) -> (u32, u32) {
    line.trim_start_matches("Prize: ")
        .split(", ")
        .map(|u| u[2..].parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn build(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    let mut it = input.lines();
    loop {
        let button_a = it.next().unwrap();
        let (a_x, a_y) = parse_button(button_a, "Button A: ");
        let button_b = it.next().unwrap();
        let (b_x, b_y) = parse_button(button_b, "Button B: ");
        let prize = it.next().unwrap();
        let (prize_x, prize_y) = parse_prize(prize);
        machines.push(Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        });
        if it.next().is_none() {
            break;
        }
    }
    machines
}

fn min_tokens_win_max(machines: &[Machine]) -> u32 {
    // The equations have only one possible solution, so there is no need to find
    // the minimum, just the first result.
    machines
        .iter()
        .filter_map(|machine| {
            (0..=100)
                .find_map(|press_a| (0..=100).find_map(|press_b| machine.press(press_a, press_b)))
        })
        .sum()
}

fn part2(machines: &[Machine]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let machines = build(&input);

    println!("Part 1: {}", min_tokens_win_max(&machines));
    println!("Part 2: {}", part2(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(min_tokens_win_max(&build(INPUT_TEST)), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
