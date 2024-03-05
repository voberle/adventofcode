# Day 8: [Space Image Format](https://adventofcode.com/2019/day/8)

## Part 1

The important part here was to read the input as bytes, not chars, and then use `chunks` iterator method, which made the whole thing very easy.

Also as suggested by Clippy, I used the [bytecount](https://crates.io/crates/bytecount) crate to count bytes.

## Part 2

