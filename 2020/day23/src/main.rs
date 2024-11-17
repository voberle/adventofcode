use std::io::{self, Read};

mod brute_force;

fn build(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[allow(dead_code)]
fn cups_next_to_string(cups_next: &[usize], current: usize) -> String {
    let mut result = String::new();
    let mut p = current;
    loop {
        result += &format!("{p} ");
        p = cups_next[p];
        if p == current {
            break;
        }
    }
    result
}

fn initialize_cup_next(cups: &[u32]) -> Vec<usize> {
    let mut cups_next: Vec<usize> = vec![0; cups.len() + 1];
    // There is no cup 0.
    cups_next[0] = usize::MAX;
    let len = cups.len();
    for i in 0..len {
        cups_next[cups[i] as usize] = cups[(i + 1).rem_euclid(len)] as usize;
    }
    cups_next
}

// Cups are tracked into a vec that indicates what is the next cup after each one.
fn move_cups(cups_next: &mut [usize], moves: usize, start_cup: usize) {
    let min_cup = 1;
    let max_cup = cups_next.len() - 1;

    let mut current = start_cup;
    for _m in 0..moves {
        // println!("-- move {} --", _m + 1);
        // println!("cups: {}", cups_next_to_string(&cups_next, current));

        let picks1 = cups_next[current];
        let picks2 = cups_next[picks1];
        let picks3 = cups_next[picks2];
        let picks = [picks1, picks2, picks3];
        // println!("pick up: {:?}", picks);

        // Cup label to use for the destination cup
        let mut destination_cup = current - 1;
        if destination_cup < min_cup {
            destination_cup = max_cup;
        }
        while picks.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup < min_cup {
                destination_cup = max_cup;
            }
        }

        // println!("destination: {}", destination_cup);

        // Move cups:
        // After current cup comes the cup after the 3rd one we removed.
        cups_next[current] = cups_next[picks3];
        // The 3 cups we removed are placed after the destination cup.
        cups_next[picks3] = cups_next[destination_cup];
        cups_next[destination_cup] = picks1;

        // Select new current cup.
        current = cups_next[current];
    }
}

fn cups_after_1(cups: &[u32], moves: usize) -> String {
    let mut cups_next = initialize_cup_next(cups);

    move_cups(&mut cups_next, moves, cups[0] as usize);

    // Return the list of cups after cup 1.
    let mut result = String::new();
    let mut p = cups_next[1];
    while p != 1 {
        result.push(char::from_digit(u32::try_from(p).unwrap(), 10).unwrap());
        p = cups_next[p];
    }
    result
}

fn extend_cups(cups: &mut Vec<u32>, count: usize) {
    let start_at = *cups.iter().max().unwrap() as usize + 1;
    let nb_to_add = count - cups.len();
    cups.extend((start_at..start_at + nb_to_add).map(|v| u32::try_from(v).unwrap()));
}

fn two_cups_after_1(cups: &[u32]) -> (usize, usize) {
    const CUPS_COUNT: usize = 1_000_000;
    const MOVES: usize = 10_000_000;

    // It's a bit unefficient to create this big temporary vec with one million cups, but it's fast and simple.
    let mut extended_cups = cups.to_vec();
    extend_cups(&mut extended_cups, CUPS_COUNT);

    let mut cups_next = initialize_cup_next(&extended_cups);

    move_cups(&mut cups_next, MOVES, cups[0] as usize);

    (cups_next[1], cups_next[cups_next[1]])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cups = build(input.trim());

    println!("Part 1: {}", cups_after_1(&cups, 100));

    let (c1, c2) = two_cups_after_1(&cups);
    println!("Part 2: {}", c1 * c2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = "389125467";

    #[test]
    fn test_part1() {
        assert_eq!(cups_after_1(&build(INPUT_TEST), 10), "92658374");
        assert_eq!(cups_after_1(&build(INPUT_TEST), 100), "67384529");
    }

    #[test]
    fn test_part2() {
        let (c1, c2) = two_cups_after_1(&build(INPUT_TEST));
        assert_eq!(c1, 934001);
        assert_eq!(c2, 159792);
        assert_eq!(c1 * c2, 149245887792);
    }
}
