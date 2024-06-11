# Day 17: [Pyroclastic Flow](https://adventofcode.com/2022/day/17)

## Part 1

Beautiful puzzle, having to reimplement a Tetris.

I was stuck for a bit, with test giving the right answer but not real input. Insight was add a check to make sure I didn't overwrite any unit.

Debug logging can be enabled with:

    --features my_debug

## Part 2

Part 2 is a classical pattern detection problem, to "jump ahead", but not such an easy one.

I went for detecting when the top of the chamber (like top 20 lines) repeat for a specific rock type.

Performance: While I didn't try to optimize for speed, it runs quite fast, 3.2 ms for both parts.

## Update

I realized later that I could have simplified things by listing the coordinates of each rock, trying to move the rocks and check if it works.