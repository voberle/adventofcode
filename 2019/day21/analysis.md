# Springscript analysis

## Base program for part 1

For part 1, we had following holes to jump:

    #####.###########
    #####.#..########
    #####...#########

which is done with:

    NOT(A, J), // 1-away is empty
    NOT(C, T),
    OR(T, J), // or 3-away is empty
    AND(D, J), // and 4-away is ground

As a simpler boolean expression, in valid Rust, it's:

    (!a || !c) && d

This is verified by the tests.

## Part 2 analysis

Each time we jump, we jump above the next 3 spots.

For each setup, we mark the jump positions that are ok with X.

What we had after part 1:

      XXX
    #####.###########
      X   X
    #####.#..########
        X
    #####...#########

and the expression:

    (!a || !c) && d
---
Then part 2 added several more configurations, for each we improve the expression to support them.

        X   X   X
    #####...##.##.###
       XX
    #####..##########
             ABCDEFGHI

Add `!b && !e`:

    (!a || !c || (!b && !e)) && d

Reordered to as we don't have parenthesis in Springscript:

    ((!b && !e) || !a || !c) && d
---
        X   X   
    #####.#.#..######
      O   X            => Bad jump to exclude
       ABCDEFGHI

Change `!c` to `!c && g`:

    ((!b && !e) || !a || (!c && g)) && d
---
      X   X   X
    #####.##..#.#####
       O   X   X      => Bad jump to exclude
       ABCDEFGHI

We can support this one by changing previous one a bit and replace `g` with `h`:

    ((!b && !e) || !a || (!c && h)) && d

Removing the parenthesis on this one:

    (!b && !e && d) || (!a && d) || (!c && h && d)
---
       XX    X
    #####..###...####

This one passes.

---
       ^   X   X   V
    #####.###..#.####
        ABCDEFGHI

That last one required a dedicated extra condition:

    !b && c && d

## Part 2 verification

So the expression we got:

    (!b && d && !e) || (!a && d) || (!c && d && h) || (!b && c && d)

Verifying it for each pattern:

        ^   V
    #####.###########
         ABCDEFGHI      !a && d
    
      ^   X   V
    #####.#..########
       ABCDEFGHI        !c && d && h
           ABCDEFGHI    !a && d
    
        ^   V
    #####...#########   !a && d
         ABCDEFGHI
    
        ^   X   X   V
    #####...##.##.###
         ABCDEFGHI      !a && d
             ABCDEFGHI  !b && d && !e
                 ABCDEFGHI  !a && d
    
        ^   V
    #####..##########
         ABCDEFGHI      !a && d
    
        ^   X   V
    #####.#.#..######
         ABCDEFGHI      !c && d && h
             ABCDEFGHI  !a && d
    
      ^   X   X   V
    #####.##..#.#####
       ABCDEFGHI        !c && d && h
           ABCDEFGHI    !c && d && h
           ABCDEFGHI    !b && d && !e
               ABCDEFGHI    !a && d
    
        ^   V^   V
    #####..###...####
         ABCDEFGHI      !a && d
              ABCDEFGHI !a && d
    
       ^   X   X   V
    #####.###..#.####
        ABCDEFGHI       !b && c && d
            ABCDEFGHI   !b && d && !e
