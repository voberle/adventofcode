use std::io::{self, Read};

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
}

fn count_dots(line: &str) -> usize {
    line.chars().take_while(|c| *c == '.' && *c != '\n').count()
}

fn build(input: &str) -> (Vec<String>, Vec<CrosswordLine>) {
    let mut words = Vec::new();

    let mut it = input.lines();
    for word in it.by_ref() {
        if word.is_empty() {
            break;
        }
        words.push(word.to_string());
    }

    let mut crossword = Vec::new();
    for line in it {
        let mut trimmed = line.trim();
        let before = count_dots(trimmed);
        trimmed = &trimmed[before..];
        let letter = trimmed.chars().next().unwrap();
        trimmed = &trimmed[1..];
        let after = count_dots(trimmed);
        crossword.push(CrosswordLine {
            before,
            letter,
            after,
        });
    }

    (words, crossword)
}

fn latin1_to_string(s: &str) -> String {
    let bytes = s.as_bytes().to_vec();

    let mut result = String::new();

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        // println!("  {:#010b}", b);

        if b < 0x80 {
            result.push(b as char);
        } else {
            // Count how many parts there are
            let parts_count = b.leading_ones() as usize;

            // It's always 2, but it could also be 3 or 4 normally?
            // <https://en.wikipedia.org/wiki/UTF-8>
            assert!(parts_count == 2);

            // HACK: I don't get why that works?!?
            i += 2;

            // Collect bytes to convert them to a UTF-8 char
            let mut bytes_for_char = vec![b];
            for _ in 0..parts_count - 1 {
                i += 1;
                bytes_for_char.push(bytes[i]);
            }

            let utf_str = String::from_utf8(bytes_for_char).unwrap();

            // for b in &bytes_for_char {
            //     print!("    {:#010b}, ", b);
            // }
            // println!("- '{}'", utf_str);

            result.push_str(&utf_str);
        }

        i += 1;
    }

    result
}

fn fix_word_encoding(word: &str, index: usize) -> String {
    let mut corrected = word.to_string();
    if index % 3 == 0 {
        corrected = latin1_to_string(&corrected);
    }
    if index % 5 == 0 {
        corrected = latin1_to_string(&corrected);
    }
    // if it divides by 3 and 5 (ie 15), we have double miscoding.
    corrected
}

fn fix_words(words: &[String]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .map(|(i, word)| fix_word_encoding(word, i + 1))
        .collect()
}

fn answer(words: &[String], crossword: &[CrosswordLine]) -> usize {
    let fixed_words = fix_words(words);
    // for (i, w) in fixed_words.iter().enumerate() {
    //     println!("{}: {}", i + 1, w);
    // }
    // println!();

    crossword
        .iter()
        .map(|cw_line| {
            let line_len = cw_line.len();
            fixed_words
                .iter()
                .enumerate()
                .find(|(_, word)| {
                    // To check if a word matches the crossword line, we just need to check if the total length is correct
                    // and if the known letter is at the right position.

                    // We need chars count, not bytes len.
                    if word.chars().count() != line_len {
                        return false;
                    }

                    let char_at = word.chars().nth(cw_line.before).unwrap();
                    if char_at == cw_line.letter {
                        return true;
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

    println!("Answer: {}", answer(&words, &crossword));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    fn str_to_vec(list: &str) -> Vec<String> {
        list.lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn test_latin1_to_string() {
        assert_eq!(latin1_to_string("religiÃ«n"), "religiën");
        assert_eq!(latin1_to_string("kÃ¼rst"), "kürst");
        assert_eq!(latin1_to_string("roekoeÃ«n"), "roekoeën");
        // Double
        assert_eq!(
            latin1_to_string(&latin1_to_string("pugilarÃÂ£o")),
            "pugilarão"
        );
    }

    #[test]
    fn test_fix_word_encoding() {
        assert_eq!(fix_word_encoding("geléet", 1), "geléet");
        assert_eq!(fix_word_encoding("religiÃ«n", 3), "religiën");
        assert_eq!(fix_word_encoding("kÃ¼rst", 5), "kürst");
        assert_eq!(fix_word_encoding("roekoeÃ«n", 6), "roekoeën");
        assert_eq!(fix_word_encoding("pugilarÃÂ£o", 15), "pugilarão");
    }

    #[test]
    fn test_fixed_encoding() {
        let words = str_to_vec(
            r"geléet
träffs
religiÃ«n
tancées
kÃ¼rst
roekoeÃ«n",
        );
        assert_eq!(
            fix_words(&words),
            str_to_vec(
                r"geléet
träffs
religiën
tancées
kürst
roekoeën"
            )
        );
    }

    #[test]
    fn test_answer() {
        let (words, crossword) = build(INPUT_TEST);
        assert_eq!(answer(&words, &crossword), 50);
    }
}
