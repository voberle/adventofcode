# Day 9: [Marble Mania](https://adventofcode.com/2018/day/9)

## Part 1

This one was mainly about implementing the instructions as precisely as possible, and a bit of debugging on the first test case to get all the details right.

## Part 2

The second part turned out to be nicely slow with my vector-based implementation. I could have waited for it to finish (might have taken around 30 min), but I wanted to see if I can do better.

I didn't find a way to optimize the algorith, so I looked for a better data structure to implement the circle. A linked list comes to mind, but of course those are [famously not popular in Rust](https://rust-unofficial.github.io/too-many-lists/index.html). The Rust standard LinkedList doesn't support inserting in the middle of the list (at least not in stable).

Looking for alternatives, I found the [`dlv_list` crate](https://docs.rs/dlv-list/latest/dlv_list/struct.VecList.html#method.remove), which seems to be an "approved" approach. Converting my code to it wasn't too hard, and it worked: It ran in around 90 ms for part 1 + 2.