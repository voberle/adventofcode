# Day 7: [Recursive Circus](https://adventofcode.com/2017/day/7)

## Part 1

Maybe I went a bit overkill with the data structure I used to store the input. I have 3 vectors with all the info, and programs are vector indexes. There is no need to manipulate the program names to execute the algo.

## Part 2

That wasn't as easy.

I used a recursive approach to find the weights of each tower on each platform. Then starting from the bottom, I find which is the highest platform that is unbalanced, which then gives us the item with the wrong weight.