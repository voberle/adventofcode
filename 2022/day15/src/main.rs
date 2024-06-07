use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Pos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn get_tuning_freq(&self) -> i64 {
        i64::from(self.x) * 4_000_000 + i64::from(self.y)
    }
}

#[derive(Debug)]
struct Square {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Square {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn get_top_left(&self) -> Pos {
        Pos::new(self.min_x, self.min_y)
    }

    fn get_top_right(&self) -> Pos {
        Pos::new(self.max_x, self.min_y)
    }

    fn get_bottom_left(&self) -> Pos {
        Pos::new(self.min_x, self.max_y)
    }

    fn get_bottom_right(&self) -> Pos {
        Pos::new(self.max_x, self.max_y)
    }

    fn is_dot(&self) -> bool {
        self.min_x == self.max_x && self.min_y == self.max_y
    }
}

#[derive(Debug)]
struct SensorData {
    sensor: Pos,
    beacon: Pos,
    rayon: u32,
}

impl SensorData {
    fn in_range(&self, pos: &Pos) -> bool {
        self.sensor.distance(pos) <= self.rayon
    }

    fn square_in_range(&self, square: &Square) -> bool {
        self.sensor.distance(&square.get_top_left()) <= self.rayon
            && self.sensor.distance(&square.get_top_right()) <= self.rayon
            && self.sensor.distance(&square.get_bottom_left()) <= self.rayon
            && self.sensor.distance(&square.get_bottom_right()) <= self.rayon
    }

    #[allow(clippy::cast_possible_wrap)]
    fn minmax_x(&self) -> (i32, i32) {
        (
            self.sensor.x - self.rayon as i32,
            self.sensor.x + self.rayon as i32,
        )
    }
}

fn build(input: &str) -> Vec<SensorData> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let p = re.captures(line).unwrap();
            let sensor = Pos::new(p[1].parse().unwrap(), p[2].parse().unwrap());
            let beacon = Pos::new(p[3].parse().unwrap(), p[4].parse().unwrap());
            let rayon = sensor.distance(&beacon);
            SensorData {
                sensor,
                beacon,
                rayon,
            }
        })
        .collect()
}

fn beacon_not_present_row(sensor_data: &[SensorData], row: i32) -> usize {
    let (min_x, max_x) = sensor_data
        .iter()
        .map(SensorData::minmax_x)
        .fold((i32::MAX, i32::MIN), |acc, e| {
            (acc.0.min(e.0), acc.1.max(e.1))
        });
    // println!("min_x={} max_x={}", min_x, max_x);

    (min_x..=max_x)
        .filter(|x| {
            let pos = Pos::new(*x, row);
            sensor_data.iter().any(|sd| {
                // If a beacon is found at the position, return false immediately.
                if sd.beacon == pos {
                    return false;
                }
                // Otherwise, check if it's in range.
                sd.in_range(&pos)
            })
        })
        .count()
}

// Brute-force version, only working on small test data.
#[allow(dead_code)]
fn distress_signal_tuning_freq_brute_force(sensor_data: &[SensorData], max: i32) -> i64 {
    const MIN: i32 = 0;

    for x in MIN..=max {
        for y in MIN..=max {
            let pos = Pos::new(x, y);
            if sensor_data.iter().all(|sd| !sd.in_range(&pos)) {
                return pos.get_tuning_freq();
            }
        }
    }
    panic!("No distress signal found")
}

// We look if a square is fully in range of a sensor.
// If it is, no need to check it further.
// If it isn't, we divide the square in 4 and check each again.
fn check_square(sensor_data: &[SensorData], square: &Square) -> Option<i64> {
    if square.is_dot() {
        let pos = Pos::new(square.min_x, square.min_y);
        if sensor_data.iter().all(|sd| !sd.in_range(&pos)) {
            // println!("Position: {},{}", pos.x, pos.y);
            return Some(pos.get_tuning_freq());
        }
        return None;
    }

    if sensor_data.iter().any(|sd| sd.square_in_range(square)) {
        // Square is fully in the range of a sensor, ignore it.
        return None;
    }

    // Divide the square in 4 and check each.
    let middle_x = (square.min_x + square.max_x) / 2;
    let middle_y = (square.min_y + square.max_y) / 2;
    let r = check_square(
        sensor_data,
        &Square::new(square.min_x, square.min_y, middle_x, middle_y),
    );
    if r.is_some() {
        return r;
    }
    let r = check_square(
        sensor_data,
        &Square::new(middle_x + 1, square.min_y, square.max_x, middle_y),
    );
    if r.is_some() {
        return r;
    }
    let r = check_square(
        sensor_data,
        &Square::new(square.min_x, middle_y + 1, middle_x, square.max_y),
    );
    if r.is_some() {
        return r;
    }
    check_square(
        sensor_data,
        &Square::new(middle_x + 1, middle_y + 1, square.max_x, square.max_y),
    )
}

fn distress_signal_tuning_freq(sensor_data: &[SensorData], max: i32) -> i64 {
    const MIN: i32 = 0;

    check_square(sensor_data, &Square::new(MIN, MIN, max, max)).expect("No distress signal found")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sensor_data = build(&input);

    println!(
        "Part 1: {}",
        beacon_not_present_row(&sensor_data, 2_000_000)
    );
    println!(
        "Part 2: {}",
        distress_signal_tuning_freq(&sensor_data, 4_000_000)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(beacon_not_present_row(&build(INPUT_TEST), 10), 26);
    }

    #[test]
    fn test_part2_brute_force() {
        assert_eq!(
            distress_signal_tuning_freq_brute_force(&build(INPUT_TEST), 20),
            56000011
        );
    }
}
