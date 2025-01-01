# Day 24: [Crossed Wires](https://adventofcode.com/2024/day/24)

## Part 1

Fairly simple gate simulation.

After an initial implementation using `String` and maps, I reimplemented it to refer to all gates by indexes only. That was about 10 times faster for the execution part.

## Part 2

cat resources/input| python3 generate_graph.py > input.gv
dot -Tpdf input.gv -o input.pdf -Kdot    