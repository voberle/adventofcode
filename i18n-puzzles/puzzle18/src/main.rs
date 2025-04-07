use std::{
    fmt::Display,
    io::{self, Read},
};

use deunicode::deunicode;

fn build(input: &str) -> Vec<String> {
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

#[allow(dead_code)]
fn remove_bidi_chars(s: &str) -> String {
    deunicode(s)
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(i64),
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Multiply,
    Divide,
    LRI,
    RLI,
    PDI,
}
use Token::{
    CloseParenthesis, Divide, LRI, Minus, Multiply, Number, OpenParenthesis, PDI, Plus, RLI,
};
use itertools::Itertools;

impl Token {
    fn calc(self, val1: i64, val2: i64) -> i64 {
        match self {
            Plus => val1 + val2,
            Minus => val1 - val2,
            Multiply => val1 * val2,
            Divide => val1 / val2,
            _ => panic!("Invalid operator: {self:?}"),
        }
    }

    fn flip(self) -> Token {
        match self {
            Number(n) => {
                if n >= 10 {
                    Number(
                        n.to_string()
                            .chars()
                            .rev()
                            .collect::<String>()
                            .parse()
                            .unwrap(),
                    )
                } else {
                    self
                }
            }
            OpenParenthesis => CloseParenthesis,
            CloseParenthesis => OpenParenthesis,
            _ => self,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "{n}"),
            OpenParenthesis => write!(f, "("),
            CloseParenthesis => write!(f, ")"),
            Plus => write!(f, " + "),
            Minus => write!(f, " - "),
            Multiply => write!(f, " * "),
            Divide => write!(f, " / "),
            LRI => write!(f, "⏵"),
            RLI => write!(f, "⏴"),
            PDI => write!(f, "⏶"),
        }
    }
}

#[derive(Debug, Clone)]
struct Expression(Vec<Token>);

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        let mut tokens = Vec::new();
        let mut current_number = String::new();

        for c in value.chars() {
            if c.is_ascii_digit() {
                current_number.push(c);
            } else {
                if !current_number.is_empty() {
                    tokens.push(Number(current_number.parse().unwrap()));
                    current_number.clear();
                }
                if c == ' ' {
                    continue;
                }
                tokens.push(match c {
                    '(' => OpenParenthesis,
                    ')' => CloseParenthesis,
                    '+' => Plus,
                    '-' => Minus,
                    '*' => Multiply,
                    '/' => Divide,
                    '\u{2066}' => LRI,
                    '\u{2067}' => RLI,
                    '\u{2069}' => PDI,
                    _ => panic!("Invalid char '{c}'"),
                });
            }
        }
        if !current_number.is_empty() {
            tokens.push(Number(current_number.parse().unwrap()));
        }
        Self(tokens)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            write!(f, "{token}")?;
        }
        Ok(())
    }
}

impl Expression {
    // Calculates the results by ignoring the BiDi chars.
    fn calculate(&self) -> i64 {
        // Implementation of the Dijkstra Shunting Yard Algorithm
        // Based on the pseudo-code from https://www.geeksforgeeks.org/expression-evaluation/

        fn precedence(token: Token) -> u8 {
            match token {
                Multiply | Divide => 2,
                Plus | Minus => 1,
                _ => 0,
            }
        }

        fn pop_values_push_result(operator: &Token, values: &mut Vec<i64>) {
            let val2 = values.pop().unwrap();
            let val1 = values.pop().unwrap();
            let result = operator.calc(val1, val2);
            // Push the result onto the value stack.
            values.push(result);
        }

        // Values and operators stack.
        let mut values: Vec<i64> = Vec::new();
        let mut operators: Vec<Token> = Vec::new();

        for token in &self.0 {
            // Go through each token in order.
            match token {
                Number(n) => values.push(*n),
                OpenParenthesis => operators.push(*token),
                CloseParenthesis => {
                    // While the top of the operator stack is not a open parenthesis.
                    while !matches!(operators.last(), Some(OpenParenthesis)) {
                        let operator = operators.pop().unwrap();
                        pop_values_push_result(&operator, &mut values);
                    }
                    // Pop the open parenthesis from the operator stack, and discard it.
                    operators.pop();
                }
                Plus | Minus | Multiply | Divide => {
                    // While the operator stack is not empty, and the top has the same or greater precedence as thisOp,
                    while !operators.is_empty()
                        && precedence(*operators.last().unwrap()) >= precedence(*token)
                    {
                        let operator = operators.pop().unwrap();
                        pop_values_push_result(&operator, &mut values);
                    }
                    operators.push(*token);
                }
                LRI | RLI | PDI => {}
            }
        }

        // While the operator stack is not empty.
        while let Some(operator) = operators.pop() {
            pop_values_push_result(&operator, &mut values);
        }

        // At this point the operator stack should be empty, and the value stack has one value, the final result.
        assert_eq!(values.len(), 1);
        values[0]
    }

