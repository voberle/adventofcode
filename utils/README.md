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

To print all output in tests:

    cargo test -- --color always --nocapture

### Graphviz

Generate a PDF:

    dot -Tpdf -Ksfdp input.gv > input.pdf

`digraph {}` and `->` for directed graphs, `graph {}` and `--` for non-directed ones.

### Performance

- [FxHash](https://github.com/cbreeden/fxhash) for faster HashSet/HashMap.
- [Memoization](https://en.wikipedia.org/wiki/Memoization) for caching recursion calls.
- [Rayon](https://docs.rs/rayon/latest/rayon/) for using multiple threads easily with iterators for example.

### Benchmarking

Easy benchmarking with [hyperfine](https://github.com/sharkdp/hyperfine):

    cargo b --release
    hyperfine --warmup 5 'cat resources/input | ../target/release/day01'

More fine benchmarking can be done by wrapping the code with:

    let now = std::time::Instant::now();

    println!("Execution time: {:.2?}", now.elapsed());
