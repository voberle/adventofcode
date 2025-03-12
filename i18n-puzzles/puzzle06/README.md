# Puzzle 6: [Mojibake puzzle dictionary](https://i18n-puzzles.com/puzzle/6/)

The [iso-latin-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1) encoding uses 8 bits for each char.

If a word needs to be corrected, I go through each byte. If the byte is less than 127, the value is the same in UTF-8, so we just add it to the correct string.

Otherwise, it's a UTF-8 char represented by more than one byte. The number of leading ones indicates how many parts there are (as explained in this [amazing video](https://www.youtube.com/watch?v=MijmeoH9LT4)).

To understand how it works, let's look how the string was miscoded.

Let's say we had 'ü' in the original string. In UTF-8 this is 2 bytes `11000011 10111100`.

When the system expecting iso-latin-1 loads it, these 2 bytes become two separate characters, `11000011` and `10111100`. Each represent some non-ASCII char, and they are bigger than 127.

So when storing it in UTF-8, each characters needs again two bytes:

- `11000011` becomes `110xxxxx 10xxxxxx`
- `10111100` becomes `110xxxxx 10xxxxxx`

This explains why we have 4 bytes in the miscoded string for each non-ASCII char.

So to fix the miscoding, we do the inverse process:

Given 4 bytes, we split them into two pairs

We convert each pair into an UTF-8 char. We do this by putting them into an array, convert that byte array to a String with `String::from_utf8()` and take the first char of the String.

This gives us two chars. We convert them to a `u8`, put them into and array and convert it into a String again.
