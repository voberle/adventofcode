# New day creation for AoC

A little program that automates the creation of a new AoC day.

## Usage

You must be in the year directory, then just:

    aoc_new 12

## Dependencies

- Advent of Code command-line tool [aoc-cli](https://github.com/scarvalhojr/aoc-cli)

## Starting a new year _aka day 1_

All challenges are in corresponding year sub-directory, and share a common `Cargo.toml` with workspaces. So when starting a new year:

- Create a new YYYY directory.
- Add a `Cargo.toml` file to it (copy from previous year and remove all the "day" entries from "members" section).

Then proceed to do like other days.