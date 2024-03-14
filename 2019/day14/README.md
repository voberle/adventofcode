# Day 14: [Space Stoichiometry](https://adventofcode.com/2019/day/14)

## Part 1

My first attempt used recursion, which worked for the simpler test cases (4 first ones), but the recursion got stuck for test 5 and real input.

Later I realized there was a simpler approach that didn't need to explore many options. Since each chemical can only be produced by one reaction, if a certain chemical is needed, we will for sure need to execute that reaction, so there is no need to wait. If the reaction produces more than we need, we keep aside and use it if we need it for further reactions.

## Part 2

For part 2 I used the fact that part 1 algo is really fast to just implement it with a binary search.