    // Determine the embedding level for each token, with the method described in the puzzle.
    fn embedding_levels(&self) -> Vec<usize> {
        let mut level = 0;
        self.0
            .iter()
            .map(|token| {
                match token {
                    Number(_) => {
                        // Numbers will always end up from left to right. So 42 is always 42 and never 24, no matter what the surrounding BiDi markers say.
                        // To account for this, you must increase the embedding level for digits up to the nearest even number.
                        if level % 2 == 1 { level + 1 } else { level }
                    }
                    RLI => {
                        level += 1;
                        assert_eq!(level % 2, 1); // Level is now an odd number.
                        // The new level applies only on next token
                        level - 1
                    }
                    LRI => {
                        level += 1;
                        assert_eq!(level % 2, 0); // Level is now an even number.
                        level - 1
                    }
                    PDI => {
                        level -= 1;
                        level
                    }
                    _ => level,
                }
            })
            .collect()
    }

    // Represents the levels as a string, in the way done in the puzzle.
    #[allow(dead_code)]
    fn embedding_levels_as_str(&self, levels: &[usize]) -> String {
        self.0
            .iter()
            .zip(levels.iter())
            .map(|(token, level)| {
                let token_as_str = token.to_string();
                level.to_string().repeat(token_as_str.chars().count())
            })
            .join("")
    }
}

// Flips a list of tokens.
fn flip(tokens: &[Token]) -> Vec<Token> {
    tokens.iter().rev().map(|token| token.flip()).collect()
    // TODO: Return iterator
}

// Flips the highest level of the expression, updating both the expression and level list.
// Returns false if nothing can be flipped (highest level is 0).
fn flip_highest_level(expr: &mut Expression, levels: &mut Vec<usize>) -> bool {
    let highest_level = *levels.iter().max().unwrap();
    // println!("Highest level: {}", highest_level);
    if highest_level == 0 {
        return false;
    }

    // Find the indexes of the sections that need to be flipped.
    let index_levels: Vec<(usize, &usize)> = levels.iter().enumerate().collect();
    let parts = index_levels
        .split(|(_index, level)| **level < highest_level)
        .filter(|p| !p.is_empty())
        .collect_vec();
    let sections = parts
        .iter()
        .map(|p| (p.first().unwrap().0, p.last().unwrap().0))
        .collect_vec();

    // Flip the sections.
    for (start, end) in sections {
        // println!("Flipping from {start} to {end}");
        if start == end {
            // Stretches of length 1 only matter if it's numbers.
            expr.0[start] = expr.0[start].flip();
            levels[start] -= 1;
        } else {
            // Longer section.
            assert!(start < end);
            expr.0.splice(start..=end, flip(&expr.0[start..=end]));
            levels.splice(
                start..=end,
                std::iter::repeat(highest_level - 1).take(end - start + 1),
            );
        }
    }

    true
}

fn reverse_expression(expr: &Expression) -> Expression {
    let mut expr_reversed = expr.clone();
    let mut levels = expr.embedding_levels();

    while flip_highest_level(&mut expr_reversed, &mut levels) {}
    expr_reversed
}

fn calc_diff(line: &str) -> u64 {
    let expr: Expression = line.into();
    let rex_result = expr.calculate();

    let expr_reversed = reverse_expression(&expr);
    let lynx_result = expr_reversed.calculate();

    let diff = rex_result.abs_diff(lynx_result);
    println!("Lynx: {lynx_result}, Rex: {rex_result}. Absolute difference: {diff}.");
    diff
}

