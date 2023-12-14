// https://adventofcode.com/2023/day/1
// input_test: 142
// input_test2: 281
// input part 1: 54990
// input part 2: 54473

fn main() {
    let stdin = std::io::stdin();
    const PART2: bool = true;
    let strings = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut total: u32 = 0;
    for line in stdin.lines() {
        let mut s = line.unwrap();
        if PART2 {
            dbg!(&s);
            let mut i = 0;
            // We need to make sure we replace the first number we find
            // The right calibration values for string "eighthree" is 83 and for "sevenine" is 79.
            while i < s.len() {
                for pair in strings {
                    if s[i..].starts_with(pair.0) {
                        s.replace_range(i..i+1, &pair.1.to_string());
                    }
                }
                i += 1;
            }
            dbg!(&s);
        }
        let d: Vec<u32> = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        if !d.is_empty() {
            let d1 = d.first().unwrap();
            let d2 = d.last().unwrap();
            let line_total = d1 * 10 + d2;
            dbg!(line_total);
            total += line_total;
        }
    }
    println!("{total}");
}
