# Puzzle 8: [Unicode passwords](https://i18n-puzzles.com/puzzle/8/)

This is a follow-up to the puzzle 3.

I used external crates here, which made it easy (but I learned less about Unicode...).

The [is_vowel](https://docs.rs/is-vowel/latest/is_vowel/index.html) crate checks if a char is a vowel.

For the consonant check, I use the fact that a consonant is an alphabetic character that is not a vowel.

For the no recurring chars, I make the string lowercase and then convert all non-ASCII characters to ASCII equivalents, which  has the effect or removing the accents, with the [deunicode](https://crates.io/crates/deunicode) crate.
