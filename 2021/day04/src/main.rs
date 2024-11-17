use std::io::{self, Read};

// A spot on a bingo board.
#[derive(Debug, Clone, Copy)]
enum Spot {
    Unmarked(u32),
    Marked(u32),
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<Spot>>);

impl Board {
    fn new() -> Self {
        Self(Vec::new())
    }

    #[allow(dead_code)]
    fn print(&self) {
        const RED: &str = "\x1b[1m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                let spot = self.0[row][col];
                match spot {
                    Spot::Unmarked(v) => print!("{v}\t"),
                    Spot::Marked(v) => print!("{RED}{v}{RESET}\t"),
                }
            }
            println!();
        }
    }

    fn row_wins(&self, row: usize) -> bool {
        (0..5).all(|col| matches!(self.0[row][col], Spot::Marked(_)))
    }

    fn col_wins(&self, col: usize) -> bool {
        (0..5).all(|row| matches!(self.0[row][col], Spot::Marked(_)))
    }

    // Try to place a number on a board. Returns true if it causes the board to be won.
    fn place_number(&mut self, number: u32) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                if matches!(self.0[row][col], Spot::Unmarked(n) if n == number) {
                    self.0[row][col] = Spot::Marked(number);

                    if self.row_wins(row) || self.col_wins(col) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn unmarked_sum(&self) -> u32 {
        self.0
            .iter()
            .flatten()
            .filter_map(|s| match s {
                Spot::Unmarked(v) => Some(v),
                Spot::Marked(_) => None,
            })
            .sum()
    }
}

fn build(input: &str) -> (Vec<u32>, Vec<Board>) {
    // Parsing is a bit ugly :-(
    let mut it = input.lines();
    let numbers = it
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    it.next();

    let mut boards = Vec::new();
    let mut board: Board = Board::new();
    for line in it {
        if line.is_empty() {
            boards.push(board);
            board = Board::new();
            continue;
        }

        board.0.push(
            line.split_whitespace()
                .map(|n| Spot::Unmarked(n.parse::<u32>().unwrap()))
                .collect(),
        );
    }
    boards.push(board);

    (numbers, boards)
}

// Returns the indexes of the winning boards, if any.
fn place_number(boards: &mut [Board], number: u32) -> Vec<usize> {
    let mut winning_boards = Vec::new();
    for (i, board) in boards.iter_mut().enumerate() {
        if board.place_number(number) {
            // A number can be on multiple boards, make sure we place it on all
            // even if we have a winning one.
            winning_boards.push(i);
        }
    }
    winning_boards
}

fn final_score(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut boards = boards.to_vec();

    for number in numbers {
        let winning_boards = place_number(&mut boards, *number);
        if !winning_boards.is_empty() {
            assert_eq!(winning_boards.len(), 1);
            let idx = *winning_boards.first().unwrap();
            return boards[idx].unmarked_sum() * number;
        }
    }
    panic!("No winner")
}

fn squid_final_score(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut boards = boards.to_vec();

    for number in numbers {
        let winning_boards = place_number(&mut boards, *number);

        if !winning_boards.is_empty() {
            // If it was the last board, squid won.
            if boards.len() == 1 {
                assert_eq!(winning_boards.len(), 1);
                let idx = *winning_boards.first().unwrap();
                // println!("Last number {}", number);
                // boards[idx].print();
                return boards[idx].unmarked_sum() * number;
            }

            // Remove winning boards.
            for idx in winning_boards.iter().rev() {
                boards.remove(*idx);
            }
        }
    }
    panic!("No winner")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (numbers, boards) = build(&input);

    println!("Part 1: {}", final_score(&numbers, &boards));
    println!("Part 2: {}", squid_final_score(&numbers, &boards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (numbers, boards) = build(INPUT_TEST);
        assert_eq!(final_score(&numbers, &boards), 4512);
    }

    #[test]
    fn test_part2() {
        let (numbers, boards) = build(INPUT_TEST);
        assert_eq!(squid_final_score(&numbers, &boards), 1924);
    }
}
