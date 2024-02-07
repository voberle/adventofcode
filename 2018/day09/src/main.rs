use std::io::{self, Read};

use regex::Regex;

fn build(input: &str) -> (usize, u32) {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let p = re.captures(input).unwrap();
    (p[1].parse().unwrap(), p[2].parse().unwrap())
}

fn winning_score(players_count: usize, last_marble: u32) -> u32 {
    let mut circle: Vec<u32> = vec![0];
    let mut marble = 1;
    let mut scores = vec![0; players_count];
    let mut current_pos = 0;
    for player in (0..players_count).cycle() {
        if marble > 0 && marble % 23 == 0 {
            scores[player] += marble;
            // equivalent of going back by 7, but doesn't risk going negative
            current_pos = (current_pos + circle.len() - 1 - 7).rem_euclid(circle.len());
            scores[player] += circle.remove(current_pos);
            current_pos = (current_pos + 1).rem_euclid(circle.len());
        } else {
            circle.insert(current_pos + 1, marble);
            current_pos = (current_pos + 2).rem_euclid(circle.len());
        }
        // println!("[{}] {:?} - {}", player + 1, circle, current_pos);

        if marble == last_marble {
            // game over
            break;
        }
        marble += 1;
    }
    *scores.iter().max().unwrap()
}

fn part2(players_count: usize, last_marble: u32) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (player_count, last_marble) = build(input.trim());

    println!("Part 1: {}", winning_score(player_count, last_marble));
    println!("Part 2: {}", winning_score(player_count, last_marble));
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
