# Day 5: [Binary Boarding](https://adventofcode.com/2020/day/5)

## Part 1

Nothing complicated there. I probably didn't need the Dir enum for such a small code.

## Part 2

Here using a simple vector to map the grid turned out very pratical, as the seat ID happens to be the index of the vector to use.

## Update

I understand now that the seats are just binary numbers, which makes things quite a bit simpler.

Some solved it with simple bash commands, like:

    tr 'FBLR' '0101' <input | sort -nr | head -1 | sed -e's/.*/2i&p/' | dc

Or even shorter:

    dc -e 2i$(tr FBLR 0101 < input | sort -r | head -n 1)p
