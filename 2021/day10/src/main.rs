use std::io::{self, Read};

fn build(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// If the line is illegal, returns the first illegal character of it.
fn is_illegal(line: &[char]) -> Option<char> {
    let mut open_chars: Vec<char> = Vec::new();
    for c in line {
        match c {
            '(' | '[' | '{' | '<' => open_chars.push(*c),
            ')' => {
                if open_chars.pop().unwrap() != '(' {
                    return Some(*c);
                }
            }
            ']' => {
                if open_chars.pop().unwrap() != '[' {
                    return Some(*c);
                }
            }
            '}' => {
                if open_chars.pop().unwrap() != '{' {
                    return Some(*c);
                }
            }
            '>' => {
                if open_chars.pop().unwrap() != '<' {
                    return Some(*c);
                }
            }
            _ => panic!("Invalid char"),
        }
    }
    None
}

fn score_invalid(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid char"),
    }
}

fn total_syntax_error_score(lines: &[Vec<char>]) -> u32 {
    lines
        .iter()
        .filter_map(|line| is_illegal(line))
        .map(score_invalid)
        .sum()
}

// Assumes lines are not corrupted.
fn complete_line(line: &[char]) -> Vec<char> {
    let mut open_chars: Vec<char> = Vec::new();
    for c in line {
        match c {
            '(' | '[' | '{' | '<' => {
                open_chars.push(*c);
            }
            ')' | ']' | '}' | '>' => {
                open_chars.pop();
            }
            _ => panic!("Invalid char"),
        }
    }

    // Now we have all remaining open chars. We just need to close them in reversed order.
    open_chars
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Invalid char"),
        })
        .collect()
}

fn score_incomplete(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid char"),
    }
}

fn score_for(line: &[char]) -> u64 {
    line.iter().fold(0, |acc, e| acc * 5 + score_incomplete(*e))
}

fn middle_score(lines: &[Vec<char>]) -> u64 {
    let mut scores: Vec<u64> = lines
        .iter()
        .filter(|line| is_illegal(line).is_none())
        .map(|line| complete_line(line))
        .map(|end| score_for(&end))
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Part 1: {}", total_syntax_error_score(&lines));
    println!("Part 2: {}", middle_score(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    fn tovec(s: &str) -> Vec<char> {
        s.chars().collect::<Vec<_>>()
    }

    #[test]
    fn test_is_illegal() {
        assert_eq!(is_illegal(&tovec("{([(<{}[<>[]}>{[]{[(<()>")), Some('}'));
        assert_eq!(is_illegal(&tovec("[[<[([]))<([[{}[[()]]]")), Some(')'));
        assert_eq!(is_illegal(&tovec("[{[{({}]{}}([{[{{{}}([]")), Some(']'));
        assert_eq!(is_illegal(&tovec("[<(<(<(<{}))><([]([]()")), Some(')'));
        assert_eq!(is_illegal(&tovec("<{([([[(<>()){}]>(<<{{")), Some('>'));
    }

    #[test]
    fn test_part1() {
        assert_eq!(total_syntax_error_score(&build(INPUT_TEST)), 26397);
    }

    #[test]
    fn test_complete_line() {
        assert_eq!(
            complete_line(&tovec("[({(<(())[]>[[{[]{<()<>>")),
            tovec("}}]])})]")
        );
        assert_eq!(
            complete_line(&tovec("[(()[<>])]({[<{<<[]>>(")),
            tovec(")}>]})")
        );
        assert_eq!(
            complete_line(&tovec("(((({<>}<{<{<>}{[]{[]{}")),
            tovec("}}>}>))))")
        );
        assert_eq!(
            complete_line(&tovec("{<[[]]>}<{[{[{[]{()[[[]")),
            tovec("]]}}]}]}>")
        );
        assert_eq!(
            complete_line(&tovec("<{([{{}}[<[[[<>{}]]]>[]]")),
            tovec("])}>")
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(middle_score(&build(INPUT_TEST)), 288957);
    }
}
