use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use chrono::TimeZone;
use chrono::{DateTime, NaiveDateTime, Utc};
use itertools::Itertools;

struct Signal {
    local_datetime: NaiveDateTime,
    station: String,
}

impl Signal {
    fn to_utc_2018c(&self) -> Option<DateTime<Utc>> {
        let tz: chrono_tz_2018c::Tz = self.station.parse().unwrap();
        Self::convert_to_utc(&tz, &self.local_datetime)
    }

    fn to_utc_2018g(&self) -> Option<DateTime<Utc>> {
        let tz: chrono_tz_2018g::Tz = self.station.parse().unwrap();
        Self::convert_to_utc(&tz, &self.local_datetime)
    }

    fn to_utc_2021b(&self) -> Option<DateTime<Utc>> {
        let tz: chrono_tz_2021b::Tz = self.station.parse().unwrap();
        Self::convert_to_utc(&tz, &self.local_datetime)
    }

    fn to_utc_2023d(&self) -> Option<DateTime<Utc>> {
        let tz: chrono_tz_2023d::Tz = self.station.parse().unwrap();
        Self::convert_to_utc(&tz, &self.local_datetime)
    }

    fn convert_to_utc<T: TimeZone>(
        tz: &T,
        local_datetime: &NaiveDateTime,
    ) -> Option<DateTime<Utc>> {
        match tz.from_local_datetime(local_datetime) {
            chrono::LocalResult::Single(dt) => Some(dt.with_timezone(&Utc)),
            _ => None,
        }
    }
}

fn build(input: &str) -> Vec<Signal> {
    input
        .lines()
        .map(|line| {
            let (time, tz) = line.split("; ").collect_tuple().unwrap();

            let local_datetime = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap();
            Signal {
                local_datetime,
                station: tz.to_string(),
            }
        })
        .collect()
}

fn common_signal_time(signals: &[Signal]) -> String {
    // Map with all the possible UTC times and the associated stations.
    let mut stations_times: HashMap<DateTime<Utc>, HashSet<String>> = HashMap::default();

    for signal in signals {
        for time in [
            signal.to_utc_2018c(),
            signal.to_utc_2018g(),
            signal.to_utc_2021b(),
            signal.to_utc_2023d(),
        ]
        .into_iter()
        .flatten()
        {
            stations_times
                .entry(time)
                .and_modify(|e| {
                    e.insert(signal.station.to_string());
                })
                .or_insert({
                    let mut set = HashSet::default();
                    set.insert(signal.station.to_string());
                    set
                });
        }
    }

    // Find how many unique stations we have.
    let unique_stations: HashSet<String> = stations_times.values().flatten().cloned().collect();

    // Find the signal that has all the stations.
    let common_signal = stations_times
        .iter()
        .find(|(_, stations)| stations.len() == unique_stations.len())
        .unwrap();
    common_signal.0.to_rfc3339()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Answer: {}", common_signal_time(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(
            common_signal_time(&build(INPUT_TEST)),
            "2024-04-09T17:49:00+00:00"
        );
    }
}
