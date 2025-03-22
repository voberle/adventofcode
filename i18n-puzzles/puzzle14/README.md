# Puzzle 14: [Metrification in Japan](https://i18n-puzzles.com/puzzle/14/)

There were two tricky parts in this puzzle.

Converting the number was of course the hard part.

First I divide the string into a maximum of 3 parts (the myriads), splitting by the ten thousands characters.

Then I handle each part in the same way: I split it into exactly 4 sub-parts, some of them potentially empty. Each sub-part contains 0 or 1 char, which is the digit of the corresponding power of ten character.

The other tricky part was the units. I decided to use Mo as the base units, to be able to deal only with integers.

This actually required to use 128 bits when it came to calculating the square meters.
