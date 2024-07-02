# Day 8: [Haunted Wasteland](https://adventofcode.com/2023/day/8)

First part was fairly simple to implement, but part 2 didn't scale at all with the same method.
It was rather disappointing that you had to notice a special pattern in the data to make it work, which I didn't.

Nevertheless I had a lot of fun trying to optimize to the end the brute force method, see `day_part2` sub-folder.

I removed the use of HashMap and used vectors, which sped things up over 10 x.

Further tinkering to help the compiler do the job got things almost 3 x faster.

Processing 100 M items in the loop too 200 ms. As there were 13_133_452 M iterations to do, it would have taken about 7 hours on my MacBook Air M1.

## Update

I updated the code to match the format of the other years.
