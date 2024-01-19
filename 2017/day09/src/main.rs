use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn score(s: &[char]) -> u32 {
    let mut score = 0;

    let mut bracket_level = 0;
    let mut in_garbage = false; // there are no "garbage levels"

    let mut i = 0;
    while i < s.len() {
        let c = s[i];
        if c == '!' {
            // println!("{i}->{c}: !, ignore next");
            i += 2; // ok if we jump below the end, while condition will catch it
            continue;
        }
        if in_garbage {
            // println!("{i}->{c}: in_garbage");
            // Inside garbage: Only looking for closing char
            if c == '>' {
                // println!("{i}->{c}: Stop in_garbage");
                in_garbage = false;
            }
            i += 1;
            continue;
        }
        if c == '<' {
            // println!("{i}->{c}: Start in_garbage");
            in_garbage = true;
        } else if c == '{' {
            bracket_level += 1;
            // println!("{i}->{c}: bracket_level++, {}", bracket_level);
        } else if c == '}' && bracket_level > 0 {
            // closed a group, so update the score
            score += bracket_level;
            bracket_level -= 1;
            // println!("{i}->{c}: bracket_level--, {}; score={score}", bracket_level);
        }

        i += 1;
    }
    score
}

fn part2(input: &[char]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input_parsed = build(&input);

    println!("Part 1: {}", score(&input_parsed));
    println!("Part 2: {}", part2(&input_parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score(&build(r#"{}"#)), 1);
        assert_eq!(score(&build(r#"{{{}}}"#)), 6);
        assert_eq!(score(&build(r#"{{},{}}"#)), 5);
        assert_eq!(score(&build(r#"{{{},{},{{}}}}"#)), 16);
        assert_eq!(score(&build(r#"{<a>,<a>,<a>,<a>}"#)), 1);
        assert_eq!(score(&build(r#"{{<ab>},{<ab>},{<ab>},{<ab>}}"#)), 9);
        assert_eq!(score(&build(r#"{{<!!>},{<!!>},{<!!>},{<!!>}}"#)), 9);
        assert_eq!(score(&build(r#"{{<a!>},{<a!>},{<a!>},{<ab>}}"#)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
