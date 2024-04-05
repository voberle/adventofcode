# AoC Intcode Computer

The Intcode computer as a library.

## Running a program

The main binary is a program executor, executing the Intcode program passed as argument, using stdin for input.

For example:

    echo "1" | cargo r --bin main --release -- ../day09/resources/input

## ASCII interface

To use the Aft Scaffolding Control and Information Interface (ASCII), there is the ascii binary.

For example to run the [Intcode assembler](https://github.com/matushorvath/xzintbit?tab=readme-ov-file):

    cargo r --bin ascii --release -- xzintbit/as.input < xzintbit/hello-world.s > xzintbit/hello-world.o
    echo .$ | cat xzintbit/hello-world.o - | cargo r --bin ascii --release -- xzintbit/ld.input > xzintbit/hello-world.input
    cargo r --bin ascii --release -- xzintbit/hello-world.input

Or the day 25 game:

    cargo r --bin ascii --release -- ../day25/resources/input

## Testing

Run all the supported puzzles with:

    cargo r --bin previous_days --release
