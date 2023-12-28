# Day 20: [Pulse Propagation](https://adventofcode.com/2023/day/20)

## Part 1

Part 1 was fun to implement, with a clean use of a trait implemented by each module.
I had to be careful with the design to avoid running into problems with mutable configuration being borrowed again, but as I anticipated this early, I got it quickly right.
The initialization of Conjunction was a bit annoying to get right.
For the test inputs, they go back regularly to their initial state, so it's possible to optimize it even further, however that's not the case for the real input.

## Part 2

Part 2 was a very short description, inviting for trying a brute-force version that of course doesn't work.

It required analyzing the graph and finding some pattern in it (cf Day 8).

With Graphviz, removed the prefix in the input, added `digraph {}` and set some different shapes for the module types. Then:

    dot -Tpdf input.gv > input.pdf

So we see for groups of nodes with similar arrangement.

Doing a back-analysis from *rx*:

- *rx* is only connected to the *dt* conjunction module.
- *dt* is connected to *ks*, *pm*, *dl*, *vk*. They are all conjunction modules, but have only one input, so act as simple inverters.
- These 4 are themselves connected to 4 "highly connected" conjunction: *vr*, *pf*, *ts*, *xd*.

It means

- *dt* needs to remember high pulses, so it sends low to *rx*.
- *ks*, *pm*, *dl*, *vk* must therefore have sent high, so have received low.
- So the high connected nodes must all remember high pulses.

Therefore one way is to find how many pulses are needed to get the *ks*, *pm*, *dl*, *vk* in low state, and the find the lowest common denominator for those.