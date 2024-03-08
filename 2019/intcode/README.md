# AoC Intcode Computer

The Intcode computer as a library.

## Running a program

The main binary is a program executor, executing the Intcode program passed as argument, using stdin for input.

For example:

    echo "1" | cargo r --bin main --release -- ../day09/resources/input

## Testing

Run all the supported puzzles with:

    cargo r --bin previous_days --release
