use std::io::{self, Read};

use chrono::{NaiveDateTime, Timelike};
use fxhash::FxHashMap;
use regex::Regex;

#[derive(Debug)]
enum Event {
    ShiftBegins(usize),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct Entry {
    time: NaiveDateTime, // a date without a timezome.
    event: Event,
}

impl Entry {
    fn get_minute(&self) -> u32 {
        self.time.minute()
    }
}

fn build_sorted_entries(input: &str) -> Vec<Entry> {
    let re_main = Regex::new(r"\[(.+)\] (.+)").unwrap();
    let re_event = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut entries: Vec<Entry> = input
        .lines()
        .map(|line| {
            let p = re_main.captures(line).unwrap();
            let time = NaiveDateTime::parse_from_str(&p[1], "%Y-%m-%d %H:%M").unwrap();
            let event = if p[2].starts_with("Guard") {
                let pe = re_event.captures(&p[2]).unwrap();
                Event::ShiftBegins(pe[1].parse().unwrap())
            } else if &p[2] == "falls asleep" {
                Event::FallsAsleep
            } else if &p[2] == "wakes up" {
                Event::WakesUp
            } else {
                panic!("Invalid input");
            };
            Entry { time, event }
        })
        .collect();
    // Input isn't sorted.
    entries.sort_by_key(|e| e.time);
    entries
}

fn increase_sleeping_minutes(hour: &mut [usize], from: u32, to: u32) {
    for i in from..to {
        hour[i as usize] += 1;
    }
}

fn asleep_count(hour: &[usize]) -> usize {
    hour.iter().sum()
}

// For each guard, how often were they at sleep at the specified minute
fn get_guards_sleeping_schedule(entries: &[Entry]) -> FxHashMap<usize, Vec<usize>> {
    let mut guards_sleeping_schedule: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    let mut asleep_range: (u32, u32) = (0, 0);
    let mut current_guard = 0; // assuming no guard with ID 0
    for e in entries {
        match e.event {
            Event::ShiftBegins(id) => {
                current_guard = id;
            }
            Event::FallsAsleep => {
                asleep_range.0 = e.get_minute();
            }
            Event::WakesUp => {
                // Assuming guards always wake up before next shift
                asleep_range.1 = e.get_minute();
                assert_ne!(current_guard, 0);
                guards_sleeping_schedule
                    .entry(current_guard)
                    .and_modify(|hour| {
                        increase_sleeping_minutes(hour, asleep_range.0, asleep_range.1);
                    })
                    .or_insert({
                        let mut hour = vec![0; 60];
                        increase_sleeping_minutes(&mut hour, asleep_range.0, asleep_range.1);
                        hour
                    });
            }
        }
    }
    guards_sleeping_schedule
}

fn strategy_1_result(entries: &[Entry]) -> usize {
    let guards_sleeping_schedule = get_guards_sleeping_schedule(entries);

    let (guard_id, hour) = guards_sleeping_schedule
        .iter()
        .max_by_key(|(_, hour)| asleep_count(hour))
        .unwrap();
    let minute = hour.iter().enumerate().max_by_key(|(_, s)| *s).unwrap().0;

    guard_id * minute
}

fn strategy_2_result(entries: &[Entry]) -> usize {
    let guards_sleeping_schedule = get_guards_sleeping_schedule(entries);

    let (guard_id, minute, _) = guards_sleeping_schedule
        .iter()
        .map(|(guard_id, v)| {
            let (minute, asleep_count) = v.iter().enumerate().max_by_key(|(_, s)| *s).unwrap();
            (guard_id, minute, asleep_count)
        })
        .max_by_key(|(_guard_id, _minute, asleep_count)| *asleep_count)
        .unwrap();

    guard_id * minute
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let entries = build_sorted_entries(&input);
    // for e in &entries {
    //     println!("{:?}", e);
    // }

    println!("Part 1: {}", strategy_1_result(&entries));
    println!("Part 2: {}", strategy_2_result(&entries));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(strategy_1_result(&build_sorted_entries(INPUT_TEST)), 240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(strategy_2_result(&build_sorted_entries(INPUT_TEST)), 4455);
    }
}
