use std::io::{self, Read};

struct Tile {
    id: u64,
    grid: Vec<bool>,
}

impl Tile {
    const SIZE: usize = 10;

    fn print(&self) {
        for row in 0..Tile::SIZE {
            for p in row * Tile::SIZE..(row + 1) * Tile::SIZE {
                let c = self.grid[p];
                print!("{}", if c { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn build(input: &str) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    let it = input.lines();

    let mut id = 0;
    let mut grid = Vec::new();
    for line in it {
        if line.is_empty() {
            tiles.push(Tile { id, grid });
            id = 0;
            grid = Vec::new();
            continue;
        }
        if line.starts_with("Tile ") {
            assert_eq!(id, 0);
            id = line
                .trim_start_matches("Tile ")
                .trim_end_matches(':')
                .parse()
                .unwrap();
            continue;
        }
        grid.extend(line.chars().map(|c| c == '#'));
    }
    tiles.push(Tile { id, grid });
    tiles
}

fn part1(tiles: &[Tile]) -> i64 {
    0
}

fn part2(tiles: &[Tile]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tiles = build(&input);
    // for t in &tiles {
    //     println!("{} {}", t.id, t.grid.len());
    //     t.print();
    // }

    println!("Part 1: {}", part1(&tiles));
    println!("Part 2: {}", part2(&tiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
