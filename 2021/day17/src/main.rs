use std::io::{self, Read};

use regex::Regex;

struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetArea {
    fn build(input: &str) -> Self {
        let re = Regex::new(r"target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)").unwrap();
        let p = re.captures(input).unwrap();
        Self {
            x_min: p[1].parse().unwrap(),
            x_max: p[2].parse().unwrap(),
            y_min: p[3].parse().unwrap(),
            y_max: p[4].parse().unwrap(),
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        (self.x_min..=self.x_max).contains(&x) && (self.y_min..=self.y_max).contains(&y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    // Initial velocity
    vel_x: i32,
    vel_y: i32,
    // Current positon
    x: i32,
    y: i32,
}

impl Probe {
    fn new(x_vel: i32, y_vel: i32) -> Self {
        Self {
            vel_x: x_vel,
            vel_y: y_vel,
            x: 0,
            y: 0,
        }
    }

    fn step(&self) -> Self {
        let x = self.x + self.vel_x;
        let y = self.y + self.vel_y;
        let x_vel = if self.vel_x > 0 {
            self.vel_x - 1
        } else if self.vel_x < 0 {
            self.vel_x + 1
        } else { // doesn't change if already 0
            self.vel_x
        };
        let y_vel = self.vel_y - 1;
        Self {
            vel_x: x_vel,
            vel_y: y_vel,
            x,
            y,
        }
    }

    fn is_in_target(&self, target_area: &TargetArea) -> bool {
        target_area.contains(self.x, self.y)
    }

    fn is_past_target(&self, target_area: &TargetArea) -> bool {
        self.x > target_area.x_max || self.y < target_area.y_min
    }
}

// Moves the probe until it either reaches the target area or misses it definitively.
// If the target is reached, returns the highest y of this shot.
// If the target is missed, returns None.
fn shoot(probe: &Probe, target_area: &TargetArea) -> Option<i32> {
    // Highest y in this shot.
    let mut highest_y = i32::MIN;

    let mut probe = probe.clone();
    loop {
        probe = probe.step();
        highest_y = highest_y.max(probe.y);

        if probe.is_past_target(target_area) {
            // println!(
            //     ".. [{},{}] {},{}: Missed",
            //     probe.vel_x, probe.vel_y, probe.x, probe.y
            // );
            return None;
        } else if probe.is_in_target(target_area) {
            // println!(
            //     ".. [{},{}] {},{}: Hit",
            //     probe.vel_x, probe.vel_y, probe.x, probe.y
            // );
            return Some(highest_y);
        }
        // println!(
        //     ".. [{},{}] {},{}: ..",
        //     probe.vel_x, probe.vel_y, probe.x, probe.y
        // );
    }
}

fn highest_possible_y(target_area: &TargetArea) -> i32 {
    let mut max_y = i32::MIN;
    for vel_y in 1.. {
        if vel_y < target_area.y_min {
            break;
        }

        // Highest y found for this specific y velocity.
        let mut highest_y_for_vel = i32::MIN;
        for vel_x in 1.. {
            // If velocity x is so big that we would miss on first try, then stop
            if vel_x > target_area.x_max {
                break;
            }
            
            let probe = Probe::new(vel_x, vel_y);
            if let Some(highest_y) = shoot(&probe, target_area) {
                // If shot hit, save highest y.
                if highest_y > highest_y_for_vel {
                    highest_y_for_vel = highest_y;
                } else {
                    // we found highest for this y velocity, try next.
                    // println!("{vel_x},{vel_y}: {}, {}, {}", highest_y, highest_y_for_vel, max_y);
                    break;
                }
                // } else {
                //     // If we missed the target with the smallest x, then this y is bad.
                //     if highest_y_for_vel == i32::MIN {
                //         break;
                //     }
            }
            // If shot missed, try next x.
        }

        if highest_y_for_vel > max_y {
            max_y = highest_y_for_vel;
            println!("{}", max_y);
        } else {
            // break;
        }
    }

    // println!(" {},{}: {}", probe.x, probe.y, res);

    max_y
}

fn part2(target_area: &TargetArea) -> i32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let target_area = TargetArea::build(&input);

    println!("Part 1: {}", highest_possible_y(&target_area));
    println!("Part 2: {}", part2(&target_area));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(highest_possible_y(&TargetArea::build(INPUT_TEST)), 45);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TargetArea::build(INPUT_TEST)), 0);
    }
}
