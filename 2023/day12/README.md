# Day 12: [Hot Springs](https://adventofcode.com/2023/day/12)

## Part 1

That was the hardest one so far probably.

The first approach was to find all possible records arrangements and check if they are valid.
But this doesn't work even for the smaller records of part 1, as it breaks down if the record contains a lot of unknowns.

I found a better approach by searching all the possible way of placing the contiguous groups, and checking if they were valid.
As there were much less combinations, and checking validaty was fast, this runs in about 10 seconds for part 1.

Sadly, the approach is completely unusable for part 2.

## Part 2

For part 2, it turns out that recursion with [Memoization](https://en.wikipedia.org/wiki/Memoization) is the right approach.

I had initially thought at recursion, but didn't do it as it's not usually considered to be a good approach with Rust, and also because it didn't seem clear how to implement it.

Once I realized that other methods wouldn't scale, I did a deep dive into recursion again. The idea was to find all possible places for the first group, then recursively for the next one, and so on. What was tricky was to figure out how exactly to pass the data between the recursive methods. Passing indexes as I initially did turned out too complicated.

So I decided to modify the main state string, by making the positions where I placed the damaged group as unusuable for next groups, i.e. setting them to dots. This turned out simpler to implement and I got the test cases to pass.

There was however still a bug with the full data. Fortunately I had the implementation of part 1, so I could know what were the correct arrangement numbers for each record. It allowed to find the ones that were still failing. And after doing a deep dive into one, I fixed the last bug.

For memoization, there is the [memoize](https://crates.io/crates/memoize) cradle with worked well, just had to remove the references from the function to memoize. And the whole thing ran in under 2 seconds!