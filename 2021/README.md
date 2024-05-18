# Advent of Code 2021

## Wrapup

- Day 1 to 6 were nice, with experience they were simple enough but still interesting. I enjoyed making the code as short and nice as possible. I'm very happy with the result for days 5 and 6 in particular.
- OEIS was handy in day 7.
- Day 8 was the first complex one. It was a fun deduction game.
- Day 9 and 11 had similarities as grids where elements change based on their neighbors.
- Day 12 was finding all paths problem. I enjoyed applying correctly DFS there, and then optimizing it.
- Day 13 was one of those where a picture forms a code of ASCII letter. I used an external crate for doing OCR on those letters.
- Day 14 was the problem of optimizing operations on a sequence, and in this case I got it fairly fast.
- Then came a finding the shortest path problem in day 15, with neat abstraction of the cave using a trait.
- Parsing some packet in day 16 and finding a trajectory in day 17 were not too hard.
- Day 18 was complicated, and it's a typical day that was made harder both by using Rust, and by my habit of always converting the input into some data structure instead of just manipulating a string.
- In Day 19 I struggled to understand the description for the orientations. Fortunately trying out all options ended up working.
- Day 20 was original in that the real input had something extra versus the test. Luckily for me, I noticed it quickly.

## Days

### Day 1: [Sonar Sweep](day01/README.md) 🌟🌟

Simple. Itertools tuple_windows().

### Day 2: [Dive!](day02/README.md) 🌟🌟

Moving instructions.

### Day 3: [Binary Diagnostic](day03/README.md) 🌟🌟

Binary numbers.

### Day 4: [Giant Squid](day04/README.md) 🌟🌟

Bingo game. Parsing.

### Day 5: [Hydrothermal Venture](day05/README.md) 🌟🌟

Line intersections.

### Day 6: [Lanternfish](day06/README.md) 🌟🌟

Optimization of exponentially growing problem.

### Day 7: [The Treachery of Whales](day07/README.md) 🌟🌟

OEIS. Triangular numbers.

### Day 8: [Seven Segment Search](day08/README.md) 🌟🌟

Deduction.

### Day 9: [Smoke Basin](day09/README.md) 🌟🌟

Grid.

### Day 10: [Syntax Scoring](day10/README.md) 🌟🌟

Syntax checking.

### Day 11: [Dumbo Octopus](day11/README.md) 🌟🌟

Grid.

### Day 12: [Passage Pathing](day12/README.md) 🌟🌟

Find all paths. DFS.

### Day 13: [Transparent Origami](day13/README.md) 🌟🌟

Paper folding. ASCII code reading, OCR.

### Day 14: [Extended Polymerization](day14/README.md) 🌟🌟

Sequence generation. Optimization.

### Day 15: [Chiton](day15/README.md) 🌟🌟

Shortest path in Grid. Dijkstra. Virtual grid.

### Day 16: [Packet Decoder](day16/README.md) 🌟🌟

Packet decoding, binary.

### Day 17: [Trick Shot](day17/README.md) 🌟🌟

Shooting a probe, trajectory tracking.

### Day 18: [Snailfish](day18/README.md) 🌟🌟

Parsing, recursive data struct.

### Day 19: [Beacon Scanner](day19/README.md) 🌟🌟

3D space.

### Day 20: [Trench Map](day20/README.md) 🌟🌟

Image transformation.

### Day 21: [Dirac Dice](day21/README.md) 🌟

Dice game. Modulos.

### Day 22: [Reactor Reboot](day22/README.md) 🌟

Cuboids, 3D.