use std::io::{self, Read};

use chrono::{DateTime, Utc};
use itertools::Itertools;

fn build(input: &str) -> Vec<String> {
    input.lines().map(ToString::to_string).collect()
}

fn get_date_time_utc(line: &str) -> DateTime<Utc> {
    // Convert the string into DateTime<FixedOffset>
    let datetime = DateTime::parse_from_rfc3339(line).unwrap();

    // Convert the string into DateTime<Utc>
    datetime.with_timezone(&Utc)
}

fn answer(lines: &[String]) -> String {
    // A hash map with the number of times each item appears.
    let counts = lines.iter().map(|line| get_date_time_utc(line)).counts();

    counts
        .iter()
        .find(|(_time, c)| **c == 4)
        .unwrap()
        .0
        .to_rfc3339()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Answer: {}", answer(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(answer(&build(INPUT_TEST)), "2019-06-05T12:15:00+00:00");
    }
}
