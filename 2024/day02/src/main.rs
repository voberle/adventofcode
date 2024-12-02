use std::io::{self, Read};

struct Report(Vec<u32>);

impl Report {
    fn new(line: &str) -> Self {
        Self(
            line.split_ascii_whitespace()
                .map(|level| level.parse().unwrap())
                .collect(),
        )
    }

    fn is_safe(&self) -> bool {
        let mut prev = self.0[0];
        let mut sign: i64 = 0;
        for n in self.0.iter().skip(1) {
            let diff: i64 = i64::from(*n) - i64::from(prev);
            if sign != 0 {
                if diff * sign < 0 {
                    // Direction is changing
                    return false;
                }
            } else if diff < 0 {
                sign = -1;
            } else if diff > 0 {
                sign = 1;
            } else {
                return false;
            }

            if !(1..=3).contains(&diff.abs()) {
                return false;
            }

            prev = *n;
        }
        true
    }
}

fn build(input: &str) -> Vec<Report> {
    input.lines().map(Report::new).collect()
}

fn safe_reports_count(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

fn part2(reports: &[Report]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reports = build(&input);

    println!("Part 1: {}", safe_reports_count(&reports));
    println!("Part 2: {}", part2(&reports));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(safe_reports_count(&build(INPUT_TEST)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
