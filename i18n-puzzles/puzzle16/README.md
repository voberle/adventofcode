# Puzzle 16: [8-bit unboxing](https://i18n-puzzles.com/puzzle/16/)

First step was converting the input to UTF-8, which I did using the `codepage_437` crate.

Then I normalized and cleaned up the screencap: I replaced all noise characters with space, and used only single line pipes.

For the real input I also removed the decorative frame around it.

To find the number of rotations, I used a combination of code and manual investigation.

The first thing I did was rotate all the line pipes that have one of their side empty, as we are sure they need to be rotated.

Then I used the fact that each pipe type has a maximum number of rotations it would make: 0 for the cross, 1 for the lines and 4 for the others.

So if a pipe had no rotation left, I could make deductions for its neighbours. I implemented several of such deductions, which allowed to connect a lot of the pipes.

What I was left then was a grid mostly connected, but not fully. The remaining disconnected pipes could be rotated manually, as they weren't many (3 for test, 16 for real input).
