# Day 21: [Dirac Dice](https://adventofcode.com/2021/day/21)

## Part 1

Somehow the most tricky here was getting the modulo to work correctly.

## Part 2

I was stuck on this one for a while because I had misunderstood the description: For part 2, I had forgotten that the dice is rolled 3 times, and each roll produces 3 values. So 27 options total.

The other important thing to realize is that universes repeat themselves, there will be many cases where an universe that has happened before reappears. The first approach that comes to mind for such problems is recursion with memoization, but as I studied the problem, it felt easier to me to have a hashmap of all possible universes, with the key being the game state and the value how many of those universes we have.

Since the copies of the universe *replace* the universe, I used a pop / add approach. The standard Rust HashMap doesn't make this easy, so I used a `BTreeMap` which has pop methods.

The last hurdle was implementing correct turn support.

Second part runs in 640 ms. There is no "Fx" version of BTreeMap unfortunately.

## Update

Turns out that using a FxHashMap, copying the keys and removing values by keys is much faster than using a BTreeMap: It now runs in 9 ms.