# Advent of Code 2023

My Rust implementations of [Advent of Code 2023](https://adventofcode.com/2023).

I'm learning Rust in the process, so my goal for the exercises is not always
to do it the most concise or fastest, but also to try out things.

## Exercices overview

### Day 1: [Trebuchet?!](https://adventofcode.com/2023/day/1)

Simple exercices for the first day, nothing especially interesting to note.

### Day 2: [Cube Conundrum](https://adventofcode.com/2023/day/2)

Simple as well, most work was about parsing the input.
As the input is sanitized, the parsing can be made naively.

### Day 3: [Gear Ratios](https://adventofcode.com/2023/day/3)

Getting a bit more complex, used a trait and HashSet and HashMap on this one.
Interesting use of hash structures `entry` + `or_insert_with`.

### Day 4: [Scratchcards](https://adventofcode.com/2023/day/4)

Fairly easy one. Most interesting was the use of `HashSet::intersection()` method to solve it.

### Day 5: [If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

Things got suddenly much harder on day 5. Part 1 took a bit of work but wasn't too bad,
but part 2 didn't scale with the part 1 approach. Unfortunately I didn't find the trick
to do it efficiently and somehow brute forced it by running it manually on various ranges
until I found the value.
First use of RegEx as well.

### Day 6: [Day 6: Wait For It](https://adventofcode.com/2023/day/6)

After a tough day, it got much easier again. Input was so small that I put directly in the code.
Cool use of iterator `zip` method.

### Day 7: [Camel Cards](https://adventofcode.com/2023/day/7)

A cool exercise solved with structures for the cards and hands and custom ordering trait implementations.

### Day 8: [Haunted Wasteland](https://adventofcode.com/2023/day/8)

First part was fairly simple to implement, but part 2 didn't scale at all with the same method.
It was rather disappointing that you had to notice a special pattern in the data to make it work, which I didn't.

Nevertheless I had a lot of fun trying to optimize to the end the brute force method, see `day_part2` sub-folder.
I removed the use of HashMap and used vectors, which sped things up over 10 x.
Further tinkering to help the compiler do the job got things almost 3 x faster.
Processing 100 M items in the loop too 200 ms. As there were 13_133_452 M iterations to do,
it would have taken about 7 hours on my MacBook Air M1.

### Day 9: [Mirage Maintenance](https://adventofcode.com/2023/day/9)

Easy one, only interesting thing to note is use of `windows` iterator method.

### Day 10: [Pipe Maze](https://adventofcode.com/2023/day/10)

The hardest one so far! Part 1 required some work, but didn't have too many challenges.

Part 2 however was really tricky to get working. I used the approach of going along the loop
and counting all area on one side of the loop, but the design of the exercise made this very hard
(especially the fact that the pipes could be next to each other without free space in between).

Getting a solution that worked for the most complex test data was hard, but once I did, it still failed on the real input.
I was stuck until I saw how some people were doing beautiful visualizations of the grid.
I decided to do it also for fun, but it was key: With a better visual of the graph,
I could see where some area was not counted and investigate a specific case, leading me to find the fix. Hurrah!
At the end, the code is even fairly clean considering the complexity of navigating the loop.

### Day 11: [Cosmic Expansion](https://adventofcode.com/2023/day/11)

Another case where the first implementation of part 1 didn't scale at all for part 2 and had to be redone.
In the first attempt, the universe was modelled as a two-dimensional array, but with the universe growing 140 M wide, it had to change.
So I changed the model to only list the positions of the galaxy.
The nice thing is how I approached the refactoring of the expansion function:
I first replaced all iterators in it with index accesses of the 2-dimensions array, validating that it still works,
and that was then easier to convert to list of galaxies model.

Starting to use `BufRead` and traits so that the input parsing code can be used on stdin or on files (for tests).

### Day 12: [Hot Springs](https://adventofcode.com/2023/day/12)

#### Part 1

That was the hardest one so far probably.
The first approach was to find all possible records arrangements and check if they are valid.
But this doesn't work even for the smaller records of part 1, as it breaks down if the record contains a lot of unknowns.
I found a better approach by searching all the possible way of placing the contiguous groups, and checking if they were valid.
As there were much less combinations, and checking validaty was fast, this runs in about 10 seconds for part 1.
Sadly, the approach is completely unusable for part 2.

#### Part 2

For part 2, it turns out that recursion with [Memoization](https://en.wikipedia.org/wiki/Memoization) is the right approach.
I had initially thought at recursion, but didn't do it as it's not usually considered to be
a good approach with Rust, and also because it didn't seem clear how to implement it.
Once I realized that other methods wouldn't scale, I did a deep dive into recursion again. The idea was to find all possible places
for the first group, then recursively for the next one, and so on. What was tricky was to figure out how exactly
to pass the data between the recursive methods. Passing indexes as I initially did turned out too complicated.
So I decided to modify the main state string, by making the positions where I placed the damaged group as unusuable for next groups,
i.e. setting them to dots. This turned out simpler to implement and I got the test cases to pass.
There was however still a bug with the full data. Fortunately I had the implementation of part 1, so I could know what were the correct
arrangement numbers for each record. It allowed to find the ones that were still failing. And after doing a deep dive into one,
I fixed the last bug.

For memoization, there is the [memoize](https://crates.io/crates/memoize) cradle with worked well, just had to remove the references
from the function to memoize. And the whole thing ran in under 2 seconds!

### Day 13: [Point of Incidence](https://adventofcode.com/2023/day/13)

For part 1, I implemented a brute force approach but relatively optimized, and it runs nicely fast.
Used a `Vec<char>` for the two-dimensional array this time. This is great for getting the lines,
but the columns require a copy and it's far less optimal.
Part 2 still worked with the same approach, of trying all possible smudge changes, it remained fast enough.
Main challenge here was understanding the task correctly, that the original reflection could still be valid 
and simply needed to be ignored.

### Day 14: [Parabolic Reflector Dish](https://adventofcode.com/2023/day/14)

For part 1, main tricky thing was getting the right approach on how to collapse a single lines.
I had a few attempts that were too complicated and failed on some cases. Finally I found that the easiest
was to build the new string char by char, as seen in `collapse_down`.
Part 2 was obviously not going to work in a pure brute force way, but I went this direction anyway, as I thought
there might be some periodicity in the answers that would help. So once I had the cycle support (not as part 1 could be easily reused),
I ran the first few thousands and indeed it was periodic. I thought at implementing caching using this, but even with caching,
1 billion iterations is a lot. So instead I used some simple math to jump ahead with the iterations using the period.

One thing to notice is that I had very similar array management code as to day 13.
This code could be made more reusable and put in a library.

### Day 15: [Lens Library](https://adventofcode.com/2023/day/15)

Part 1 was one of the easiest since a few days, with a clean use of iterator `fold`.
Part 2 required just properly following the instructions.

### Day 16: [The Floor Will Be Lava](https://adventofcode.com/2023/day/16)

Part 1 works as a recursive approach where we save which position and direction
have already been taken.