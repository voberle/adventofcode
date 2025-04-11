# Puzzle 19: [Out of date](https://i18n-puzzles.com/puzzle/19/)

To deal with timezones, I have been using the `chrono_tz` crate. That crate ships with the latest version of the timezone database.

Here I'm fortunate that Rust libraries all ship as source and are usually very easy to build locally.

I build 4 versions of the `chrono_tz` crate, each with different versions of the timezone database. I wrote a script that does it all automatically: Downloads the crate source, the timezone database, changes the crate version to work around a cargo limitation and builds the whole thing.

The following script does this:

    ./setup_chrono_tz.sh

In the `Cargo.toml`, with path and package, I can specify 4 different crates, for the different versions.

To then solve the puzzle, I convert each time to the 4 possible UTCs, using the 4 versions of chrono_tz. I count how many stations there are for each time, and the one time that has all the stations is the answer.
