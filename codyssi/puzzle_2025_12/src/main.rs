use std::{
    collections::VecDeque,
    io::{self, Read},
};

#[derive(Debug, Clone)]
struct Grid {
    values: Vec<u64>,
    rows: usize,
    cols: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = value
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }
}

impl Grid {
    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                print!("{c} ");
            }
            println!();
        }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn shift_row(&mut self, row: usize, shift_amount: usize) {
        self.values[row * self.cols..(row + 1) * self.cols]
            .rotate_right(shift_amount.rem_euclid(self.cols));
    }

    fn shift_col(&mut self, col: usize, shift_amount: usize) {
        let values: Vec<_> = self
            .values
            .iter()
            .skip(col)
            .step_by(self.cols)
            .copied()
            .collect();
        for (row, val) in values.iter().enumerate().take(self.rows) {
            let shift_to_pos = self.pos((row + shift_amount).rem_euclid(self.rows), col);
            self.values[shift_to_pos] = *val;
        }
    }

    fn modify_all(&mut self, amount: u64, action_fn: fn(u64, u64) -> u64) {
        self.values
            .iter_mut()
            .for_each(|val| *val = action_fn(*val, amount));
    }

    fn modify_row(&mut self, row: usize, amount: u64, action_fn: fn(u64, u64) -> u64) {
        self.values[row * self.cols..(row + 1) * self.cols]
            .iter_mut()
            .for_each(|val| *val = action_fn(*val, amount));
    }

    fn modify_col(&mut self, col: usize, amount: u64, action_fn: fn(u64, u64) -> u64) {
        self.values
            .iter_mut()
            .skip(col)
            .step_by(self.cols)
            .for_each(|val| *val = action_fn(*val, amount));
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ShiftRow(usize, usize),
    ShiftCol(usize, usize),
    AddAll(u64),
    AddRow(usize, u64),
    AddCol(usize, u64),
    SubAll(u64),
    SubRow(usize, u64),
    SubCol(usize, u64),
    MultiplyAll(u64),
    MultiplyRow(usize, u64),
    MultiplyCol(usize, u64),
}

impl From<&str> for Instruction {
    #[allow(clippy::match_on_vec_items)]
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split_ascii_whitespace().collect();
        match parts[0] {
            "SHIFT" => match parts[1] {
                "ROW" => Instruction::ShiftRow(
                    parts[2].parse::<usize>().unwrap() - 1,
                    parts[4].parse().unwrap(),
                ),
                "COL" => Instruction::ShiftCol(
                    parts[2].parse::<usize>().unwrap() - 1,
                    parts[4].parse().unwrap(),
                ),
                _ => panic!("Unknown Shift instruction"),
            },
            "ADD" => match parts[2] {
                "ALL" => Instruction::AddAll(parts[1].parse().unwrap()),
                "ROW" => Instruction::AddRow(
                    parts[3].parse::<usize>().unwrap() - 1,
                    parts[1].parse().unwrap(),
                ),
                "COL" => Instruction::AddCol(
                    parts[3].parse::<usize>().unwrap() - 1,
                    parts[1].parse().unwrap(),
                ),
                _ => panic!("Unknown Add instruction"),
            },
            "SUB" => match parts[2] {
                "ALL" => Instruction::SubAll(parts[1].parse().unwrap()),
                "ROW" => Instruction::SubRow(
                    parts[3].parse::<usize>().unwrap() - 1,
                    parts[1].parse().unwrap(),
                ),
                "COL" => Instruction::SubCol(
                    parts[3].parse::<usize>().unwrap() - 1,
                    parts[1].parse().unwrap(),
                ),
                _ => panic!("Unknown Sub instruction"),
            },
            "MULTIPLY" => match parts[2] {
                "ALL" => Instruction::MultiplyAll(parts[1].parse().unwrap()),
                "ROW" => Instruction::MultiplyRow(
                    parts[3].parse::<usize>().unwrap() - 1,
                    parts[1].parse().unwrap(),
                ),
                "COL" => Instruction::MultiplyCol(
                    parts[3].parse::<usize>().unwrap() - 1,
                    parts[1].parse().unwrap(),
                ),
                _ => panic!("Unknown Multiply instruction"),
            },
            _ => panic!("Unknown instruction"),
        }
    }
}

const VALUES_COUNT: u64 = 1_073_741_824;

