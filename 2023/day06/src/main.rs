use std::io::{self, Read};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    const fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn traveled(&self, hold: u64) -> u64 {
        let time_for_moving = self.time - hold;
        let speed = hold;
        time_for_moving * speed
    }

    // Simple version
    fn _count_ways_to_win(&self) -> usize {
        (0..self.time)
            .filter(|h| self.traveled(*h) > self.distance)
            .count()
    }

    // A bit optimized, as we don't need to go through all times.
    fn count_ways_to_win(&self) -> usize {
        usize::try_from(self.time).unwrap()
            - (0..self.time)
                .take_while(|h| self.traveled(*h) <= self.distance)
                .count()
            - (0..self.time)
                .rev()
                .take_while(|h| self.traveled(*h) <= self.distance)
                .count()
    }
}

fn build(input: &str) -> Vec<Race> {
    let mut it = input.lines();
    it.next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .zip(
            it.next()
                .unwrap()
                .strip_prefix("Distance: ")
                .unwrap()
                .split_whitespace()
                .map(|i| i.parse().unwrap()),
        )
        .map(|(t, d)| Race::new(t, d))
        .collect()
}

fn build_converted(input: &str) -> Race {
    let p: Vec<u64> = input
        .lines()
        .map(|line| {
            line.split(':')
                .nth(1)
                .unwrap()
                .replace(' ', "")
                .parse()
                .unwrap()
        })
        .collect();
    Race::new(p[0], p[1])
}

fn find_nb_ways(games: &[Race]) -> usize {
    games.iter().map(Race::count_ways_to_win).product()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let games = build(&input);
    println!("Part 1: {}", find_nb_ways(&games));

    let race = build_converted(&input);
    println!("Part 2: {}", race.count_ways_to_win());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_races() {
        let race = Race::new(7, 9);
        assert_eq!(race.traveled(0), 0);
        assert_eq!(race.traveled(1), 6);
        assert_eq!(race.traveled(2), 10);
        assert_eq!(race.traveled(3), 12);
        assert_eq!(race.traveled(4), 12);
        assert_eq!(race.traveled(5), 10);
        assert_eq!(race.traveled(6), 6);
        assert_eq!(race.traveled(7), 0);
        assert_eq!(race.count_ways_to_win(), 4);

        assert_eq!(Race::new(15, 40).count_ways_to_win(), 8);
        assert_eq!(Race::new(30, 200).count_ways_to_win(), 9);
    }

    static INPUT1_TEST: &str = "\
Time:      7  15   30
Distance:  9  40  200\
";

    static INPUT2_TEST: Race = Race::new(71530, 940200);

    #[test]
    fn test_part1() {
        assert_eq!(find_nb_ways(&build(INPUT1_TEST)), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(INPUT2_TEST.count_ways_to_win(), 71503);
    }
}
