use std::io::{self, Read};

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone, Utc, Weekday};
use chrono_tz::Tz;
use itertools::Itertools;

// Is this date (in local timezone) during the office working hours?
// Working day is from 08:30 to 17:00.
fn is_office_working_hour(local_dt: &DateTime<Tz>) -> bool {
    const START_DAY: NaiveTime = NaiveTime::from_hms_opt(8, 30, 0).unwrap();
    const END_DAY: NaiveTime = NaiveTime::from_hms_opt(17, 0, 0).unwrap();

    (START_DAY..END_DAY).contains(&local_dt.time())
}

// Is this date (in local timezone) during the working week?
fn is_working_day(local_dt: &DateTime<Tz>) -> bool {
    [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
    ]
    .contains(&local_dt.weekday())
}

// Represents a location, either an office or a customer.
#[derive(Debug)]
struct Location {
    #[allow(dead_code)]
    name: String,
    timezone: Tz,
    public_holidays: Vec<NaiveDate>,
}

impl Location {
    // Finds the position of where the timezone begins.
    // That feels clumsy, how to do it better?
    fn get_timezone_beg(line: &str, first_slash_pos: usize) -> usize {
        let mut timezone_beg = 0;
        for (i, c) in line.bytes().enumerate() {
            if c.is_ascii_whitespace() {
                timezone_beg = i;
            }
            if i == first_slash_pos {
                break;
            }
        }
        timezone_beg
    }

    fn get_timezone_end(line: &str, first_slash_pos: usize) -> usize {
        line.bytes()
            .skip(first_slash_pos)
            .position(|c| c.is_ascii_whitespace())
            .unwrap()
            + first_slash_pos
    }

    fn build(line: &str) -> Self {
        // We use the fact that the timezone is in the middle and the only element with '/' (one or two).
        // Not that we can deal in bytes, so use find and slices.
        let first_slash_pos = line.find('/').unwrap();
        let timezone_beg = Self::get_timezone_beg(line, first_slash_pos);
        let timezone_end = Self::get_timezone_end(line, first_slash_pos);

        let name = line[0..timezone_beg].trim().to_string();

        let timezone_str = line[timezone_beg..timezone_end].trim().to_string();
        // Gets the timezone from the string.
        let timezone: Tz = timezone_str.parse().unwrap();

        let public_holidays = line[timezone_end..]
            .trim()
            .split(';')
            .map(|s| {
                // Parses the time (without any timezone info).
                NaiveDate::parse_from_str(s, "%d %B %Y").unwrap()
            })
            .collect_vec();

        Self {
            name,
            timezone,
            public_holidays,
        }
    }

    fn is_public_holiday(&self, local_dt: &DateTime<Tz>) -> bool {
        let day = local_dt.day();
        let month = local_dt.month();
        self.public_holidays
            .iter()
            .any(|d| d.day() == day && d.month() == month)
    }

    // For offices: Is this office working at this minute?
    fn is_office_working_at(&self, utc_dt: &DateTime<Utc>) -> bool {
        // Convert utc time to time in this timezone.
        let local_dt = utc_dt.with_timezone(&self.timezone);

        is_office_working_hour(&local_dt)
            && is_working_day(&local_dt)
            && !self.is_public_holiday(&local_dt)
    }

    // For customers: Is this minute in the working week (Monday to Friday)?
    fn can_request_support_at(&self, utc_dt: &DateTime<Utc>) -> bool {
        let local_dt = utc_dt.with_timezone(&self.timezone);

        // Customer support is available 24h, but only on working week.
        is_working_day(&local_dt) && !self.is_public_holiday(&local_dt)
    }
}

fn build(input: &str) -> (Vec<Location>, Vec<Location>) {
    let (offices, customers) = input.split("\n\n").collect_tuple().unwrap();
    (
        offices.lines().map(Location::build).collect(),
        customers.lines().map(Location::build).collect(),
    )
}

// We calculate the overtime by steps of 30 minutes, as this is the smallest
// divider of work time and timezones.
const STEP_MINUTES: i64 = 30;

