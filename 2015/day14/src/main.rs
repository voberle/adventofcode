use std::io::{self, Read};

use regex::Regex;

struct Reindeer {
    #[allow(dead_code)]
    name: String,
    flying_dist: u32,
    flying_time: u32,
    resting_time: u32,
}

fn build(input: &str) -> Vec<Reindeer> {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Reindeer {
                name: caps[1].to_string(),
                flying_dist: caps[2].parse().unwrap(),
                flying_time: caps[3].parse().unwrap(),
                resting_time: caps[4].parse().unwrap(),
            }
        })
        .collect()
}

fn dist_after(r: &Reindeer, time: u32) -> u32 {
    let flying_period = r.flying_time + r.resting_time;
    // How many full flying periods could he do
    let full_flying_periods = time / flying_period;

    let remaining_time = time - full_flying_periods * flying_period;
    let remainder_flying_time = if remaining_time > r.flying_time {
        r.flying_time
    } else {
        remaining_time
    };

    (full_flying_periods * r.flying_time + remainder_flying_time) * r.flying_dist
}

fn max_dist_after(reindeers: &[Reindeer], time: u32) -> u32 {
    reindeers.iter().map(|r| dist_after(r, time)).max().unwrap()
}

fn part2(reindeers: &[Reindeer]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reindeers = build(&input);

    println!("Part 1: {}", max_dist_after(&reindeers, 2503));
    println!("Part 2: {}", part2(&reindeers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_dist_after() {
        let comet = Reindeer {
            name: "Comet".to_string(),
            flying_dist: 14,
            flying_time: 10,
            resting_time: 127,
        };
        let dancer = Reindeer {
            name: "Dancer".to_string(),
            flying_dist: 16,
            flying_time: 11,
            resting_time: 162,
        };

        assert_eq!(dist_after(&comet, 1), 14);
        assert_eq!(dist_after(&dancer, 1), 16);
        assert_eq!(dist_after(&comet, 10), 140);
        assert_eq!(dist_after(&dancer, 10), 160);
        assert_eq!(dist_after(&comet, 11), 140);
        assert_eq!(dist_after(&dancer, 11), 176);
        assert_eq!(dist_after(&comet, 12), 140);
        assert_eq!(dist_after(&dancer, 12), 176);
        assert_eq!(dist_after(&comet, 137), 140);
        assert_eq!(dist_after(&comet, 147), 280);
        assert_eq!(dist_after(&dancer, 185), 352);

        assert_eq!(dist_after(&comet, 1000), 1120);
        assert_eq!(dist_after(&dancer, 1000), 1056);
    }

    #[test]
    fn test_part1() {
        assert_eq!(max_dist_after(&build(INPUT_TEST), 1000), 1120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
