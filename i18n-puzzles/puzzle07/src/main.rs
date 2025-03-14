use std::io::{self, Read};

use chrono::{DateTime, FixedOffset, TimeDelta, Timelike};
use chrono_tz::{OffsetComponents, Tz};

// Extract the offset from the timestamp string.
fn get_offset_char(timestamp: &str) -> char {
    timestamp.chars().nth(25).unwrap()
}

#[derive(Debug)]
struct AuditEntry {
    timestamp: String,
    datetime: DateTime<FixedOffset>,
    correct_duration: u32,
    wrong_duration: u32,
}

impl AuditEntry {
    fn build(line: &str) -> Self {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        // Convert the string into DateTime<FixedOffset>
        let datetime = DateTime::parse_from_rfc3339(parts[0]).unwrap();
        Self {
            timestamp: parts[0].to_string(),
            datetime,
            correct_duration: parts[1].parse().unwrap(),
            wrong_duration: parts[2].parse().unwrap(),
        }
    }

    fn get_corrected_time(&self) -> DateTime<Tz> {
        let halifax_time = self.datetime.with_timezone(&Tz::America__Halifax);
        let santiago_time = self.datetime.with_timezone(&Tz::America__Santiago);

        let halifax_dst_offset = halifax_time.offset().dst_offset();
        let santiago_dst_offset = santiago_time.offset().dst_offset();

        let offset_char = get_offset_char(&self.timestamp);
        assert!(offset_char == '3' || offset_char == '4');

        let delta = TimeDelta::minutes(i64::from(self.correct_duration))
            - TimeDelta::minutes(i64::from(self.wrong_duration));

        let corrected_halifax_time = halifax_time.checked_add_signed(delta).unwrap();
        let corrected_santiago_time = santiago_time.checked_add_signed(delta).unwrap();

        // In Halifax, during winter, DST offset is 0 and time string ends with 4.
        if (halifax_dst_offset == TimeDelta::zero() && offset_char == '4')
            || (halifax_dst_offset != TimeDelta::zero() && offset_char == '3')
        {
            corrected_halifax_time
        } else if (santiago_dst_offset == TimeDelta::zero() && offset_char == '3')
            || (santiago_dst_offset != TimeDelta::zero() && offset_char == '4')
        {
            corrected_santiago_time
        } else {
            // Ambiguous, so it shouldn't matter which one we use.
            assert_eq!(corrected_halifax_time, corrected_santiago_time);

            corrected_santiago_time
        }
    }
}

fn build(input: &str) -> Vec<AuditEntry> {
    input.lines().map(AuditEntry::build).collect()
}

fn get_corrected_times(audit_trail: &[AuditEntry]) -> Vec<DateTime<Tz>> {
    audit_trail
        .iter()
        .map(AuditEntry::get_corrected_time)
        .collect()
}

fn calc_answer(correct_times: &[DateTime<Tz>]) -> usize {
    correct_times
        .iter()
        .enumerate()
        .map(|(i, correct_time)| (i + 1) * correct_time.hour() as usize)
        .sum()
}

fn answer(audit_trail: &[AuditEntry]) -> usize {
    let correct_times = get_corrected_times(audit_trail);
    calc_answer(&correct_times)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let audit_trail = build(&input);

    println!("Answer: {}", answer(&audit_trail));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_offset_char() {
        assert_eq!(get_offset_char("2012-11-05T09:39:00.000-04:00"), '4');
    }
    #[test]
    fn test_get_corrected_time() {
        let entry = AuditEntry::build("2012-11-05T09:39:00.000-04:00	969	3358");
        let correct_time = DateTime::parse_from_rfc3339("2012-11-03T18:50:00.000-03:00").unwrap();
        assert_eq!(entry.get_corrected_time(), correct_time);
    }

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");
    const INPUT_TEST_CORRECTED_TIMES: &str = r"2012-11-03T18:50:00.000-03:00
2012-05-29T11:43:00.000-04:00
2001-01-15T21:51:00.000-03:00
2017-05-13T23:40:00.000-03:00
2005-09-02T22:25:00.000-04:00
2008-03-23T15:50:00.000-03:00
2016-03-13T10:23:00.000-03:00
2015-08-16T16:49:00.000-03:00
2013-11-01T07:32:00.000-03:00
2010-04-16T21:51:00.000-04:00";

    #[test]
    fn test_get_corrected_times() {
        let audit_trail = build(INPUT_TEST);
        let corrected_times = get_corrected_times(&audit_trail);
        for (res, exp) in corrected_times
            .iter()
            .zip(INPUT_TEST_CORRECTED_TIMES.lines())
        {
            println!("{}", res);
            assert_eq!(
                res.to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
                exp
            );
        }
    }

    #[test]
    fn test_answer() {
        let audit_trail = build(INPUT_TEST);
        assert_eq!(answer(&audit_trail), 866);
    }
}
