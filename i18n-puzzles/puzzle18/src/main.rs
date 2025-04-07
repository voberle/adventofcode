use std::io::{self, Read};

use deunicode::deunicode;

fn build(input: &str) -> Vec<String> {
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn remove_bidi_chars(s: &str) -> String {
    deunicode(s)
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(u64),
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Multiply,
    Divide,
}
use Token::{CloseParenthesis, Divide, Minus, Multiply, Number, OpenParenthesis, Plus};

impl Token {
    fn calc(self, val1: u64, val2: u64) -> u64 {
        match self {
            Plus => val1 + val2,
            Minus => val1 - val2,
            Multiply => val1 * val2,
            Divide => val1 / val2,
            _ => panic!("Invalid operator: {self:?}"),
        }
    }
}

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
                    tokens.push(Token::Number(current_number.parse().unwrap()));
                    current_number.clear();
                }
                if c == ' ' {
                    continue;
                }
                tokens.push(match c {
                    '(' => Token::OpenParenthesis,
                    ')' => Token::CloseParenthesis,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Multiply,
                    '/' => Token::Divide,
                    _ => panic!("Invalid char"),
                });
            }
        }
        if !current_number.is_empty() {
            tokens.push(Token::Number(current_number.parse().unwrap()));
        }
        Self(tokens)
    }
}

impl Expression {
    fn calculate(&self) -> u64 {
        // Implementation of the Dijkstra Shunting Yard Algorithm
        // Based on the pseudo-code from https://www.geeksforgeeks.org/expression-evaluation/

        fn precedence(token: Token) -> u8 {
            match token {
                Multiply | Divide => 2,
                Plus | Minus => 1,
                _ => 0,
            }
        }

        fn pop_values_push_result(operator: &Token, values: &mut Vec<u64>) {
            let val2 = values.pop().unwrap();
            let val1 = values.pop().unwrap();
            let result = operator.calc(val1, val2);
            // Push the result onto the value stack.
            values.push(result);
        }

        // Values and operators stack.
        let mut values: Vec<u64> = Vec::new();
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
}

fn scams_sum(lines: &[String]) -> u64 {
    0
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

    #[test]
    fn test_bidi_removal() {
        let input = "\u{2067}(1 * ((\u{2066}(66 / 2)\u{2069} - 15) - 4)) * (1 + (1 + 1))\u{2069}";
        assert_eq!(
            remove_bidi_chars(input),
            "(1 * (((66 / 2) - 15) - 4)) * (1 + (1 + 1))"
        );
    }

    #[test]
    fn test_calculate() {
        let expr: Expression = "(1 * (((66 / 2) - 15) - 4)) * (1 + (1 + 1))".into();
        assert_eq!(expr.calculate(), 42);
    }

    #[test]
    fn test_rex_calculation() {
        fn calc(line: &str) -> u64 {
            let s = remove_bidi_chars(line);
            let expr: Expression = s.as_str().into();
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
    fn test_answer() {
        let lines = build(INPUT_TEST_1);
        assert_eq!(scams_sum(&lines), 19282);
    }
}
