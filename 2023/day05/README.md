# Day 5: [If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

Things got suddenly much harder on day 5. Part 1 took a bit of work but wasn't too bad, but part 2 didn't scale with the part 1 approach.

Unfortunately I didn't find the trick to do it efficiently and somehow brute forced it by running it manually on various ranges until I found the value.

First use of RegEx as well.

## Update

After doing the older years, I came back to this problem.

I update the code and fixed the part 2. This time, I quickly got the idea of doing a binary search with a recursive function. The main trick is that we can use the following optimization: If the difference between the location ranges and seed ranges is the same, then the location beginning is also the smallest.

With that, both parts run in 3.5 ms.