# Day 20: [A Regular Map](https://adventofcode.com/2018/day/20)

## Part 1

### Regex parsing into a graph

The most difficult in this puzzle was to parse the regex. It took me a while to figure out how to handle the empty option in particular, and decide how I wanted to represent it.

I ended up making the problem a bit easier by first producing an intermediate data structure. That made the graph creation a bit easier.

The great Regex visualizatier [Regexper](https://regexper.com) was extremely useful to see how the graph was suppose to be for the test input, like for [test input 4](https://regexper.com/#%5EESSWWN%28E%7CNNENN%28EESS%28WNSE%7C%29SSS%7CWWWSSSSE%28SW%7CNNNE%29%29%29%24).

The important insight I had was that I didn't need to bother connecting the last nodes to the "end" node, as it was implicit. That simplified the generation problem.

To verify that the graph produced was corrected, I generate a Graphviz version of it and compared it against the Regexper version.

### Part 1 answer

To my surprise, I didn't need to generate the maze to find the answer to part 1. Finding the shortest path to all the end nodes and taking the biggest one gave the correct answer. This is probably because no path overlap each other?

## Part 2

For part 2, I tried to also use only the graph, by walking on all possible paths and marking all elements that were less than 1000 steps away. Unfortunately that didn't produce the right answer.