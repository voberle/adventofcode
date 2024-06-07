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

    (min_x..max_x)
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

fn part2(sensor_data: &[SensorData]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sensor_data = build(&input);

    println!(
        "Part 1: {}",
        beacon_not_present_row(&sensor_data, 2_000_000)
    );
    println!("Part 2: {}", part2(&sensor_data));
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
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
