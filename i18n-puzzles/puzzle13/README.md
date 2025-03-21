# Puzzle 13: [Gulliver's puzzle dictionary](https://i18n-puzzles.com/puzzle/13/)

The crossword mechanism is the same as in puzzle 6, meaning the parsing code could be fully reused, as well as most of the code to find where the words go.

The encoded words are converted into a `Vec<u8>`. Then I try to decode into each format. Note that decoding may not work for a format.

UTF-8: That's simple with the `String::from_utf8` function, and removing the BOM if it's there.

Latin1: Conversion is easy, as Unicode codepoints are a superset of iso-8859-1 characters. So just each u8 is converted to a char with `c as char`. The trick here is that this may not result in a real word with actual letters. So I'm still check if it is by testing each char with `is_alphabetic()`.

For UTF-16, there are several things to do:

- Check if we have a even number of bytes.
- Check if it starts with a BOM marker and remove it if does.
- Convert the `Vec<u8>` into a `Vec<u16>` with the right endianness.
- Finally try to convert the `Vec<u16>` into a String with `String::from_utf16`.

Then to solve the crossword, I find for each line the one decoded word that fits. Since there is only one solution, it works nicely.
