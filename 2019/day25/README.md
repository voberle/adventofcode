# Day 25: [Cryostasis](https://adventofcode.com/2019/day/25)

## Part 1

First I implemented the support to play the game.

Then I added a mechanism to load commands from a file, so whenever I had to restart the game, I didn't have to retype everthing.

I navigated the small maze and draw a [map of it](resources/map). At first I wanted my code to draw this maze, but since it's not a nice grid, that didn't work, so I did it by hand.

While navigating I collected all the items that didn't end the game, and reached the Security Checkpoint.

To pass that, you need to carry a precise set of items, out of the 8 collected so far. I tried a bit by hand, but there were too many combinations, so I wrote code that tries all of them, et voilà, got the passcode to finish the last day.