# Day 14: [Parabolic Reflector Dish](https://adventofcode.com/2023/day/14)

For part 1, main tricky thing was getting the right approach on how to collapse a single lines.
I had a few attempts that were too complicated and failed on some cases. Finally I found that the easiest was to build the new string char by char, as seen in `collapse_down`.

Part 2 was obviously not going to work in a pure brute force way, but I went this direction anyway, as I thought there might be some periodicity in the answers that would help. So once I had the cycle support (not as part 1 could be easily reused),
I ran the first few thousands and indeed it was periodic. I thought at implementing caching using this, but even with caching,
1 billion iterations is a lot. So instead I used some simple math to jump ahead with the iterations using the period.

One thing to notice is that I had very similar array management code as to day 13.
This code could be made more reusable and put in a library.
