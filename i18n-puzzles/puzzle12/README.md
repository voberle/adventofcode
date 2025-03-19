# Puzzle 12: [Sorting it out](https://i18n-puzzles.com/puzzle/12/)

It wasn't very complicated, but there was quite a bit of code to write. Also avoiding code duplication while keeping it readable was interesting.

- Letter-by-letter filtering was done by filtering out non-alphabetic chars.
- As in a previous puzzle, accents are removed with `deunicode` crate. Here however I had to do it char by char for Swedish with `deunicode_char` to not remove the accents of the 3 accentuated letters we use in sorting.
- The sorting in Swedish is done via custom char compare method, where I simply assign a numerical value to the 29 letters.
- Finally the Dutch infixes removing was easy, although the sorted example in the description moving them at the end made things a big weird at first. I ended up modifying the example for my tests to have the infixes in their original location.

Maybe there was a crate that would have done it all for free, but I didn't bother searching and it where would the fun be then!
