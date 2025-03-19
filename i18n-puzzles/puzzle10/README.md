# Puzzle 10: [Unicode passwords strike back!](https://i18n-puzzles.com/puzzle/10/)

The approach here is to create all possible versions of the password, and see if any matches the bcrypt hash.

So for this, I first decompose the password: Each unicode character that can be decomposed into two is decomposed. I'm using the `unicode_normalization` crate `decompose_canonical()` function for this.

Then I detect all possible pairs in that decomposed strings.

Finally, I try all possible combinations of composed/decomposed chars and see if any matches. For each password, there is a maximum of 2 to the power of the number of pairs.
