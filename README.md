# Advent of Code

My *Advent Of Code* code.

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

I try to have all exercises free of any default Clippy warnings.

I also run Clippy in pedantic mode regularly, and add interesting checks to the workspace cargo file.

    cargo clippy -- -W clippy::all -W clippy::pedantic