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
        let mut prev: u32 = *self.0.first().unwrap();
        let mut direction: i64 = 0; // 1 if increasing, -1 if decreasing

        for n in self.0.iter().skip(1) {
            let diff: i64 = i64::from(*n) - i64::from(prev);
            if direction != 0 {
                if diff * direction < 0 {
                    // Difference and direction have different sign, it means direction is changing and report is unsafe.
                    return false;
                }
            } else {
                // Direction isn't set, setting it.
                direction = diff.signum();
            }

            if !(1..=3).contains(&diff.abs()) {
                // Levels differ by too much, report is unsafe.
                return false;
            }

            prev = *n;
        }
        true
    }

    fn clone_with_level_removed(&self, level_pos: usize) -> Self {
        let mut copy = self.0.clone();
        copy.remove(level_pos);
        Self(copy)
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for pos in 0..self.0.len() {
            let dampened_report = self.clone_with_level_removed(pos);
            if dampened_report.is_safe() {
                return true;
            }
        }

        false
    }
}

fn build(input: &str) -> Vec<Report> {
    input.lines().map(Report::new).collect()
}

fn safe_reports_count(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

fn safe_reports_with_dampener_count(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|r| r.is_safe_with_problem_dampener())
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reports = build(&input);

    println!("Part 1: {}", safe_reports_count(&reports));
    println!("Part 2: {}", safe_reports_with_dampener_count(&reports));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn check_one_element_report() {
        // Input doesn't have one element reports, just making sure code doesn't crash on it.
        assert!(Report::new("2").is_safe());
    }

    #[test]
    fn test_part1() {
        assert_eq!(safe_reports_count(&build(INPUT_TEST)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(safe_reports_with_dampener_count(&build(INPUT_TEST)), 4);
    }
}
