use std::io::{self, Read};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Machine {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    prize_x: u64,
    prize_y: u64,
}

fn parse_button(line: &str, name: &str) -> (u64, u64) {
    line.trim_start_matches(name)
        .split(", ")
        .map(|u| u[2..].parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn parse_prize(line: &str) -> (u64, u64) {
    line.trim_start_matches("Prize: ")
        .split(", ")
        .map(|u| u[2..].parse::<u64>().unwrap())
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

impl Machine {
    fn cost(press_a: u64, press_b: u64) -> u64 {
        press_a * 3 + press_b
    }

    // Press the buttons the specified amount of times.
    // Returns the cost if we got the prize.
    fn press(&self, press_a: u64, press_b: u64) -> Option<u64> {
        let p_x = press_a * self.a_x + press_b * self.b_x;
        let p_y = press_a * self.a_y + press_b * self.b_y;
        if p_x == self.prize_x && p_y == self.prize_y {
            Some(Self::cost(press_a, press_b))
        } else {
            None
        }
    }

    fn brute_solve(&self) -> Option<u64> {
        // The equations have only one possible solution, so there is no need to find
        // the minimum, just the first result.
        (0..=100).find_map(|press_a| (0..=100).find_map(|press_b| self.press(press_a, press_b)))
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn solve(&self) -> Option<u64> {
        let ax = self.a_x as i64;
        let ay = self.a_y as i64;
        let bx = self.b_x as i64;
        let by = self.b_y as i64;
        let px = self.prize_x as i64;
        let py = self.prize_y as i64;

        // We have following equations (a and b are unknown):
        //  a * ax + b * bx = px
        //  a * ay + b * by = py
        // We use the elimination method to get rid of a,
        // multiplying the first by ay and the second by ax:
        //  (a * ax + b * bx) * ay = px * ay
        //  (a * ay + b * by) * ax = py * ax
        // then we substract the second equation from the first:
        //  b * bx * ay - b * by * ax = px * ay - py * ax
        //  b * (bx * ay - by * ax) = px * ay - py * ax
        // giving us b:
        //  b = (px * ay - py * ax) / (bx * ay - by * ax)
        // If this division works without a modulo, we have a b.
        // Then we can get a with:
        //  a = (px - b * bx) / ax
        // and to the same check.

        let num_b = px * ay - py * ax;
        let den_b = bx * ay - by * ax;

        if den_b != 0 && num_b % den_b == 0 {
            let b = num_b / den_b;

            let num_a = px - b * bx;
            let den_a = ax;
            if den_a != 0 && num_a % den_a == 0 {
                let a = num_a / den_a;
                assert!(a > 0 && b > 0);

                let cost = Self::cost(a as u64, b as u64);
                assert_eq!(cost, self.press(a as u64, b as u64).unwrap());
                return Some(cost);
            }
        }
        None
    }

    fn add_to_prize(&self, to_add: u64) -> Self {
        let mut copy = *self;
        copy.prize_x += to_add;
        copy.prize_y += to_add;
        copy
    }
}

#[allow(dead_code)]
fn tokens_win_brute_force(machines: &[Machine]) -> u64 {
    machines.iter().filter_map(Machine::brute_solve).sum()
}

fn tokens_win(machines: &[Machine]) -> u64 {
    machines.iter().filter_map(Machine::solve).sum()
}

fn tokens_win_big_prize(machines: &[Machine]) -> u64 {
    const PRIZE_BONUS: u64 = 10_000_000_000_000;
    machines
        .iter()
        .filter_map(|machine| machine.add_to_prize(PRIZE_BONUS).solve())
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let machines = build(&input);

    // println!("Part 1 brute force: {}", tokens_win_brute_force(&machines));
    println!("Part 1: {}", tokens_win(&machines));
    println!("Part 2: {}", tokens_win_big_prize(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(tokens_win_brute_force(&build(INPUT_TEST)), 480);
        assert_eq!(tokens_win(&build(INPUT_TEST)), 480);
    }
}
