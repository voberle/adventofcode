use std::{
    fmt::Display,
    io::{self, Read},
};

use itertools::Itertools;

// A line is the crossword.
#[derive(Debug)]
struct CrosswordLine {
    before: usize,
    letter: char,
    after: usize,
}

impl CrosswordLine {
    fn len(&self) -> usize {
        self.before + self.after + 1
    }

    fn build(line: &str) -> Self {
        fn count_dots(line: &str) -> usize {
            line.chars().take_while(|c| *c == '.' && *c != '\n').count()
        }

        let mut trimmed = line.trim();
        let before = count_dots(trimmed);
        trimmed = &trimmed[before..];
        let letter = trimmed.chars().next().unwrap();
        trimmed = &trimmed[1..];
        let after = count_dots(trimmed);
        Self {
            before,
            letter,
            after,
        }
    }
}

impl Display for CrosswordLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            ".".repeat(self.before),
            self.letter,
            ".".repeat(self.after)
        )
    }
}

#[derive(Debug)]
struct EncodedWord(Vec<u8>);

impl EncodedWord {
    fn build(line: &str) -> Self {
        let h = (0..line.len())
            .step_by(2)
            .map(|i| {
                let s = &line[i..=i + 1];
                u8::from_str_radix(s, 16).unwrap()
            })
            .collect();
        Self(h)
    }

    fn has_bom(&self, bom_marker: &[u8]) -> bool {
        self.0.starts_with(bom_marker)
    }

    fn decode_as_utf_8(&self) -> Option<String> {
        // The UTF-8 representation of the BOM is the (hexadecimal) byte sequence EF BB BF.
        if self.has_bom(&[0xEF, 0xBB, 0xBF]) {
            String::from_utf8(self.0[3..].to_vec()).ok()
        } else {
            String::from_utf8(self.0.clone()).ok()
        }
    }

    fn decode_as_latin1(&self) -> Option<String> {
        // BOM not supported.

        // Unicode codepoints are a superset of iso-8859-1 characters.
        let decoded: String = self.0.iter().map(|&c| c as char).collect();

        if decoded.chars().all(char::is_alphabetic) {
            // Check if all chars are letters.
            Some(decoded)
        } else {
            None
        }
    }

    fn as_u16_le(&self) -> Vec<u16> {
        self.0
            .chunks(2)
            .map(|w| (u16::from(w[1]) << 8) + u16::from(w[0]))
            .collect()
    }

    fn as_u16_be(&self) -> Vec<u16> {
        self.0
            .chunks(2)
            .map(|w| (u16::from(w[0]) << 8) + u16::from(w[1]))
            .collect()
    }

    fn decode_as_utf_16_le(&self) -> Option<String> {
        if self.0.len() % 2 == 1 {
            // We require a even number of bytes.
            return None;
        }

        // In UTF-16, a BOM (U+FEFF) may be placed as the first bytes of a file
        // or character stream to indicate the endianness (byte order) of all the 16-bit code units of the file or stream.
        if self.has_bom(&[0xFF, 0xFE]) {
            String::from_utf16(&self.as_u16_le()[1..]).ok()
        } else {
            String::from_utf16(&self.as_u16_le()).ok()
        }
    }

    fn decode_as_utf_16_be(&self) -> Option<String> {
        if self.0.len() % 2 == 1 {
            return None;
        }

        if self.has_bom(&[0xFE, 0xFF]) {
            String::from_utf16(&self.as_u16_be()[1..]).ok()
        } else {
            String::from_utf16(&self.as_u16_be()).ok()
        }
    }
}

impl Display for EncodedWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|h| format!("{h:X}")).join(" "))
    }
}

fn build(input: &str) -> (Vec<EncodedWord>, Vec<CrosswordLine>) {
    let mut words = Vec::new();

    let mut it = input.lines();
    for line in it.by_ref() {
        if line.is_empty() {
            break;
        }
        words.push(EncodedWord::build(line));
    }

    let mut crossword = Vec::new();
    for line in it {
        crossword.push(CrosswordLine::build(line));
    }

    (words, crossword)
}

// Checks if the decoded word would fit at this crossword line.
// To check if a word matches the crossword line, we just need to check if the total length is correct
// and if the known letter is at the right position.
fn check_if_fits(cw_line: &CrosswordLine, word: &str) -> bool {
    // We need chars count, not bytes len.
    if word.chars().count() != cw_line.len() {
        return false;
    }

    let char_at = word.chars().nth(cw_line.before).unwrap();
    if char_at == cw_line.letter {
        return true;
    }
    false
}

