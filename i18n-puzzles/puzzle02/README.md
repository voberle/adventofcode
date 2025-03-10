# Puzzle 2: [Detecting gravitational waves](https://i18n-puzzles.com/puzzle/2/)

We need to use an external crate for managing the timezones, using the `chrono` crate.

The timestamps are in the RFC 3339 format. The chrono crate offers a way to parse it from that format and convert it to UTC.

With that, I'm using the itertools `count()` to find the number of occurences of each date.
