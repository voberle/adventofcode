use std::io::{self, Read};

fn get_hundreds_digit(n: i32) -> i32 {
    (n / 100) % 10
}

fn power_level(x: usize, y: usize, serial_number: i32) -> i32 {
    let x = i32::try_from(x).unwrap();
    let y = i32::try_from(y).unwrap();
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    get_hundreds_digit(power_level) - 5
}

const GRID_SIZE: usize = 300;

fn build_power_level_grid(serial_number: i32) -> Vec<Vec<i32>> {
    (1..=GRID_SIZE)
        .map(|y| {
            (1..=GRID_SIZE)
                .map(move |x| power_level(x, y, serial_number))
                .collect()
        })
        .collect()
}

fn power_level_square(x: usize, y: usize, square_size: usize, grid: &[Vec<i32>]) -> i32 {
    (y..y + square_size)
        .map(|y| {
            (x..x + square_size)
                .map(|x| grid[y - 1][x - 1])
                .sum::<i32>()
        })
        .sum()
}

fn largest_power_with_cache(grid: &[Vec<i32>], square_size: usize) -> (usize, usize, i32) {
    (1..=GRID_SIZE - square_size + 1)
        .flat_map(|y| {
            (1..=GRID_SIZE - square_size + 1)
                .map(move |x| (x, y, power_level_square(x, y, square_size, grid)))
        })
        .max_by_key(|(_, _, p)| *p)
        .unwrap()
}

fn largest_power_3x3(serial_number: i32) -> (usize, usize) {
    let grid = build_power_level_grid(serial_number);
    let m = largest_power_with_cache(&grid, 3);
    (m.0, m.1)
}

fn largest_power_any_size(serial_number: i32) -> (usize, usize, usize) {
    let grid = build_power_level_grid(serial_number);
    // Brute-forced
    let m = (1..=300)
        .map(|square_size| (square_size, largest_power_with_cache(&grid, square_size)))
        .max_by_key(|(_, (_, _, p))| *p)
        .unwrap();
    (m.1 .0, m.1 .1, m.0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let serial_number = input.trim().parse().unwrap();

    let (x, y) = largest_power_3x3(serial_number);
    println!("Part 1: {x},{y}");
    let (x, y, size) = largest_power_any_size(serial_number);
    println!("Part 2: {x},{y},{size}");
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
        assert_eq!(largest_power_3x3(18), (33, 45));
        assert_eq!(largest_power_3x3(42), (21, 61));
    }

    #[ignore] // a bit too slow
    #[test]
    fn test_part2() {
        assert_eq!(largest_power_any_size(18), (90, 269, 16));
        assert_eq!(largest_power_any_size(42), (232, 251, 12));
    }
}
