# Day 9: [Disk Fragmenter](https://adventofcode.com/2024/day/9)

## Part 1

It was a bit of work, but not very hard. I created the block list and moved them (using `Vec::swap`).

## Part 2

For part 2, I decided to track groups instead of individual spaces/files in my blocks. I moved all the part 1 implementation into a separate module and rewrote it for part 2.

The move method turned out a bit ugly and complicated, and because of moving things around in a vector, the whole implementation is rather slow (70 seconds).

I then optimized the reallocation of the vector by using `std::mem::replace` when possible, bringing the time to 45 seconds.

However this approach of tracking groups might be less efficient than the initial method used for part 1, since in the initial method no vector reallocation is needed.

I also later realized that there was a bug in it: When adding a free space at the end, I don't merge it with the free spaces around it, and as I result it's possible that later a bigger free space would not be found. I'm not sure why this didn't hit me with the real input, might simple luck.

## Update

I found the problem why it was running slow. It wasn't at all because the vector was being reallocated, but because after each block move, I restarted the search completely at the end of the file system. That is a big waste, it's enough to restart the search from the original location of the file we just moved.

This brings down the execution time to 67 ms for both parts.