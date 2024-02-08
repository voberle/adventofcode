use std::io::{self, Read};

fn get_hundreds_digit(n: i32) -> i32 {
    (n / 100) % 10
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    get_hundreds_digit(power_level) - 5
}

fn power_level_square(x: i32, y: i32, serial_number: i32) -> i32 {
    (y..y + 3)
        .map(|y| {
            (x..x + 3)
                .map(|x| power_level(x, y, serial_number))
                .sum::<i32>()
        })
        .sum()
}

fn largest_power(serial_number: i32) -> (i32, i32) {
    const GRID_SIZE: i32 = 300;
    let m = (1..=GRID_SIZE - 2)
        .flat_map(|y| {
            (1..=GRID_SIZE - 2).map(move |x| (x, y, power_level_square(x, y, serial_number)))
        })
        .max_by_key(|(_, _, p)| *p)
        .unwrap();
    (m.0, m.1)
}

fn part2(serial_number: i32) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let serial_number = input.trim().parse().unwrap();

    let (x, y) = largest_power(serial_number);
    println!("Part 1: {},{}", x, y);
    println!("Part 2: {}", part2(serial_number));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hundreds_digit() {
        assert_eq!(get_hundreds_digit(941), 9);
        assert_eq!(get_hundreds_digit(94), 0);
        assert_eq!(get_hundreds_digit(9413), 4);
    }

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(largest_power(18), (33, 45));
        assert_eq!(largest_power(42), (21, 61));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
