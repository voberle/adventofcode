use std::io::{self, Read};

fn build(input: &str) -> (u32, u32) {
    let lines: Vec<_> = input.lines().collect();
    (
        lines[0]
            .strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
        lines[1]
            .strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
    )
}

struct PlayerState {
    pos: u32,
    score: u32,
}

impl PlayerState {
    fn new(pos: u32) -> Self {
        Self { pos, score: 0 }
    }

    fn update(&mut self, rolls: u32) {
        // Positions above 10 wrap back around to 1.
        // Removing 1 before doing the modulo, to get it to work correctly.
        self.pos = (self.pos + rolls - 1) % 10 + 1;

        self.score += self.pos;
        // println!("Player pos {}, score {}", self.pos, self.score);
    }
}

struct DeterministicDice {
    dice: u32,
    dice_roll_count: u32,
}

impl DeterministicDice {
    fn new() -> Self {
        Self {
            dice: 1,
            dice_roll_count: 0,
        }
    }

    fn roll_dice_3_times(&mut self) -> u32 {
        // Dice is rolled 3 times by each player, and dice wraps around at 100.
        let mut r = 0;
        for _ in 0..3 {
            r += self.dice;
            self.dice += 1;
            self.dice %= 100;
        }
        self.dice_roll_count += 3;
        r
    }
}

fn play_once_for(player: &mut PlayerState, dice: &mut DeterministicDice) -> bool {
    let rolls = dice.roll_dice_3_times();

    player.update(rolls);

    player.score >= 1000
}

fn play(player1_pos: u32, player2_pos: u32) -> (PlayerState, PlayerState, DeterministicDice) {
    let mut player1 = PlayerState::new(player1_pos);
    let mut player2 = PlayerState::new(player2_pos);

    let mut dice = DeterministicDice::new();

    loop {
        if play_once_for(&mut player1, &mut dice) {
            break;
        }
        if play_once_for(&mut player2, &mut dice) {
            break;
        }
    }
    (player1, player2, dice)
}

fn deterministic_dice_result(pos1: u32, pos2: u32) -> u32 {
    let (player1, player2, dice) = play(pos1, pos2);
    let losing_score = player1.score.min(player2.score);
    losing_score * dice.dice_roll_count
}

fn quantum_play_winning_universe_cnt(pos1: u32, pos2: u32) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (pos1, pos2) = build(&input);

    println!("Part 1: {}", deterministic_dice_result(pos1, pos2));
    println!("Part 2: {}", quantum_play_winning_universe_cnt(pos1, pos2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (pos1, pos2) = build(INPUT_TEST);
        assert_eq!(deterministic_dice_result(pos1, pos2), 739785);
    }

    #[test]
    fn test_part2() {
        let (pos1, pos2) = build(INPUT_TEST);
        assert_eq!(
            quantum_play_winning_universe_cnt(pos1, pos2),
            444356092776315
        );
    }
}