fn scams_sum(lines: &[String]) -> u64 {
    lines.iter().map(|line| calc_diff(line)).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = build(&input);

    println!("Answer: {}", scams_sum(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    fn replace_bidi_chars(s: &str) -> String {
        s.replace('\u{2066}', "⏵")
            .replace('\u{2067}', "⏴")
            .replace('\u{2069}', "⏶")
    }

    #[test]
    fn test_bidi_removal() {
        let input = "\u{2067}(1 * ((\u{2066}(66 / 2)\u{2069} - 15) - 4)) * (1 + (1 + 1))\u{2069}";
        assert_eq!(
            remove_bidi_chars(input),
            "(1 * (((66 / 2) - 15) - 4)) * (1 + (1 + 1))"
        );
    }

    #[test]
    fn test_display() {
        let lines = build(INPUT_TEST_1);
        for line in lines {
            let expr: Expression = line.as_str().into();
            assert_eq!(expr.to_string(), replace_bidi_chars(&line));
        }
    }

    #[test]
    fn test_calculate() {
        let expr: Expression = "(1 * (((66 / 2) - 15) - 4)) * (1 + (1 + 1))".into();
        assert_eq!(expr.calculate(), 42);
    }

    #[test]
    fn test_rex_calculation() {
        fn calc(line: &str) -> i64 {
            let expr: Expression = line.into();
            expr.calculate()
        }

        // Calculate by ignoring BiDi chars.
        let lines = build(INPUT_TEST_1);

        assert_eq!(calc(&lines[0]), 42);
        assert_eq!(calc(&lines[1]), 260);
        assert_eq!(calc(&lines[2]), 15040);
        assert_eq!(calc(&lines[3]), 6300);
        assert_eq!(calc(&lines[4]), 2760);
        assert_eq!(calc(&lines[5]), 316);
    }

    #[test]
    fn test_embedding_levels() {
        let input = "73 + (3 * (1 * \u{2067}(((3 + (6 - 2)) * 6) + \u{2066}((52 * 6) / \u{2067}(13 - (7 - 2))\u{2069})\u{2069})\u{2069}))";
        let expr: Expression = input.into();
        assert_eq!(
            expr.to_string(),
            "73 + (3 * (1 * ⏴(((3 + (6 - 2)) * 6) + ⏵((52 * 6) / ⏴(13 - (7 - 2))⏶)⏶)⏶))"
        );

        let levels = expr.embedding_levels();
        assert_eq!(
            expr.embedding_levels_as_str(&levels),
            "00000000000000001112111121112111112111112222222222222344333343334332211000"
        );
    }

    #[test]
    fn test_flip_number() {
        assert_eq!(Number(4).flip(), Number(4));
        assert_eq!(Number(44257).flip(), Number(75244));
    }

    #[test]
    fn test_flip_sequence() {
        let expr: Expression = "31 - (7 - 2)".into();
        let res: Expression = "(2 - 7) - 13".into();
        assert_eq!(flip(&expr.0), res.0);
    }

    #[test]
    fn test_flip_highest_level() {
        let input = "73 + (3 * (1 * \u{2067}(((3 + (6 - 2)) * 6) + \u{2066}((52 * 6) / \u{2067}(13 - (7 - 2))\u{2069})\u{2069})\u{2069}))";
        let mut expr: Expression = input.into();
        let mut levels = expr.embedding_levels();
        // After 1st flip:
        assert!(flip_highest_level(&mut expr, &mut levels));
        assert_eq!(
            expr.to_string(),
            "73 + (3 * (1 * ⏴(((3 + (6 - 2)) * 6) + ⏵((52 * 6) / ⏴(31 - (7 - 2))⏶)⏶)⏶))"
        );
        // After 2nd flip:
        assert!(flip_highest_level(&mut expr, &mut levels));
        assert_eq!(
            expr.to_string(),
            "73 + (3 * (1 * ⏴(((3 + (6 - 2)) * 6) + ⏵((52 * 6) / ⏴((2 - 7) - 13)⏶)⏶)⏶))"
        );
        // After 3rd flip:
        assert!(flip_highest_level(&mut expr, &mut levels));
        assert_eq!(
            expr.to_string(),
            "73 + (3 * (1 * ⏴(((3 + (6 - 2)) * 6) + ⏵(⏶(31 - (7 - 2))⏴ / (6 * 25))⏶)⏶))"
        );
        // After 4th flip:
        assert!(flip_highest_level(&mut expr, &mut levels));
        assert_eq!(
            expr.to_string(),
            "73 + (3 * (1 * ⏴(⏶((52 * 6) / ⏴((2 - 7) - 13)⏶)⏵ + (6 * ((2 - 6) + 3)))⏶))"
        );
        // Max, can't flip anymore.
        assert!(!flip_highest_level(&mut expr, &mut levels));
    }

    #[test]
    fn test_answer() {
        let lines = build(INPUT_TEST_1);
        assert_eq!(scams_sum(&lines), 19282);
    }
}
