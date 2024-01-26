# Advent of Code 2017

## Wrapup

Another fun set of Advent of Code puzzles. It's the 4th one I did, after 2023, 2015 and 2016. It didn't feel as hard anymore, as I get more experience with the puzzles and with Rust.

Tricky days were day 3 (the spiral and its math), day 7 was lots of code (towers on platforms), day 11 with the hexa grid, day 21 as I misunderstood the meaning of rotate a square and day 23 where I converted the assembly to C.

In some puzzles, I used nice solutions or tricks. I'm very happy I also got to try Rust threads and channels in day 18.

There were multiple puzzles with CPU instructions to execute, which is always fun. Maybe I will do a general implementation that can execute most of the puzzles, that could be fun.

AoC creator has a nice post where he talks about the [what goes into the design of this year's puzzles](https://www.reddit.com/r/adventofcode/comments/7idn6k/comment/dqy08tk/).

## Days

### Day 1: [Inverse Captcha](day01/README.md) 🌟🌟

With `wrapping_index`.

### Day 2: [Corruption Checksum](day02/README.md) 🌟🌟

Itertools `minmax()` and `permutations()`

### Day 3: [Spiral Memory](day03/README.md) 🌟🌟

Calculating...

### Day 4: [High-Entropy Passphrases](day04/README.md) 🌟🌟

Anagrams. An easy one.

### Day 5: [A Maze of Twisty Trampolines, All Alike](day05/README.md) 🌟🌟

Easy.

### Day 6: [Memory Reallocation](day06/README.md) 🌟🌟

Writing my own iterator `max_by_key` replacement with `fold`.

### Day 7: [Recursive Circus](day07/README.md) 🌟🌟

Optimized data structure.

### Day 8: [I Heard You Like Registers](day08/README.md) 🌟🌟

Instructions, registers.

### Day 9: [Stream Processing](day09/README.md) 🌟🌟

Counting open brackets, ignoring garbage.

### Day 10: [Knot Hash](day10/README.md) 🌟🌟

Circular list reversal.

### Day 11: [Hex Ed](day11/README.md) 🌟🌟

Hexagonal grids, cube coordinates.

### Day 12: [Digital Plumber](day12/README.md) 🌟🌟

Bi-directional graph, connected groups.

### Day 13: [Packet Scanners](day13/README.md) 🌟🌟

Zig-zag in a sequence with a cache.

### Day 14: [Disk Defragmentation](day14/README.md) 🌟🌟

Knot-hash from Day 10. Finding regions in grid.

### Day 15: [Dueling Generators](day15/README.md) 🌟🌟

Brute-force.

### Day 16: [Permutation Promenade](day16/README.md) 🌟🌟

Instructions execution. Periodic.

### Day 17: [Spinlock](day17/README.md) 🌟🌟

Circular buffer, rem_euclid.

### Day 18: [Duet](day18/README.md) 🌟🌟

Instruction. Registers with HashMap. Threads, channels and Arc pointers.

### Day 19: [A Series of Tubes](day19/README.md) 🌟🌟

Grid, following a line.

### Day 20: [Particle Swarm](day20/README.md) 🌟🌟

Moving particules. Brute-force.

### Day 21: [Fractal Art](day21/README.md) 🌟🌟

Dividing and merging squares, rotating and flipping. Not so easy.

### Day 22: [Sporifica Virus](day22/README.md) 🌟🌟

Grid moving.

### Day 23: [Coprocessor Conflagration](day23/README.md) 🌟🌟

Instruction. Reuse of [Day 18](day18/README.md). Converting assembly into C.

### Day 24: [Electromagnetic Moat](day24/README.md) 🌟🌟

Recursive approach for longest path.

### Day 25: [The Halting Problem](day25/README.md)  🌟🌟

Parsing.