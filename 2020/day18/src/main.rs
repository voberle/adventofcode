use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(u64),
    Addition,
    Multiplication,
    OpenParenthesis,
    CloseParenthesis,
}
use Token::{Addition, CloseParenthesis, Multiplication, Number, OpenParenthesis};

impl Token {
    fn new(c: char) -> Self {
        match c {
            '+' => Addition,
            '*' => Multiplication,
            '(' => OpenParenthesis,
            ')' => CloseParenthesis,
            '0'..='9' => Number(u64::from(c.to_digit(10).unwrap())),
            _ => panic!("Invalid token"),
        }
    }

    // Precedence of operators.
    fn precedence_1(&self) -> u8 {
        match self {
            Addition | Multiplication => 1,
            _ => 0,
        }
    }

    fn precedence_2(&self) -> u8 {
        match self {
            Addition => 2,
            Multiplication => 1,
            _ => 0,
        }
    }
}

type PrecedenceFn = fn(&Token) -> u8;

struct Expression(Vec<Token>);

impl Expression {
    fn build(input: &str) -> Self {
        // Luckily for us, the input as only single digit numbers, makes the parsing easier.
        Self(
            input
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(Token::new)
                .collect(),
        )
    }

    fn calc(&self, precedence_fn: PrecedenceFn) -> u64 {
        // Implementation of the Dijkstra Shunting Yard Algorithm
        // Based on the pseudo-code from https://www.geeksforgeeks.org/expression-evaluation/

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
                        let val2 = values.pop().unwrap();
                        let val1 = values.pop().unwrap();
                        let result = match operator {
                            Addition => val1 + val2,
                            Multiplication => val1 * val2,
                            _ => panic!("Invalid operator: {operator:?}"),
                        };
                        // Push the result onto the value stack.
                        values.push(result);
                    }
                    // Pop the open parenthesis from the operator stack, and discard it.
                    operators.pop();
                }
                Addition | Multiplication => {
                    // While the operator stack is not empty, and the top has the same or greater precedence as thisOp,
                    while !operators.is_empty()
                        && precedence_fn(operators.last().unwrap()) >= precedence_fn(token)
                    {
                        let operator = operators.pop().unwrap();
                        let val2 = values.pop().unwrap();
                        let val1 = values.pop().unwrap();
                        let result = match operator {
                            Addition => val1 + val2,
                            Multiplication => val1 * val2,
                            _ => panic!("Invalid operator: {operator:?}"),
                        };
                        values.push(result);
                    }
                    operators.push(*token);
                }
            }
        }

        // While the operator stack is not empty.
        while let Some(operator) = operators.pop() {
            let val2 = values.pop().unwrap();
            let val1 = values.pop().unwrap();
            let result = match operator {
                Addition => val1 + val2,
                Multiplication => val1 * val2,
                _ => panic!("Invalid operator: {operator:?}"),
            };
            values.push(result);
        }

        // At this point the operator stack should be empty, and the value stack has one value, the final result.
        assert_eq!(values.len(), 1);
        values[0]
    }
}

fn build(input: &str) -> Vec<Expression> {
    input.lines().map(Expression::build).collect()
}

fn sum_expressions(expressions: &[Expression], precedence_fn: PrecedenceFn) -> u64 {
    expressions
        .iter()
        .map(|expr| expr.calc(precedence_fn))
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let expressions = build(input.trim());

    println!(
        "Part 1: {}",
        sum_expressions(&expressions, Token::precedence_1)
    );
    println!(
        "Part 2: {}",
        sum_expressions(&expressions, Token::precedence_2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPR_1: &str = "1 + 2 * 3 + 4 * 5 + 6";
    const EXPR_2: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    const EXPR_3: &str = "2 * 3 + (4 * 5)";
    const EXPR_4: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    const EXPR_5: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    const EXPR_6: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_calc_1() {
        assert_eq!(Expression::build(EXPR_1).calc(Token::precedence_1), 71);
        assert_eq!(Expression::build(EXPR_2).calc(Token::precedence_1), 51);
        assert_eq!(Expression::build(EXPR_3).calc(Token::precedence_1), 26);
        assert_eq!(Expression::build(EXPR_4).calc(Token::precedence_1), 437);
        assert_eq!(Expression::build(EXPR_5).calc(Token::precedence_1), 12240);
        assert_eq!(Expression::build(EXPR_6).calc(Token::precedence_1), 13632);
    }

    #[test]
    fn test_calc_2() {
        assert_eq!(Expression::build(EXPR_1).calc(Token::precedence_2), 231);
        assert_eq!(Expression::build(EXPR_2).calc(Token::precedence_2), 51);
        assert_eq!(Expression::build(EXPR_3).calc(Token::precedence_2), 46);
        assert_eq!(Expression::build(EXPR_4).calc(Token::precedence_2), 1445);
        assert_eq!(Expression::build(EXPR_5).calc(Token::precedence_2), 669060);
        assert_eq!(Expression::build(EXPR_6).calc(Token::precedence_2), 23340);
    }
}
