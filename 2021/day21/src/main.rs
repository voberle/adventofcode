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
    value: u32,
    roll_count: u32,
}

impl DeterministicDice {
    fn new() -> Self {
        Self {
            // Before first roll, value is 0, we will start with 1 on first roll.
            value: 0,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        self.value += 1;
        // Dice wraps around at 100.
        self.value %= 100;
        self.roll_count += 1;
        self.value
    }
}

fn roll_dice_3_times(dice: &mut DeterministicDice) -> u32 {
    // Dice is rolled 3 times by each player.
    dice.roll() + dice.roll() + dice.roll()
}

fn play(
    player1_pos: u32,
    player2_pos: u32,
    dice: &mut DeterministicDice,
    winning_score: u32,
) -> (PlayerState, PlayerState) {
    let mut player1 = PlayerState::new(player1_pos);
    let mut player2 = PlayerState::new(player2_pos);

    loop {
        player1.update(roll_dice_3_times(dice));
        if player1.score >= winning_score {
            break;
        }

        player2.update(roll_dice_3_times(dice));
        if player2.score >= winning_score {
            break;
        }
    }
    (player1, player2)
}

fn practice_game_result(pos1: u32, pos2: u32) -> u32 {
    let mut dice = DeterministicDice::new();

    let (player1, player2) = play(pos1, pos2, &mut dice, 1000);

    let losing_score = player1.score.min(player2.score);
    losing_score * dice.roll_count
}

fn real_game_result(pos1: u32, pos2: u32) -> u64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (pos1, pos2) = build(&input);

    println!("Part 1: {}", practice_game_result(pos1, pos2));
    println!("Part 2: {}", real_game_result(pos1, pos2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (pos1, pos2) = build(INPUT_TEST);
        assert_eq!(practice_game_result(pos1, pos2), 739785);
    }

    #[test]
    fn test_part2() {
        let (pos1, pos2) = build(INPUT_TEST);
        assert_eq!(real_game_result(pos1, pos2), 444356092776315);
    }
}
