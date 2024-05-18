use std::io::{self, Read};

#[derive(Debug)]
struct Cuboid {
    x_low: i32,
    x_high: i32,
    y_low: i32,
    y_high: i32,
    z_low: i32,
    z_high: i32,
}

impl Cuboid {
    fn build(line: &str) -> Self {
        let p: Vec<_> = line
            .split(',')
            .flat_map(|axe| axe[2..].split("..").map(|c| c.parse().unwrap()))
            .collect();
        Self {
            x_low: p[0],
            x_high: p[1],
            y_low: p[2],
            y_high: p[3],
            z_low: p[4],
            z_high: p[5],
        }
    }

    fn is_initialization(&self) -> bool {
        self.x_low.abs() <= 50
            && self.x_high.abs() <= 50
            && self.y_low.abs() <= 50
            && self.y_high.abs() <= 50
            && self.z_low.abs() <= 50
            && self.z_high.abs() <= 50
    }

    fn all_cubes(&self) -> Vec<(i32, i32, i32)> {
        (self.x_low..=self.x_high)
            .flat_map(|x| {
                (self.y_low..=self.y_high)
                    .flat_map(move |y| (self.z_low..=self.z_high).map(move |z| (x, y, z)))
            })
            .collect()
    }
}

fn build(input: &str) -> Vec<(bool, Cuboid)> {
    input
        .lines()
        .map(|line| {
            if let Some(coords) = line.strip_prefix("on ") {
                (true, Cuboid::build(coords))
            } else if let Some(coords) = line.strip_prefix("off ") {
                (false, Cuboid::build(coords))
            } else {
                panic!("Invalid input")
            }
        })
        .collect()
}

#[allow(clippy::cast_sign_loss)]
fn cubes_on_after_init(reboot_steps: &[(bool, Cuboid)]) -> usize {
    // Brute-force version, unlikely to work for part 2 :-(
    let mut reactor = vec![vec![vec![false; 101]; 101]; 101];
    for (action, cuboid) in reboot_steps.iter().filter(|(_, s)| s.is_initialization()) {
        for c in cuboid.all_cubes() {
            reactor[(c.0 + 50) as usize][(c.1 + 50) as usize][(c.2 + 50) as usize] = *action;
        }
    }
    reactor.iter().flatten().flatten().filter(|v| **v).count()
}

fn part2(reboot_steps: &[(bool, Cuboid)]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reboot_steps = build(&input);

    println!("Part 1: {}", cubes_on_after_init(&reboot_steps));
    println!("Part 2: {}", part2(&reboot_steps));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_2: &str = include_str!("../resources/input_test_2");

    #[test]
    fn test_part1() {
        assert_eq!(cubes_on_after_init(&build(INPUT_TEST_1)), 39);
        assert_eq!(cubes_on_after_init(&build(INPUT_TEST_2)), 590784);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
