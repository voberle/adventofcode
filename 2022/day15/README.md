# Day 15: [Beacon Exclusion Zone](https://adventofcode.com/2022/day/15)

## Part 1

For part 1, I took the naive approach of brute-forcing it.

## Part 2

The main idea is that it's easy to check if a square is in range of the sensors. Since the sensor range is rather big, it's possible to efficiently exclude a lot of positions for the search by checking big squares.

So I check if a square the size of the search space fits fully in one of the cicles. If it doesn't, I divide the square in 4 and check each. If a square fits fully, we don't need to check it further.

Part 2 takes only 6 ms with this approach.

Part 1 is a bit slow however.