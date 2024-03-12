# Day 13: [Care Package](https://adventofcode.com/2019/day/13)

## Part 1

The description of this day looks so excting, a game!

## Part 2

Part 2 was fun but not that easy. I ended up implementing the working game, but then I still had to *win* at the game. I implemented a save/reload mechanism, that allowed to save progress, and so managed to beat it.

### Update

Later from other solutions I realized it was easy to play the game automatically, as you just need to keep the paddle always under the ball (have matching x coordinates). This actually didn't fully work for me, as it still sometimes missed the ball. I added a tweak to keep the paddle towards the middle when the ball and paddle are aligned, and then it worked.

## Playing the game

Run it with:

    cargo r --release -- play

To use the saved game data:

    cargo r --release -- saved

And to have the full game play on its own:

    cargo r --release -- auto

or using the moves saved in the winning input:

    cargo r --release -- auto_file
