# Day 20: [Jurassic Jigsaw](https://adventofcode.com/2020/day/20)

## Part 1

To find which borders connect, I decided to see the borders as 10 bits integers, as this makes comparaison easier and faster.

So I first created all the "border integers" for each tiles: There are 8 of them, 1 for each side and then each side reversed as well.

Then I built a graph, showing to which tile each one can connect.

For part 1, we didn't have to assemble the image fully, as we only needed the corners to get the answer. And the corners are the tiles that can connect to only two other tiles. There happens to be 4 of them, on the example and in the real input. It's likely because there is only one way to assemble the tiles.

NB: I was a bit stuck as to how to find which tiles fit together. I explained what I was doing to my daughter and doing this made me realize I need to solve a puzzle, a jigsaw puzzle in fact (as the title of the day says).

## Part 2

Unfortunately my shortcut for part 1 to find only the corners wasn't enough for part 2, we really need to assemble the image.

Assembling the image wasn't that easy. I started with a corner, and tried all tile orientations until one works, then moved to the next tile and so on.

Then once I had the tiles positions and correct orientations, removing their borders and merging them was a matter of a simpler loop.

I made the square grid code reusable between tiles and the assembled picture to be able to reuse the rotation and flipping code. This then made finding the sea monsters easy.
