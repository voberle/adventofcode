use std::io::{self, Read};

fn build(input: &str) -> Vec<i64> {
    // Warning: The real input contains duplicates.
    input.lines().map(|v| v.parse().unwrap()).collect()
}

// Store the value of the coordinate and its original position.
#[derive(Debug, Clone, Copy)]
struct Coord {
    value: i64,
    position: usize,
}

fn create_ext_list<const DECRYPTION_KEY: i64>(coordinates: &[i64]) -> Vec<Coord> {
    coordinates
        .iter()
        .enumerate()
        .map(|(position, v)| Coord {
            value: v * DECRYPTION_KEY,
            position,
        })
        .collect()
}

// Version of move method that uses Vector::swap.
// Works for part 1, but doesn't scale for part 2.
#[allow(
    dead_code,
    clippy::comparison_chain,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn move_item_swap(decrypted: &mut [Coord], index: usize) {
    let pos = decrypted.iter().position(|c| c.position == index).unwrap();
    let item = decrypted[pos].value;
    if item > 0 {
        let len = decrypted.len();
        for p in pos..pos + item as usize {
            decrypted.swap(p.rem_euclid(len), (p + 1).rem_euclid(len));
        }
    } else if item < 0 {
        let len = decrypted.len() as isize;
        let mut p: isize = pos as isize;
        while p > pos as isize - item.abs() as isize {
            decrypted.swap(p.rem_euclid(len) as usize, (p - 1).rem_euclid(len) as usize);
            p -= 1;
        }
    }
}

// Version of move with remove/insert.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
fn move_item_insert(decrypted: &mut Vec<Coord>, index: usize) {
    let pos = decrypted.iter().position(|c| c.position == index).unwrap();

    let item_value = decrypted[pos].value;
    if item_value == 0 {
        return;
    }

    let item = decrypted.remove(pos);

    let new_pos = (pos as i64 + item_value).rem_euclid(decrypted.len() as i64);

    decrypted.insert(new_pos as usize, item);
}

fn sum_of_groove_coordinates(coords: &[Coord]) -> i64 {
    let pos_zero = coords.iter().position(|c| c.value == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let p = (pos_zero + offset).rem_euclid(coords.len());
            coords[p].value
        })
        .sum()
}

fn groove_coordinates_sum(coordinates: &[i64]) -> i64 {
    let mut decrypted = create_ext_list::<1>(coordinates);

    for index in 0..coordinates.len() {
        // move_item_swap(&mut decrypted, index);
        move_item_insert(&mut decrypted, index);
    }

    sum_of_groove_coordinates(&decrypted)
}

#[allow(clippy::unreadable_literal)]
fn groove_coordinates_sum_full(coordinates: &[i64]) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;

    let mut decrypted = create_ext_list::<DECRYPTION_KEY>(coordinates);

    for _ in 0..10 {
        for index in 0..coordinates.len() {
            move_item_insert(&mut decrypted, index);
        }
    }
    sum_of_groove_coordinates(&decrypted)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let coordinates = build(&input);

    println!("Part 1: {}", groove_coordinates_sum(&coordinates));
    println!("Part 2: {}", groove_coordinates_sum_full(&coordinates));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    fn as_vec(coords: &[Coord]) -> Vec<i64> {
        coords.iter().map(|c| c.value).collect()
    }

    #[test]
    fn test_move_item_insert() {
        let mut v = create_ext_list::<1>(&[1, 2, -3, 3, -2, 0, 4]);
        // 1 moves between 2 and -3:
        move_item_insert(&mut v, 0);
        assert_eq!(as_vec(&v), vec![2, 1, -3, 3, -2, 0, 4]);
        // 2 moves between -3 and 3:
        move_item_insert(&mut v, 1);
        assert_eq!(as_vec(&v), vec![1, -3, 2, 3, -2, 0, 4]);
        // -3 moves between -2 and 0:
        move_item_insert(&mut v, 2);
        assert_eq!(as_vec(&v), vec![1, 2, 3, -2, -3, 0, 4]);
        // 3 moves between 0 and 4:
        move_item_insert(&mut v, 3);
        assert_eq!(as_vec(&v), vec![1, 2, -2, -3, 0, 3, 4]);
        // -2 moves between 4 and 1:
        move_item_insert(&mut v, 4);
        v.rotate_left(1); // rotate by 1 to match the test cases in description.
        assert_eq!(as_vec(&v), vec![1, 2, -3, 0, 3, 4, -2]);
        // 0 does not move:
        move_item_insert(&mut v, 5);
        assert_eq!(as_vec(&v), vec![1, 2, -3, 0, 3, 4, -2]);
        // 4 moves between -3 and 0:
        move_item_insert(&mut v, 6);
        assert_eq!(as_vec(&v), vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_calc_groove_coordinates_sum() {
        let coords = create_ext_list::<1>(&[1, 2, -3, 4, 0, 3, -2]);
        assert_eq!(sum_of_groove_coordinates(&coords), 3);
    }

    #[test]
    fn test_other_case_1() {
        let coords = vec![3, 1, 0];
        // Final state: 3, 1, 0
        assert_eq!(groove_coordinates_sum(&coords), 4);
    }

    #[test]
    fn test_other_case_2() {
        let coords = vec![0, -1, -1, 1];
        // Final state: -1, 1, -1, 0
        assert_eq!(groove_coordinates_sum(&coords), 0);
    }

    #[test]
    fn test_other_case_part2() {
        let coords = vec![0, 4, 5, 1, 2, 3];
        assert_eq!(groove_coordinates_sum_full(&coords), 6492713224);
    }

    #[test]
    fn test_part1() {
        assert_eq!(groove_coordinates_sum(&build(INPUT_TEST)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(groove_coordinates_sum_full(&build(INPUT_TEST)), 1623178306);
    }
}
