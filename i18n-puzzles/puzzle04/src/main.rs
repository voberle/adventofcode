use std::io::{self, Read};

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono_tz::Tz;

#[derive(Debug)]
struct PlaceTime {
    place: String,
    time: String,
}

impl PlaceTime {
    fn build(line: &str) -> Self {
        let (p, t) = line.trim().split_once(' ').unwrap();
        Self {
            place: p.trim().to_string(),
            time: t.trim().to_string(),
        }
    }

    // Converts the place and time to a DateTime with timezone support.
    fn get_datetime(&self) -> DateTime<Tz> {
        // Gets the timezone from the string.
        let tz: Tz = self.place.parse().unwrap();

        // Parses the time (without any timezone info).
        let local_datetime = NaiveDateTime::parse_from_str(&self.time, "%b %d, %Y, %H:%M").unwrap();

        // Converts the local time to a DateTime one with a timezone.
        tz.from_local_datetime(&local_datetime).unwrap()

        // Converting to UTC isn't necessary: dt.with_timezone(&chrono_tz::UTC)
    }
}

impl From<PlaceTime> for DateTime<Tz> {
    fn from(val: PlaceTime) -> Self {
        val.get_datetime()
    }
}

#[derive(Debug)]
struct Trip {
    departure: PlaceTime,
    arrival: PlaceTime,
}

impl Trip {
    // Returns the duration of the trip in minutes.
    fn duration(&self) -> i64 {
        let delta = self.arrival.get_datetime() - self.departure.get_datetime();
        delta.num_minutes()
    }
}

fn build(input: &str) -> Vec<Trip> {
    let mut itinerary = Vec::new();

    let mut it = input.lines();
    loop {
        let departure = it.next().unwrap().trim_start_matches("Departure:");
        let arrival = it.next().unwrap().trim_start_matches("Arrival:");
        itinerary.push(Trip {
            departure: PlaceTime::build(departure),
            arrival: PlaceTime::build(arrival),
        });
        if it.next().is_none() {
            break;
        }
    }
    itinerary
}

fn answer(itinerary: &[Trip]) -> i64 {
    itinerary.iter().map(Trip::duration).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let itinerary = build(&input);

    println!("Answer: {}", answer(&itinerary));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(answer(&build(INPUT_TEST)), 3143);
    }
}
