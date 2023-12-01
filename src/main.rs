// https://adventofcode.com/2023/day/1

fn main() {
    let stdin = std::io::stdin();
    let mut total: u32 = 0;
    for line in stdin.lines() {
        let s = line.unwrap();
        let d: Vec<Option<u32>> = s.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10))
            .collect();
        if !d.is_empty() {
            let d1 = d.first().unwrap().unwrap();
            let d2 = d.last().unwrap().unwrap();
            let line_total = d1 * 10 + d2;
            dbg!(line_total);
            total += line_total;
        }
    }
    println!("{total}");
}
