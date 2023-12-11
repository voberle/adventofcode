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
