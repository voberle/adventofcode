# Day 21: [Springdroid Adventure](https://adventofcode.com/2019/day/21)

## Part 1

The Intcode computer running another interpreter, that's twisted! A very cool idea.

Implementation of the interpreter execution wasn't difficult. My code is a bit verbose, as I wanted the springscript program itselt to look pretty. I also like my use of an enum to handle the different types of output.

To find the program itself, I tried different small programs (from the examples and some variants), which made the droid fall at different places and got me to see which patterns I needed to avoid. That way I got the first 3 instructions.

As I was experimenting more and adding a 4th instruction to see if I would get other patterns, I got lucky as one of my random attempts worked.

## Part 2

Of course with part 2 I couldn't rely on luck anymore, and brute-forcing wasn't an option either.

I took the approach of building a logical expression in classical C format when doing the analysis, improving it with each new pattern I found. It wasn't easy, but by [carefully writing each step](analysis.md), I got to the end of it.

Converting that expression to springscript was hard as well, since my logical expression used parenthesis (or AND operator precedence over OR), but springscript doesn't have this.

I wrote a test that checks all possible combinations of registers for both the springscript and the logical expression version, and that was very helpful in checking I that my springscript code did what it was supposed to.
