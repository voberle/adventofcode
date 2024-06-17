use std::io::{self, Read};

// We don't need to track all blizzards on each minute. We can calculate for each blizzard where
// they will be at each minute, and we are only interested in the state of 4 positions.
// Also only a sub-set of blizzards can reach a specific position, those on same row or column.
// // Horizontal blizzards
// left_blizzards: Vec<Vec<usize>>;
// Each entry represents a line, and lines contains the column where the blizzard is.
// current_column = (initial_column + minute) % line_len

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Valley {
    up_blizzards: Vec<Vec<usize>>,
    down_blizzards: Vec<Vec<usize>>,
    left_blizzards: Vec<Vec<usize>>,
    right_blizzards: Vec<Vec<usize>>,
    horizontal_len: usize,
    vertical_len: usize,
    entrance: Pos,
    exit: Pos,
}

impl From<&str> for Valley {
    fn from(input: &str) -> Self {
        let mut up_blizzards: Vec<Vec<usize>> = vec![vec![]];
        let mut down_blizzards: Vec<Vec<usize>> = vec![vec![]];
        let mut left_blizzards: Vec<Vec<usize>> = vec![vec![]];
        let mut right_blizzards: Vec<Vec<usize>> = vec![vec![]];
        let mut horizontal_len = 0;
        let mut vertical_len = 0;
        let mut entrance = Pos { x: 0, y: 0 };
        let mut exit = Pos { x: 0, y: 0 };

        for (y, line) in input.lines().enumerate() {
            for (x, e) in line.chars().enumerate() {
                if y == 0 {
                    if e == '.' {
                        entrance.x = x;
                        entrance.y = 0;
                    } else {
                        assert_eq!(e, '#');
                    }
                    horizontal_len += 1;
                } else {
                    match e {
                        '.' => {
                            // Last dot found will be the exit.
                            exit.x = x;
                            exit.y = y;
                        }
                        '^' => {
                            while up_blizzards.len() <= x {
                                up_blizzards.push(vec![]);
                            }
                            up_blizzards[x].push(y);
                        }
                        'v' => {
                            while down_blizzards.len() <= x {
                                down_blizzards.push(vec![]);
                            }
                            down_blizzards[x].push(y);
                        }
                        '<' => {
                            while left_blizzards.len() <= y {
                                left_blizzards.push(vec![]);
                            }
                            left_blizzards[y].push(x);
                        }
                        '>' => {
                            while right_blizzards.len() <= y {
                                right_blizzards.push(vec![]);
                            }
                            right_blizzards[y].push(x);
                        }
                        '#' => {}
                        _ => panic!("Invalid map item"),
                    }
                }
            }
            vertical_len += 1;
        }
        horizontal_len -= 2;
        vertical_len -= 2;
        Self {
            up_blizzards,
            down_blizzards,
            left_blizzards,
            right_blizzards,
            horizontal_len,
            vertical_len,
            entrance,
            exit,
        }
    }
}

impl Valley {
    fn is_blizzard_at(&self, x: usize, y: usize, minute: usize) -> bool {
        for initial_x in &self.left_blizzards[y] {
            let current_x = (initial_x - minute).rem_euclid(self.horizontal_len);
            if x == current_x {
                return true;
            }
        }
        for initial_x in &self.right_blizzards[y] {
            let current_x = (initial_x + minute).rem_euclid(self.horizontal_len);
            if x == current_x {
                return true;
            }
        }
        for initial_y in &self.up_blizzards[x] {
            let current_y = (initial_y - minute).rem_euclid(self.vertical_len);
            if y == current_y {
                return true;
            }
        }
        for initial_y in &self.down_blizzards[x] {
            let current_y = (initial_y + minute).rem_euclid(self.vertical_len);
            if y == current_y {
                return true;
            }
        }
        false
    }
}

fn time_to_reach_goal(valley: &Valley) -> usize {
    0
}

fn part2(valley: &Valley) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let valley = input.as_str().into();
    // println!("{:?}", valley);

    println!("Part 1: {}", time_to_reach_goal(&valley));
    println!("Part 2: {}", part2(&valley));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(time_to_reach_goal(&INPUT_TEST.into()), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT_TEST.into()), 0);
    }
}
