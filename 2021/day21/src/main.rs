use std::{
    collections::BTreeMap,
    io::{self, Read},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Turn {
    One,
    Two,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Game {
    player1: PlayerState,
    player2: PlayerState,
    turn: Turn,
}

impl Game {
    fn losing_score(&self) -> u32 {
        self.player1.score.min(self.player2.score)
    }
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let player1_pos = lines[0]
            .strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap();
        let player2_pos = lines[1]
            .strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap();
        Self {
            player1: PlayerState::new(player1_pos),
            player2: PlayerState::new(player2_pos),
            turn: Turn::One,
        }
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

fn practice_game_result(game: &Game) -> u32 {
    const WINNING_SCORE: u32 = 1000;

    let mut game = *game;
    let mut dice = DeterministicDice::new();

    loop {
        if game.turn == Turn::One {
            game.player1.update(roll_dice_3_times(&mut dice));
            if game.player1.score >= WINNING_SCORE {
                break;
            }
            game.turn = Turn::Two;
        }

        if game.turn == Turn::Two {
            game.player2.update(roll_dice_3_times(&mut dice));
            if game.player2.score >= WINNING_SCORE {
                break;
            }
            game.turn = Turn::One;
        }
    }

    game.losing_score() * dice.roll_count
}

// We track all universes in a map of Game => Number of copies.
// Only active games are kept there. Once a game finishes, it's removed and winning counters are updated.
// The implementation uses a BTreeMap as we need a way to pop from the map.
struct Universes {
    active: BTreeMap<Game, u64>,
    won_by_1: u64,
    won_by_2: u64,
}

impl Universes {
    fn new() -> Self {
        Self {
            active: BTreeMap::default(),
            won_by_1: 0,
            won_by_2: 0,
        }
    }

    // Adds a game to the universes.
    fn add(&mut self, game: Game, count: u64) {
        self.active
            .entry(game)
            .and_modify(|e| *e += count)
            .or_insert(count);
    }

    fn most_universes_won(&self) -> u64 {
        self.won_by_1.max(self.won_by_2)
    }
}

fn real_game_result(game: &Game) -> u64 {
    const WINNING_SCORE: u32 = 21;

    // Roll dice 3 times. Each roll creates 3 copies: 1, 2, 3.
    // This means we have following possible dice values:
    const DICE_INC: [u32; 27] = [
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ];

    let mut universes = Universes::new();
    universes.add(*game, 1);

    while let Some((game, count)) = universes.active.pop_last() {
        // Player 1
        for inc in DICE_INC {
            let mut copy = game;

            if game.turn == Turn::One {
                copy.player1.update(inc);
                copy.turn = Turn::Two;
                if copy.player1.score >= WINNING_SCORE {
                    universes.won_by_1 += count;
                } else {
                    universes.add(copy, count);
                }
            } else {
                copy.player2.update(inc);
                copy.turn = Turn::One;
                if copy.player2.score >= WINNING_SCORE {
                    universes.won_by_2 += count;
                } else {
                    universes.add(copy, count);
                }
            }
        }
    }

    universes.most_universes_won()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let game: Game = Game::from(input.as_ref());

    println!("Part 1: {}", practice_game_result(&game));
    println!("Part 2: {}", real_game_result(&game));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let game = Game::from(INPUT_TEST);
        assert_eq!(practice_game_result(&game), 739785);
    }

    #[test]
    fn test_part2() {
        let game = Game::from(INPUT_TEST);
        assert_eq!(real_game_result(&game), 444356092776315);
    }
}
