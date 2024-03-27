# Advent of Code 2019

## Wrapup

This was my 6th AoC, after 2023 and 2015 to 2018.

It was of course the famous Intcode year, with many days using the Intcode interpreter to provide for more fun challenges. This is a very smart trick, since the input is then a program instead of just a static piece of data.

There were quite a few hard ones after the first 10 days. Some of the more interesting stuff:

- My Intcode implementation isn't very sophisticated, but it's simple and clean I feel, and it runs very fast.
- Day 10 felt complicated but got it on first attempt.
- Day 12 was tricky.
- The breakout game in day 13 was super cool.
- Day 14 was complicated as well, as my first attempt with recursion didn't scale. Sleeping over it got me the right approach.
- Day 16 was again complicated, but I was quite proud I ended up seeing enough optimizations to make it work.
- Day 17 was original and fun. The Intcode mechanism enables interesting stuff.
- Day 18 was hard, as my first attempt was just too slow. I had to refactor a good chunk at the end, and it was lots of code.
- Day 21 was the springscript one, difficult but fun, proud to have solved it at the end.
- The card shuffling in day 22 was very hard, and I needed some hints to finish it.
- And day 25 to finish was a very nice way to conclude with Intcode puzzles.

## Days

### Day 1: [The Tyranny of the Rocket Equation](day01/README.md) 🌟🌟

Trivial. successors.

### Day 2: [1202 Program Alarm](day02/README.md) 🌟🌟

Intcode.

### Day 3: [Crossed Wires](day03/README.md) 🌟🌟

Path following, manhattan distance.

### Day 4: [Secure Container](day04/README.md) 🌟🌟

Password validation.

### Day 5: [Sunny with a Chance of Asteroids](day05/README.md) 🌟🌟

Intcode.

### Day 6: [Universal Orbit Map](day06/README.md) 🌟🌟

Graph.

### Day 7: [Amplification Circuit](day07/README.md) 🌟🌟

Intcode.

### Day 8: [Space Image Format](day08/README.md) 🌟🌟

Reading input as bytes. bytecount crate. ASCII picture.

### Day 9: [Sensor Boost](day09/README.md) 🌟🌟

Intcode.

### Day 10: [Monitoring Station](day10/README.md) 🌟🌟

Coordinates in 2D space. Line detection.

### Day 11: [Space Police](day11/README.md) 🌟🌟

Intcode. Moving on map. ASCII picture.

### Day 12: [The N-Body Problem](day12/README.md) 🌟🌟

Positions in 3D. LCM.

### Day 13: [Care Package](day13/README.md) 🌟🌟

Intcode. Game.

### Day 14: [Space Stoichiometry](day14/README.md) 🌟🌟

Reactions.

### Day 15: [Oxygen System](day15/README.md) 🌟🌟

Intcode. Maze. Dijkstra.

### Day 16: [Flawed Frequency Transmission](day16/README.md) 🌟🌟

Iterator repeat, cycle, take.

### Day 17: [Set and Forget](day17/README.md) 🌟🌟

Intcode. ASCII-capable. Path following.

### Day 18: [Many-Worlds Interpretation](day18/README.md) 🌟🌟

Map, shortest path, finding keys and unlocking doors.

### Day 19: [Tractor Beam](day19/README.md) 🌟🌟

Intcode.

### Day 20: [Donut Maze](day20/README.md) 🌟🌟

Maze with teleportation. Dijkstra.

### Day 21: [Springdroid Adventure](day21/README.md) 🌟🌟

Intcode. ASCII-capable. Springscript.

### Day 22: [Slam Shuffle](day22/README.md) 🌟🌟

Card deck. Linear functions.

### Day 23: [Category Six](day23/README.md) 🌟🌟

Intcode.

### Day 24: [Planet of Discord](day24/README.md) 🌟🌟

3D grid/space modification.

### Day 25: [Cryostasis](day25/README.md) 🌟🌟

Intcode. ASCII-capable. Small game.