use std::io::{self, Read};

use dlv_list::{Index, VecList};
use regex::Regex;

fn build(input: &str) -> (usize, u32) {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let p = re.captures(input).unwrap();
    (p[1].parse().unwrap(), p[2].parse().unwrap())
}

fn move_index_back(circle: &mut VecList<u32>, index: Index<u32>) -> Index<u32> {
    circle
        .get_previous_index(index)
        .unwrap_or(circle.back_index().unwrap())
}

fn move_index_forward(circle: &mut VecList<u32>, index: Index<u32>) -> Index<u32> {
    circle
        .get_next_index(index)
        .unwrap_or(circle.front_index().unwrap())
}

fn winning_score(players_count: usize, last_marble: u32) -> u32 {
    // A semi-doubly linked list implemented with a vector.
    // Allows for fast insert in the middle.
    // Since we don't have to do a lot of list navigation (2 forwards, 7 backwards), it works.
    let mut circle: VecList<u32> = VecList::new();
    circle.push_back(0);
    // The index that tracks our position in the circle.
    let mut index = circle.front_index().unwrap();

    let mut marble = 1;
    let mut scores = vec![0; players_count];
    for player in (0..players_count).cycle() {
        if marble > 0 && marble % 23 == 0 {
            scores[player] += marble;
            for _ in 0..7 {
                index = move_index_back(&mut circle, index);
            }

            // Once we remove the index, we cannot use it really anymore, so we copy it and move already to next one.
            let index_to_remove = index;
            index = move_index_forward(&mut circle, index);
            scores[player] += circle.remove(index_to_remove).unwrap();
        } else {
            index = move_index_forward(&mut circle, index);
            index = circle.insert_after(index, marble);
        }
        // println!("[{}] {:?} - {:?}", player + 1, circle, index);

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
