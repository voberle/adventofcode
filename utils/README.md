# Utilities for Advent of Code

The code is usually meant to be copied (and maybe modified) to your daily file.

This allows the utils code to evolve without breaking old AoC days.

There are tests, which don't need to be copied if the code is not modified.

## Useful links

### Tools

[Graphviz](https://graphviz.org/doc/info/lang.html)

### Algorithms

- [Depth First Search](https://en.wikipedia.org/wiki/Depth-first_search)
- [Breadth First Search](https://en.wikipedia.org/wiki/Breadth-first_search)
- [Dijkstra shortest path](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
- [Shoelace algorithm](https://www.101computing.net/the-shoelace-algorithm) for a surface

## Tips

To get the whole standard input to a String:

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

### Graphviz

Generate a PDF:

    dot -Tpdf -Ksfdp input_cluster.gv > input.pdf

`digraph {}` and `->` for directed graphs, `graph {}` and `--` for non-directed ones.

### Performance

- [FxHash](https://github.com/cbreeden/fxhash) for faster HashSet/HashMap.
- [Memoization](https://en.wikipedia.org/wiki/Memoization) for caching recursion calls.