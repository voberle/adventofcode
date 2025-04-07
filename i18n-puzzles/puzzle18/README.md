# Puzzle 18: [Rex To Lynx](https://i18n-puzzles.com/puzzle/18/)

This is one of those puzzles where you just have to do it step by step, testing each time.

First I addressed evaluating the expression, without any Bidi characters. For this, I reused some code from previous AoC days, which implements the Dijkstra Shunting Yard Algorithm:

- The expression evaluator from [AoC year 2020 day 18](../../2020/day18/README.md). The parsing was not usable however as it supported only single digit numbers.
- But in [AoC year 2022 day 13](../../2022/day13/README.md) I had the parsing.

Then I added parsing the BiDi chars. I added `Display` support, so I could get a String from the parsed expression and check at each level that things were correct.

For determining the embedded levels, it was fairly easy as the puzzle offered a good test case. Again printing the levels as a string in the same format as in the puzzle was important.

And finally I implemented flipping.

Flipping one level is done by first finding the start and end indexes of each section (with the `Vec::split()` function).

Then flipping each section was surprisingly easy: A simple iterator `rev()` and flipping each token (meaning just numbers and parenthesis). `Vec::splice()` made it easy to replace sections in the vector.

The two final bugs I had was that I stored numbers as integer, meaning when flipping 130, I would get 31 and loose a 0. So I switching to storing them as String.

The second one was that divisions required floating point numbers, so switched to `f64`.
