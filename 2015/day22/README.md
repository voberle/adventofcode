# Day 22: [Wizard Simulator 20XX](https://adventofcode.com/2015/day/22)

## Part 1

That was a fairly difficult and long one, as it required first a lot of code to implement the game engine (and test it), and then the actual fighting to find the best answer, I did it with a recursive method, DFS style. But it required very carefully checking all the instructions and making sure they were implemented precisely.

Once I had the recursive function basics working, 3 bugs blocked me:

- If the boss died on the player spells turn, not take the mana that the player was about to spent into turn.
- Make sure to decide which spells can be cast after the player spells turn, as some may have become just usable again.
- Finally, there was the need of the optimization to stop fighting if we had already spent more than the best minimum found.

## Part 2

With part 1 done, that was trivial.