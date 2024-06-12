# Day 18: [Boiling Boulders](https://adventofcode.com/2022/day/18)

## Part 1

The main question in this task was to decide how to identify the side of cube. Once that was figured out, adding all the sides to a hash set gave the answer.

## Part 2

I'm quite proud of this one as I got it right almost on first try, with only minor easy bugs to fix.

There were two main insights to get the answer:

First the exterior surface area was equal to the area found in part 1 minus the expose sides of the inside droplets. This means if I could get the list of droplet cubes, I would have the answer.

Second, if I wrap the whole lava area (the input) into a big cube, and I find the area of the part around the lava area, I could get the droplets by doing "total cubes minus exterior cubes minus lava cubes".

And finally the exterior cubes could be found fairly easily by starting at corner and exploring all neighbours.

Despite the brute-force approach, it runs in 3.4 ms.