impl Instruction {
    fn apply(&self, grid: &mut Grid) {
        fn add(n: u64, amount: u64) -> u64 {
            (n + amount).rem_euclid(VALUES_COUNT)
        }
        fn sub(n: u64, amount: u64) -> u64 {
            (n + VALUES_COUNT - amount).rem_euclid(VALUES_COUNT)
        }
        fn multiply(n: u64, amount: u64) -> u64 {
            (n * amount).rem_euclid(VALUES_COUNT)
        }

        match self {
            Instruction::ShiftRow(number, shift_amount) => grid.shift_row(*number, *shift_amount),
            Instruction::ShiftCol(number, shift_amount) => grid.shift_col(*number, *shift_amount),
            Instruction::AddAll(amount) => grid.modify_all(*amount, add),
            Instruction::AddRow(number, amount) => grid.modify_row(*number, *amount, add),
            Instruction::AddCol(number, amount) => grid.modify_col(*number, *amount, add),
            Instruction::SubAll(amount) => grid.modify_all(*amount, sub),
            Instruction::SubRow(number, amount) => grid.modify_row(*number, *amount, sub),
            Instruction::SubCol(number, amount) => grid.modify_col(*number, *amount, sub),
            Instruction::MultiplyAll(amount) => grid.modify_all(*amount, multiply),
            Instruction::MultiplyRow(number, amount) => grid.modify_row(*number, *amount, multiply),
            Instruction::MultiplyCol(number, amount) => grid.modify_col(*number, *amount, multiply),
        }
    }
}

#[derive(Debug)]
enum Action {
    Take,
    Cycle,
    Act,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "TAKE" => Action::Take,
            "CYCLE" => Action::Cycle,
            "ACT" => Action::Act,
            _ => panic!("Unknown action"),
        }
    }
}

fn build(input: &str) -> (Grid, Vec<Instruction>, Vec<Action>) {
    let parts: Vec<_> = input.split("\n\n").collect();

    let grid = parts[0].into();
    let instructions = parts[1].lines().map(Into::into).collect();
    let actions = parts[2].lines().map(Into::into).collect();

    (grid, instructions, actions)
}

fn largest_sum(grid: &Grid) -> u64 {
    let rows_sum_max: u64 = (0..grid.rows)
        .map(|row| grid.values[row..row + grid.cols].iter().sum())
        .max()
        .unwrap();
    let cols_sum_max: u64 = (0..grid.rows)
        .map(|col| grid.values.iter().skip(col).step_by(grid.cols).sum())
        .max()
        .unwrap();

    // We don't modulo the sums, so they can be bigger than VALUES_COUNT.
    rows_sum_max.max(cols_sum_max)
}

fn part1(grid: &Grid, instructions: &[Instruction]) -> u64 {
    let mut grid = grid.clone();
    for ins in instructions {
        ins.apply(&mut grid);
    }

    largest_sum(&grid)
}

fn part2(grid: &Grid, instructions: &[Instruction], actions: &[Action]) -> u64 {
    let mut grid = grid.clone();

    let mut instructions: VecDeque<Instruction> = instructions.iter().copied().collect();
    let mut current_instruction: Option<Instruction> = None;

    for action in actions {
        // println!("{action:?}: {current_instruction:?}");
        match action {
            Action::Take => {
                current_instruction = Some(instructions.pop_front().expect("Empty actions list"));
            }
            Action::Cycle => {
                if let Some(instr) = current_instruction.take() {
                    instructions.push_back(instr);
                } else {
                    panic!("Cycle action but no current instruction");
                }
            }
            Action::Act => {
                if let Some(instr) = current_instruction.take() {
                    instr.apply(&mut grid);
                } else {
                    panic!("Act action but no current instruction");
                }
            }
        }
    }

    largest_sum(&grid)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (grid, instructions, actions) = build(&input);

    println!("Part 1: {}", part1(&grid, &instructions));
    println!("Part 2: {}", part2(&grid, &instructions, &actions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_shift_row() {
        let mut grid: Grid = "0 0 0 0 0\n1 5 8 2 3\n0 0 0 0 0".into();
        let instruction: Instruction = "SHIFT ROW 2 BY 3".into();
        instruction.apply(&mut grid);

        assert_eq!(grid.values[grid.pos(1, 0)], 8);
        assert_eq!(grid.values[grid.pos(1, 1)], 2);
        assert_eq!(grid.values[grid.pos(1, 2)], 3);
        assert_eq!(grid.values[grid.pos(1, 3)], 1);
        assert_eq!(grid.values[grid.pos(1, 4)], 5);
    }

    #[test]
    fn test_sub_row() {
        let mut grid: Grid = "0 0 0 0 0\n0 0 0 0 0\n6 3 1 9 2".into();
        let instruction: Instruction = "SUB 7 ROW 3".into();
        instruction.apply(&mut grid);

        assert_eq!(grid.values[grid.pos(2, 0)], 1073741823);
        assert_eq!(grid.values[grid.pos(2, 1)], 1073741820);
        assert_eq!(grid.values[grid.pos(2, 2)], 1073741818);
        assert_eq!(grid.values[grid.pos(2, 3)], 2);
        assert_eq!(grid.values[grid.pos(2, 4)], 1073741819);
    }

    #[test]
    fn test_part1() {
        let (grid, instructions, _actions) = build(&INPUT_TEST);
        assert_eq!(part1(&grid, &instructions), 18938);
    }

    #[test]
    fn test_part2() {
        let (grid, instructions, actions) = build(&INPUT_TEST);
        assert_eq!(part2(&grid, &instructions, &actions), 11496);
    }
}
