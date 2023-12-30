# Advent of Code

## 2023

My [first participation to AoC](2023/README.md).

## Template

`template` folder has a template to be used with [Cargo Generate](https://cargo-generate.github.io/cargo-generate):

    cd YEAR
    cargo generate aoc --name dayXX

with following in `$CARGO_HOME/cargo-generate.toml`:

    [favorites.aoc]
    description = "Advent of Code template"
    path = "FULL_PATH_TO_TEMPLATE"
    vcs = "Git"
