# Puzzle 10: [Unicode passwords strike back!](https://i18n-puzzles.com/puzzle/10/)

The approach here is to create all possible versions of the password, and see if any matches the bcrypt hash.

So for this, I first decompose the password: Each unicode character that can be decomposed into two is decomposed. I'm using the `unicode_normalization` crate `decompose_canonical()` function for this.

Then I detect all possible pairs in that decomposed strings.

Finally, I try all possible combinations of composed/decomposed chars and see if any matches. For each password, there is a maximum of 2 to the power of the number of pairs.

As an optimization, we can use the fact that a lot of the login attempts are for the same users, so a lot of the password we try come up again. Therefore we can cache the bcrypt result and speed things up a lot: 5 seconds on my MacBook Air M1.

Then after seeing the Reddit thread, there is another way to do the caching: Instead of caching the bcrypt result, cache the result of the username + decomposed string. So we can compare the decomposed string to the cached version even before trying all combinations. But this doesn't help in term of performance.

Adding Rayon helps however, bringing it down to 1.2 seconds.
