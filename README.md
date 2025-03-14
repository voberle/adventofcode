# Advent of Code

My *Advent Of Code* code.

It contains the solutions from all years since 2015.

It also has some of the Advent of Code clones solutions:

- [Internationali­zation Puzzles](i18n-puzzles).
- [Pi Coding Quest](https://ivanr3d.com/projects/pi/).

## Template

`template` folder has a template to be used with [Cargo Generate](https://cargo-generate.github.io/cargo-generate):

    cd YEAR
    cargo generate aoc --name dayXX

or better using the [aoc_new tool](tools/aoc_new/README.md), in YEAR directory:

    ../aoc_new X

with following in `$CARGO_HOME/cargo-generate.toml`:

    [favorites.aoc]
    description = "Advent of Code template"
    path = "FULL_PATH_TO_TEMPLATE"
    vcs = "Git"

## Clippy

All exercises are free of any Clippy warnings. Clippy is set by default in pedantic mode in the workspace cargo file.
