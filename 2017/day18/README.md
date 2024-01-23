# Day 18: [Duet](https://adventofcode.com/2017/day/18)

## Part 1

I could reuse a lot of the Instruction related code from some previous exercises. The main change was to make the `Registers` support more generic, now supporting a variable number of registers, and the type is a generic.

## Part 2

Part 2 was a great occasion to try out Rust threads and especially MSPC channels. I know it wasn't necessary, I could have just ran one after the other, but I wanted to play with those since a while.

I used two channels, one for each communication direction. I used the `recv_timeout` receiver function to catch deadlocks, meaning it's over. Is there another way?

Finally, to have the two threads use the same list of instructions without copying them, I use a Arc shared pointer.