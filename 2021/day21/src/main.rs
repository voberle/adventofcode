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

fn roll_dice_3_times(dice: &mut u32) -> u32 {
    // Dice is rolled 3 times by each player, and dice wraps around at 100.
    let mut r = 0;
    for _ in 0..3 {
        r += *dice;
        *dice += 1;
        *dice %= 100;
    }
    r
}

fn next_position(pos: u32, rolls: u32) -> u32 {
    // Positions above 10 wrap back around to 1.
    // Removing 1 before doing the modulo, to get it to work correctly.
    let unwrapped_pos = pos + rolls - 1;
    unwrapped_pos % 10 + 1
}

fn play_once_for(
    dice: &mut u32,
    dice_roll_count: &mut u32,
    pos: &mut u32,
    score: &mut u32,
) -> bool {
    let rolls = roll_dice_3_times(dice);
    *dice_roll_count += 3;
    *pos = next_position(*pos, rolls);
    *score += *pos;
    // println!("Player pos {}, score {}", pos, score);

    *score >= 1000
}

fn play(player1: u32, player2: u32) -> (u32, u32, u32) {
    let mut score1 = 0;
    let mut score2 = 0;

    let mut pos1 = player1;
    let mut pos2 = player2;

    let mut dice = 1;
    let mut dice_roll_count = 0;

    loop {
        if play_once_for(&mut dice, &mut dice_roll_count, &mut pos1, &mut score1) {
            break;
        }
        if play_once_for(&mut dice, &mut dice_roll_count, &mut pos2, &mut score2) {
            break;
        }
    }
    (score1, score2, dice_roll_count)
}

fn loosing_score_x_dire_rolls(player1: u32, player2: u32) -> u32 {
    let (score1, score2, dice) = play(player1, player2);
    let losing_score = score1.min(score2);
    // println!("Loosing score {}; Dice rolled {} times", losing_score, dice);
    losing_score * dice
}

fn part2(player1: u32, player2: u32) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (player1, player2) = build(&input);

    println!("Part 1: {}", loosing_score_x_dire_rolls(player1, player2));
    println!("Part 2: {}", part2(player1, player2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (player1, player2) = build(INPUT_TEST);
        assert_eq!(loosing_score_x_dire_rolls(player1, player2), 739785);
    }

    #[test]
    fn test_part2() {
        let (player1, player2) = build(INPUT_TEST);
        assert_eq!(part2(player1, player2), 0);
    }
}
