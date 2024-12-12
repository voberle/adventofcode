# Day 12: [Garden Groups](https://adventofcode.com/2024/day/12)

## Part 1

The approach I took:

I explore the grid value by value and for each position find the corresponding region.

To find a region, I explore the map recursively and collects all positions of a region in a hash set.

Then to find area and perimeter:

Area super easy, just the number of positions.

For the perimeter, go through each position and see if their neighbors are in the region. I just needed a way to identify borders, and I used pairs position - direction. The trick is to be careful that "pos + up" is the same as "pos above + down".
I add all the borders to a set. At the end the perimeter is the size of the set.

## Part 2

