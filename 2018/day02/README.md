# Day 2: [Inventory Management System](https://adventofcode.com/2018/day/2)

## Part 1

I nicely did the occurences counting without any HashMap or sorting, but by using the fact that we have only letters from the alphabet, so we can use an array of size 26 to count how many occurences there are.

## Part 2

I used a few nice tricks in this one:

- Cell to have have a mutable memory location and avoid borrow checker complaints.
- Iterator zip and take_while
- A simple way to iterate over all pairs we can do with the vector, without using itertools or vector indexes.