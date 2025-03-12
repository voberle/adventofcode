use std::io::{self, Read};

struct Park {
    // Since we wrap around when walking, using a single dimension vector is very practical.
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Park {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn print_with_pos(&self, positions: &[usize]) {
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    print!("[{c}]");
                    // print!("X");
                } else {
                    print!("{c}");
                    // print!(" ");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.print_with_pos(&[]);
    }
}

// Dog poo we walked into count.
fn answer(park: &Park) -> usize {
    let mut poo_count = 0;

    let mut row = 0;
    let mut col = 0;
    loop {
        let p = park.pos(row, col);
        if p >= park.values.len() {
            break;
        }

        // park.print_with_pos(&[p]);
        // println!("-----------");

        if park.values[p] == '💩' {
            poo_count += 1;
        }

        col = (col + 2) % park.cols;
        row += 1;
    }
    poo_count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let park = Park::build(&input);

    println!("Answer: {}", answer(&park));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_answer() {
        assert_eq!(answer(&Park::build(INPUT_TEST)), 2);
    }
}
