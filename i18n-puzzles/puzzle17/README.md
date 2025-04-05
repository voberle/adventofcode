# Puzzle 17: [╳ marks the spot](https://i18n-puzzles.com/puzzle/17/)

This one turned out way more tricky to solve than it appeared initially.

There were two main challenges.

- First the map fragments have the same width, but different heights, which makes it tricky to build a partial map.
- Second, while it was relatively easy to check if two fragments fit horizontally, it wasn't possible to say if they fit vertically.

So to solve it, I used the fact that it was possible to identify which fragments were on the left border by looking for the border characters. This didn't give me their order, but fortunately they were not that many permutations: Removing the top and bottom corners, there was only 3 left border fragments and I could try all possible permutations of these left border fragments.

For each permutation, I build the left "column", and then I attempt to build the next column, from top to down. For this, I try to add fragments, converting the resulting map to to Unicode and seeing if it's valid one.

Checking if such a partial map is valid Unicode is done with the `String::from_utf8_lossy` function, which converts a Vec of u8 to a String, using the replacement character for invalid one. I ignore all the replacement characters at the end of the lines (since it's only a partial map) and check if there are any in the middle of the map. If not, the partial map is valid so far.

The loop that builds the columns is a bit too complex I feel.
