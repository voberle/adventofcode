# Puzzle 6: [Mojibake puzzle dictionary](https://i18n-puzzles.com/puzzle/6/)

The [iso-latin-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1) encoding uses 8 bits for each char.

If a word needs to be corrected, I go through each byte. If the byte is less than 127, the value is the same in UTF-8, so we just add it to the correct string.

Otherwise, it's a UTF-8 char represented by more than one byte. The number of leading ones indicates how many parts there are (as explained in this [amazing video](https://www.youtube.com/watch?v=MijmeoH9LT4)).

We collect the bytes making up the char, and convert that byte array to a String with `String::from_utf8()`.

But there is a hack I don't understand...
