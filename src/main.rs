// https://adventofcode.com/2023/day/1

fn main() {
    let stdin = std::io::stdin();
    let mut total: u32 = 0;
    for line in stdin.lines() {
        let d: Vec<u32> = line
            .unwrap()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        if !d.is_empty() {
            let d1 = d.first().unwrap();
            let d2 = d.last().unwrap();
            let line_total = d1 * 10 + d2;
            //dbg!(line_total);
            total += line_total;
        }
    }
    println!("{total}");
}
