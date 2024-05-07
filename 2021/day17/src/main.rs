use std::{
    cmp::Ordering,
    io::{self, Read},
};

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
    fn new(vel_x: i32, vel_y: i32) -> Self {
        Self {
            vel_x,
            vel_y,
            x: 0,
            y: 0,
        }
    }

    fn step(&self) -> Self {
        Self {
            vel_x: match self.vel_x.cmp(&0) {
                Ordering::Greater => self.vel_x - 1,
                Ordering::Less => self.vel_x + 1,
                Ordering::Equal => self.vel_x,
            },
            vel_y: self.vel_y - 1,
            x: self.x + self.vel_x,
            y: self.y + self.vel_y,
        }
    }

    fn is_in_target(&self, target_area: &TargetArea) -> bool {
        target_area.contains(self.x, self.y)
    }

    // Has the probe overshoot the target?
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

    let mut probe = *probe;
    loop {
        probe = probe.step();
        highest_y = highest_y.max(probe.y);

        if probe.is_past_target(target_area) {
            return None;
        } else if probe.is_in_target(target_area) {
            return Some(highest_y);
        }
    }
}

struct Result {
    max_y: i32,
    hits: usize,
}

fn try_shooting(target_area: &TargetArea) -> Result {
    let mut result = Result {
        max_y: i32::MIN,
        hits: 0,
    };

    // Start value: y velocity can start at target area lowest y, as anything lower
    // get us to shoot lower on first try already.
    // End value: Experiment showed that abs of target area lowest y works, not sure why.
    for vel_y in target_area.y_min..target_area.y_min.abs() {
        // Highest y found for this specific y velocity.
        let mut highest_y_for_vel = i32::MIN;

        // We don't need to try a x velocity bigger than max x, as we overshoot on first try.
        for vel_x in 1..=target_area.x_max {
            let probe = Probe::new(vel_x, vel_y);

            if let Some(highest_y) = shoot(&probe, target_area) {
                // If shot hit, save highest y.
                if highest_y > highest_y_for_vel {
                    highest_y_for_vel = highest_y;
                }
                // For finding highest only, we could break here on else, but not for getting all hits.

                result.hits += 1;
            }
            // If shot missed, try next x.
        }

        result.max_y = highest_y_for_vel.max(result.max_y);
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let target_area = TargetArea::build(&input);

    let result = try_shooting(&target_area);

    println!("Part 1: {}", result.max_y);
    println!("Part 2: {}", result.hits);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(try_shooting(&TargetArea::build(INPUT_TEST)).max_y, 45);
    }

    #[test]
    fn test_part2() {
        assert_eq!(try_shooting(&TargetArea::build(INPUT_TEST)).hits, 112);
    }
}
