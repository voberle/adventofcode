# AoC Intcode Computer

The Intcode computer as a library.

## Running a program

The main binary is a program executor, executing the Intcode program passed as argument, using stdin for input.

For example:

    echo "1" | cargo r --bin main --release -- ../day09/resources/input

## ASCII interface

To use the Aft Scaffolding Control and Information Interface (ASCII), for example to run the [Intcode assembler](https://github.com/matushorvath/xzintbit?tab=readme-ov-file):

    cargo r --bin ascii --release -- as.input < hello-world.s > hello-world.o
    echo .$ | cat hello-world.o - | cargo r --bin ascii --release -- ld.input > hello-world.input
    cargo r --bin ascii --release -- hello-world.input

## Testing

Run all the supported puzzles with:

    cargo r --bin previous_days --release
