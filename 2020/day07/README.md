# Day 7: [Handy Haversacks](https://adventofcode.com/2020/day/7)

## Part 1

For part 1, I created a data structure where each color is represent by an index. Then I loop on the set of rules, marking all bags that can contain gold, until we cannot mark anything anymore. This is a method I've used in some previous days on another year.

## Part 2

I used a similar approach for part 2, looping on the list of rules until there isn't anything to do anymore. That worked well, with fairly simple code and it is fast.
