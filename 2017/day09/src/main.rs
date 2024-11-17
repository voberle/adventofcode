use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn process(s: &[char]) -> (u32, usize) {
    let mut score = 0;
    // non-canceled characters within the garbage
    let mut garbage_char_count = 0;

    let mut bracket_level = 0;
    let mut in_garbage = false; // there are no "garbage levels"

    let mut i = 0;
    while i < s.len() {
        let c = s[i];

        if c == '!' {
            i += 2; // ok if we jump below the end, while condition will catch it
            continue;
        }

        if in_garbage {
            // Inside garbage: Only looking for closing char
            if c == '>' {
                in_garbage = false;
            } else {
                garbage_char_count += 1;
            }
            i += 1;
            continue;
        }

        if c == '<' {
            in_garbage = true;
        } else if c == '{' {
            bracket_level += 1;
        } else if c == '}' && bracket_level > 0 {
            // closed a group, so update the score
            score += bracket_level;
            bracket_level -= 1;
        }
        i += 1;
    }

    (score, garbage_char_count)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    let (score, garbage_char_count) = process(&input_parsed);
    println!("Part 1: {score}");
    println!("Part 2: {garbage_char_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(process(&build(r"{}")).0, 1);
        assert_eq!(process(&build(r"{{{}}}")).0, 6);
        assert_eq!(process(&build(r"{{},{}}")).0, 5);
        assert_eq!(process(&build(r"{{{},{},{{}}}}")).0, 16);
        assert_eq!(process(&build(r"{<a>,<a>,<a>,<a>}")).0, 1);
        assert_eq!(process(&build(r"{{<ab>},{<ab>},{<ab>},{<ab>}}")).0, 9);
        assert_eq!(process(&build(r"{{<!!>},{<!!>},{<!!>},{<!!>}}")).0, 9);
        assert_eq!(process(&build(r"{{<a!>},{<a!>},{<a!>},{<ab>}}")).0, 3);
    }

    #[test]
    fn test_garbage_char_count() {
        assert_eq!(process(&build(r"<>")).1, 0);
        assert_eq!(process(&build(r"<random characters>")).1, 17);
        assert_eq!(process(&build(r"<<<<>")).1, 3);
        assert_eq!(process(&build(r"<{!>}>")).1, 2);
        assert_eq!(process(&build(r"<!!>")).1, 0);
        assert_eq!(process(&build(r"<!!!>>")).1, 0);
        assert_eq!(process(&build(r#"<{o"i!a,<{i<a>"#)).1, 10);
    }
}
