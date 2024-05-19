# Day 22: [Reactor Reboot](https://adventofcode.com/2021/day/22)

## Part 1

While I was almost sure that brute-forcing this with a big 3D vector wasn't going to scale for part 2, I still did it for part 1 as it wasn't obvious how this can be done better for now.

## Part 2

Initially, I didn't want to do cube geometry (compare and transform them), because I thought it would be too complex. So I tried various ways of splitting the space into smaller areas, with recursion (still too slow) or along planes (didn't work).

So I finally decided to still do the cube geometry. I had a look for ideas on Stack Overflow, where got the insight of maintaining a list of non-overlapping cubes. Then the problem becomes for each step of the input, adding/removing the cube from that list. Since the cubes are non overlapping, we can look at each of them and it's simpler.

Implementing it wasn't trivial however.

I needed a compare method. That wasn't too hard.

Then I needed a split method. As I was exhausted, I used the help of ChatGPT-4o: I asked it about the method how to split it (not the code) and also I used it to generate the test cases. It gave me a working method, which I had to adapt still since in our case, the borders of the cube are thick and part of the cube.

Finally I needed the code that creates this list of non-overlapping cubes. The complexity here was that as we go through the list and the steps, both the list and the steps may need to be modified, by adding or removing elements to them. I solved it by using a VecDeque for the steps and poping elements from it, instead of just iterating over it.
