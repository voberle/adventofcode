use std::{
    collections::VecDeque,
    io::{self, Read},
};

use regex::Regex;

fn build(input: &str) -> (usize, u32) {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let p = re.captures(input).unwrap();
    (p[1].parse().unwrap(), p[2].parse().unwrap())
}

fn winning_score(players_count: usize, last_marble: u32) -> u32 {
    // Using a VecDecque where we push only back and pop front,
    // and rotate it instead of tracking an index.
    let mut circle: VecDeque<u32> = VecDeque::with_capacity(last_marble as usize + 1);
    circle.push_back(0);

    let mut marble = 1;
    let mut scores = vec![0; players_count];
    for player in (0..players_count).cycle() {
        if marble > 0 && marble % 23 == 0 {
            scores[player] += marble;
            // "Moving back" by rotating the queue.
            for _ in 0..7 {
                let back = circle.pop_back().unwrap();
                circle.push_front(back);
            }
            scores[player] += circle.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let front = circle.pop_front().unwrap();
                circle.push_back(front);
            }
            circle.push_front(marble);
        }

        if marble == last_marble {
            // game over
            break;
        }
        marble += 1;
    }
    *scores.iter().max().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (player_count, last_marble) = build(input.trim());

    println!("Part 1: {}", winning_score(player_count, last_marble));
    println!("Part 2: {}", winning_score(player_count, last_marble * 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning_elf_score() {
        assert_eq!(winning_score(9, 25), 32);
    }

    #[test]
    fn test_part1() {
        let (player_count, last_marble_worth) =
            build("10 players; last marble is worth 1618 points");
        assert_eq!(winning_score(player_count, last_marble_worth), 8317);
        let (player_count, last_marble_worth) =
            build("13 players; last marble is worth 7999 points");
        assert_eq!(winning_score(player_count, last_marble_worth), 146373);
        let (player_count, last_marble_worth) =
            build("17 players; last marble is worth 1104 points");
        assert_eq!(winning_score(player_count, last_marble_worth), 2764);
        let (player_count, last_marble_worth) =
            build("21 players; last marble is worth 6111 points");
        assert_eq!(winning_score(player_count, last_marble_worth), 54718);
        let (player_count, last_marble_worth) =
            build("30 players; last marble is worth 5807 points");
        assert_eq!(winning_score(player_count, last_marble_worth), 37305);
    }
}
