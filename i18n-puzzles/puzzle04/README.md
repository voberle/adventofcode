# Puzzle 4: [A trip around the world](https://i18n-puzzles.com/puzzle/4/)

The `chrono_tz` allows us to convert a city name into a `Tz` struct.

With the `chrono` crate used previously, we use `NaiveDateTime::parse_from_str()` to parse the time string, specifying the format used.

These two information are combined to get a `DateTime<Tz>`, a date with the timezone support.
