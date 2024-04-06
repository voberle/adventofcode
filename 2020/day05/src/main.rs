use std::io::{self, Read};

enum Dir {
    Front,
    Back,
    Left,
    Right,
}

impl Dir {
    fn new(c: char) -> Self {
        match c {
            'F' => Dir::Front,
            'B' => Dir::Back,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!("Invalid dir"),
        }
    }
}

struct BoardingPass {
    dirs: Vec<Dir>,
}

impl BoardingPass {
    fn build(input: &str) -> Self {
        Self {
            dirs: input.chars().map(Dir::new).collect(),
        }
    }

    fn partition(dirs: &[Dir], mut low: u32, mut high: u32) -> u32 {
        for d in dirs {
            let mid = (high - low) / 2 + 1;
            match d {
                // Lower half
                Dir::Front | Dir::Left => high -= mid,
                // Upper half
                Dir::Back | Dir::Right => low += mid,
            }
        }
        low
    }

    fn seat(&self) -> (u32, u32) {
        (
            Self::partition(&self.dirs[0..7], 0, 127),
            Self::partition(&self.dirs[7..10], 0, 7),
        )
    }

    fn seat_id(&self) -> u32 {
        let (row, col) = self.seat();
        row * 8 + col
    }
}

fn build(input: &str) -> Vec<BoardingPass> {
    input.lines().map(BoardingPass::build).collect()
}

fn highest_seat_id(boarding_passes: &[BoardingPass]) -> u32 {
    boarding_passes
        .iter()
        .map(BoardingPass::seat_id)
        .max()
        .unwrap()
}

fn part2(boarding_passes: &[BoardingPass]) -> u32 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let boarding_passes = build(&input);

    println!("Part 1: {}", highest_seat_id(&boarding_passes));
    println!("Part 2: {}", part2(&boarding_passes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_ids() {
        let pass = BoardingPass::build("BFFFBBFRRR");
        assert_eq!(pass.seat(), (70, 7));
        assert_eq!(pass.seat_id(), 567);
        let pass = BoardingPass::build("FFFBBBFRRR");
        assert_eq!(pass.seat(), (14, 7));
        assert_eq!(pass.seat_id(), 119);
        let pass = BoardingPass::build("BBFFBBFRLL");
        assert_eq!(pass.seat(), (102, 4));
        assert_eq!(pass.seat_id(), 820);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
