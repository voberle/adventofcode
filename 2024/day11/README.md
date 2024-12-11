# Day 11: [Plutonian Pebbles](https://adventofcode.com/2024/day/11)

## Part 1

Implemented part 1 in a brute-force way, knowing perfectly that in AoC spirit, it's unlikely to work for part 2.

I'm not sure that the way I split numbers is the best one.

Switching from a flat_map to a simple for loop in the blink method divided the run time by 2 (16 to 8 ms), since we removed the allocation of many small vectors.

## Part 2

I made two important observations for solving part 2:

First the order of the numbers doesn't matter for the final answer, as we care only about the number of stones, not the actual stone list.

Second, all single digits number become again a list of single digits after few iterations. All numbers except 8 take 3 to 4 iterations, and 8 generates 16192 which itself regenerates what 8 did. In a nutshell, it's likely there are only a limited amount of different numbers that are ever generated.

This means we can just store the stones in a hashmap "stone" => "stone count". This allows us to do the transformation operation only once for each type of stone at each blink.

With such changes, both parts ran in 9 ms.