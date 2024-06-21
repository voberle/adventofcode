use std::io::{self, Read};

fn analyze_games(games: &str) -> (u32, u32) {
    let mut sum_ids = 0;
    let mut power_sum = 0;
    for line in games.lines() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let split: Vec<&str> = line.split(':').collect();
        let game_id: u32 = split[0].strip_prefix("Game ").unwrap().parse().unwrap();
        let reveals: Vec<&str> = split[1].split(';').collect();
        let mut test = true;

        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;
        for r in reveals {
            let col: Vec<&str> = r.split(',').collect();
            for c in col {
                let x: Vec<&str> = c.trim().split_ascii_whitespace().collect();
                let cube_count: u32 = x[0].parse().unwrap();
                let cube_color = x[1];
                match cube_color {
                    "red" => {
                        if cube_count > 12 {
                            test = false;
                        }
                        red_count = u32::max(red_count, cube_count);
                    }
                    "green" => {
                        if cube_count > 13 {
                            test = false;
                        }
                        green_count = u32::max(green_count, cube_count);
                    }
                    "blue" => {
                        if cube_count > 14 {
                            test = false;
                        }
                        blue_count = u32::max(blue_count, cube_count);
                    }
                    _ => {}
                }
            }
        }
        if test {
            sum_ids += game_id;
        }
        power_sum += red_count * green_count * blue_count;
    }
    (sum_ids, power_sum)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (sum_ids, power_sum) = analyze_games(&input);

    println!("Part 1: {}", sum_ids);
    println!("Part 2: {}", power_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test");

    #[test]
    fn test_part1() {
        let (sum_ids, _) = analyze_games(INPUT_TEST);
        assert_eq!(sum_ids, 8);
    }

    #[test]
    fn test_part2() {
        let (_, power_sum) = analyze_games(INPUT_TEST);
        assert_eq!(power_sum, 2286);
    }
}
