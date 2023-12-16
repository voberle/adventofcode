# Day 10: [Pipe Maze](https://adventofcode.com/2023/day/10)

The hardest one so far! Part 1 required some work, but didn't have too many challenges.

Part 2 however was really tricky to get working. I used the approach of going along the loop and counting all area on one side of the loop, but the design of the exercise made this very hard (especially the fact that the pipes could be next to each other without free space in between).

Getting a solution that worked for the most complex test data was hard, but once I did, it still failed on the real input.

I was stuck until I saw how some people were doing beautiful visualizations of the grid.
I decided to do it also for fun, but it was key: With a better visual of the graph, I could see where some area was not counted and investigate a specific case, leading me to find the fix. Hurrah!

At the end, the code is even fairly clean considering the complexity of navigating the loop.