// Calculate for each minutes in the interval if an office is working.
fn calc_offices_availability(
    offices: &[Location],
    from_incl: DateTime<Utc>,
    to_excl: DateTime<Utc>,
) -> Vec<bool> {
    let step_duration = Duration::minutes(STEP_MINUTES);

    let mut offices_availability = Vec::new();
    let mut utc_dt = from_incl;
    while utc_dt < to_excl {
        offices_availability.push(
            offices
                .iter()
                .any(|office| office.is_office_working_at(&utc_dt)),
        );
        utc_dt += step_duration;
    }
    offices_availability
}

#[allow(clippy::cast_sign_loss)]
fn calc_overtime_for(
    offices_availability: &[bool],
    customer: &Location,
    from_incl: DateTime<Utc>,
    to_excl: DateTime<Utc>,
) -> u64 {
    let step_duration = Duration::minutes(STEP_MINUTES);

    let mut overtime = 0;

    let mut utc_dt = from_incl;
    let mut i = 0;
    while utc_dt < to_excl {
        if customer.can_request_support_at(&utc_dt) && !offices_availability[i] {
            overtime += STEP_MINUTES;
        }
        utc_dt += step_duration;
        i += 1;
    }

    overtime as u64
}

fn answer(offices: &[Location], customers: &[Location]) -> u64 {
    // Approach is to look day by day, UTC ones, how much overtime each needs.
    // Only going through working days, not Saturday/Sunday.

    let from_incl: DateTime<Utc> = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
    let to_excl: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

    // We calculate in advance if offices are available for each minute.
    let offices_availability = calc_offices_availability(offices, from_incl, to_excl);

    if let itertools::MinMaxResult::MinMax(min, max) = customers
        .iter()
        .map(|customer| calc_overtime_for(&offices_availability, customer, from_incl, to_excl))
        .minmax()
    {
        max - min
    } else {
        panic!("No min max found")
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (offices, customers) = build(&input);

    println!("Answer: {}", answer(&offices, &customers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_working() {
        let location = Location::build(
            "TOPlap office in Melbourne	Australia/Melbourne	26 December 2022;15 April 2022;18 April 2022;26 January 2022",
        );

        let dt = Utc.with_ymd_and_hms(2022, 4, 11, 0, 0, 0).unwrap();
        assert!(location.is_office_working_at(&dt));

        let dt = Utc.with_ymd_and_hms(2022, 4, 11, 8, 0, 0).unwrap();
        assert!(!location.is_office_working_at(&dt));
    }

    #[test]
    fn test_calc_overtime_day1() {
        let (offices, customers) = build(INPUT_TEST);

        // 9 December 2022
        let from = Utc.with_ymd_and_hms(2022, 12, 9, 0, 0, 0).unwrap();
        let to = Utc.with_ymd_and_hms(2022, 12, 10, 0, 0, 0).unwrap();

        let availability = calc_offices_availability(&offices, from, to);

        assert_eq!(
            calc_overtime_for(&availability, &customers[0], from, to),
            210
        );
        assert_eq!(
            calc_overtime_for(&availability, &customers[1], from, to),
            210
        );
        assert_eq!(
            calc_overtime_for(&availability, &customers[2], from, to),
            90
        );
    }

    #[test]
    fn test_calc_overtime_day2() {
        let (offices, customers) = build(INPUT_TEST);

        // 18 April 2022
        let from = Utc.with_ymd_and_hms(2022, 4, 18, 0, 0, 0).unwrap();
        let to = Utc.with_ymd_and_hms(2022, 4, 19, 0, 0, 0).unwrap();

        let availability = calc_offices_availability(&offices, from, to);

        assert_eq!(
            calc_overtime_for(&availability, &customers[0], from, to),
            300
        );
        assert_eq!(calc_overtime_for(&availability, &customers[1], from, to), 0);
        assert_eq!(
            calc_overtime_for(&availability, &customers[2], from, to),
            480
        );
    }

    #[test]
    fn test_calc_overtime_year() {
        let (offices, customers) = build(INPUT_TEST);

        // Year 2022
        let from = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let to = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        let availability = calc_offices_availability(&offices, from, to);

        assert_eq!(
            calc_overtime_for(&availability, &customers[0], from, to),
            41730
        );
        assert_eq!(
            calc_overtime_for(&availability, &customers[1], from, to),
            41820
        );
        assert_eq!(
            calc_overtime_for(&availability, &customers[2], from, to),
            44760
        );
    }

    #[test]
    fn test_answer() {
        let (offices, customers) = build(INPUT_TEST);
        assert_eq!(answer(&offices, &customers), 3030);
    }
}