// Prints all words that would fir on each crossword line.
#[allow(dead_code)]
fn solve(words: &[EncodedWord], crossword: &[CrosswordLine]) {
    for cw_line in crossword {
        println!("{cw_line}");
        for encoded_word in words {
            if let Some(w) = encoded_word.decode_as_utf_8() {
                if check_if_fits(cw_line, &w) {
                    println!("utf8: {w}");
                }
            }
            if let Some(w) = encoded_word.decode_as_utf_16_le() {
                if check_if_fits(cw_line, &w) {
                    println!("utf16-le: {w}");
                }
            }
            if let Some(w) = encoded_word.decode_as_utf_16_be() {
                if check_if_fits(cw_line, &w) {
                    println!("utf16-be: {w}");
                }
            }
            if let Some(w) = encoded_word.decode_as_latin1() {
                if check_if_fits(cw_line, &w) {
                    println!("latin1: {w}");
                }
            }
        }
    }
}

// Gets the solved crossword answer.
fn solved_crossword(words: &[EncodedWord], crossword: &[CrosswordLine]) -> usize {
    crossword
        .iter()
        .map(|cw_line| {
            words
                .iter()
                .enumerate()
                .find(|(_, encoded_word)| {
                    if let Some(w) = encoded_word.decode_as_utf_8() {
                        if check_if_fits(cw_line, &w) {
                            return true;
                        }
                    }
                    if let Some(w) = encoded_word.decode_as_utf_16_le() {
                        if check_if_fits(cw_line, &w) {
                            return true;
                        }
                    }
                    if let Some(w) = encoded_word.decode_as_utf_16_be() {
                        if check_if_fits(cw_line, &w) {
                            return true;
                        }
                    }
                    if let Some(w) = encoded_word.decode_as_latin1() {
                        if check_if_fits(cw_line, &w) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap()
                .0
                + 1 // for the answer we start counting at 1.
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (words, crossword) = build(&input);

    // solve(&words, &crossword);

    println!("Answer: {}", solved_crossword(&words, &crossword));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_as_u16() {
        let word = EncodedWord::build(&format!("{:X}", 0b0110000100000000));
        assert_eq!(word.as_u16_be(), [0b0110000100000000]);
        assert_eq!(word.as_u16_le(), [0b0000000001100001]);
    }

    #[test]
    fn test_utf8() {
        let word = EncodedWord::build("616e77c3a4686c65");
        assert_eq!(word.decode_as_utf_8().unwrap(), "anwähle");
    }

    #[test]
    fn test_utf8_with_bom() {
        let word = EncodedWord::build("efbbbf73796b6b696dc3a46bc3b6");
        assert_eq!(word.decode_as_utf_8().unwrap(), "sykkimäkö");
    }

    #[test]
    fn test_latin1() {
        let word = EncodedWord::build("796c74e46de47373e4");
        assert_eq!(word.decode_as_latin1().unwrap(), "yltämässä");

        let word = EncodedWord::build("61757373e46774");
        assert_eq!(word.decode_as_latin1().unwrap(), "aussägt");
    }

    #[test]
    fn test_utf16le() {
        let word = EncodedWord::build("6c00e20063006800e2007400");
        assert_eq!(word.decode_as_utf_16_le().unwrap(), "lâchât");
    }

    #[test]
    fn test_utf16le_with_bom() {
        let word = EncodedWord::build("fffe6700e20063006800e9006500");
        assert_eq!(word.decode_as_utf_16_le().unwrap(), "gâchée");
    }

    #[test]
    fn test_utf16be() {
        let word = EncodedWord::build("0070006f00eb0065006d");
        assert_eq!(word.decode_as_utf_16_be().unwrap(), "poëem");
    }

    #[test]
    fn test_utf16be_with_bom() {
        let word = EncodedWord::build("feff0069007400e4007000e400e4006800e4006e");
        assert_eq!(word.decode_as_utf_16_be().unwrap(), "itäpäähän");

        let word = EncodedWord::build("feff007300fc006e006400650072006e");
        assert_eq!(word.decode_as_utf_16_be().unwrap(), "sündern");
    }

    #[test]
    fn test_answer() {
        let (words, crossword) = build(INPUT_TEST);
        assert_eq!(solved_crossword(&words, &crossword), 47);
    }
}
