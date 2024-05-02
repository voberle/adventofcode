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

fn score(c: char) -> u32 {
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
        .map(score)
        .sum()
}

fn part2(lines: &[Vec<char>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Part 1: {}", total_syntax_error_score(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_illegal() {
        assert_eq!(
            is_illegal(&"{([(<{}[<>[]}>{[]{[(<()>".chars().collect::<Vec<_>>()),
            Some('}')
        );
        assert_eq!(
            is_illegal(&"[[<[([]))<([[{}[[()]]]".chars().collect::<Vec<_>>()),
            Some(')')
        );
        assert_eq!(
            is_illegal(&"[{[{({}]{}}([{[{{{}}([]".chars().collect::<Vec<_>>()),
            Some(']')
        );
        assert_eq!(
            is_illegal(&"[<(<(<(<{}))><([]([]()".chars().collect::<Vec<_>>()),
            Some(')')
        );
        assert_eq!(
            is_illegal(&"<{([([[(<>()){}]>(<<{{".chars().collect::<Vec<_>>()),
            Some('>')
        ); // Expected ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(total_syntax_error_score(&build(INPUT_TEST)), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
