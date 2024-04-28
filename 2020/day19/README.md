# Day 19: [Monster Messages](https://adventofcode.com/2020/day/19)

## Part 1

This one was harder.

At first I tried a solution that checked each message if the rules match, but it didn't work for me.

So instead I used the approach of building all possible strings that the rule can generate. This wasn't easy to do, but I managed.

## Part 2

With the loops introduced by part 2, the part 1 solution doesn't work.

The description hints that the input needs to be analyzed, and that most rules are unaffected by the loops.

With the part 1 code, I expanded all rules as a far as possible. This gave me all the possible cases that the two rules used by the looping rules, 42 and 31, were formed of.

So considering that rule 0 used only rules 8 and 11 (the looping rules), and rules 8 and 11 used only 42 and 31, we can concluded that all matching messages must be made of parts from rules 42 and 31.

Next looking closely at the looping rules:

    0: 8 11
    8: 42 | 42 8
    11: 42 31 | 42 11 31

Rule 8 is made of 1 or more 42:

    42
    42 42
    42 42 42
    42 42 42 42
    42 42 42 42 42

Rule 11 is made of one or more 42, followed by the same number of 31:

    42 31
    42 42 31 31
    42 42 42 31 31 31
    42 42 42 42 31 31 31 31
    42 42 42 42 42 31 31 31 31 31

So we conclude the message must start with a sequence from 42 and finish with a sequence of 31. It also must have at least 2 42 at the beginning and 1 at the end. Finally, there must be more 42 than 31.

Checking for all these rules gave me the correct number of matching messages.

## Update

Turns out using regular expressions was probably the simplest approach, although many people also managed to do it with recursion, the first approach I had tried.
