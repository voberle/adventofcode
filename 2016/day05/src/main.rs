use std::io::{self, Read};

fn find_password(door_id: &str) -> String {
    const START: &str = "00000";
    let mut password = String::with_capacity(8);
    let mut index = 0;
    while password.len() < 8 {
        let digest = md5::compute(format!("{}{}", door_id, index).as_bytes());
        let s = format!("{:x}", digest);
        if s.starts_with(START) {
            password.push(s.trim_start_matches(START)[0..1].chars().next().unwrap());
        }
        index += 1;
    }
    password
}

fn part2(door_id: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", find_password(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(find_password("abc"), "18f47a30");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), 0);
    }
}
