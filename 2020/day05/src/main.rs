use std::io::{self, Read};

fn calc_seat_id(row: usize, col: usize) -> usize {
    row * 8 + col
}

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

    fn partition(dirs: &[Dir], mut low: usize, mut high: usize) -> usize {
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

    fn seat(&self) -> (usize, usize) {
        (
            Self::partition(&self.dirs[0..7], 0, 127),
            Self::partition(&self.dirs[7..10], 0, 7),
        )
    }

    fn seat_id(&self) -> usize {
        let (row, col) = self.seat();
        calc_seat_id(row, col)
    }
}

fn build(input: &str) -> Vec<BoardingPass> {
    input.lines().map(BoardingPass::build).collect()
}

fn highest_seat_id(boarding_passes: &[BoardingPass]) -> usize {
    boarding_passes
        .iter()
        .map(BoardingPass::seat_id)
        .max()
        .unwrap()
}

fn my_seat_id(boarding_passes: &[BoardingPass]) -> usize {
    // Build the seating plan.
    let mut seating_plan = vec![false; 8 * 128];
    for p in boarding_passes {
        // Seat ID happens to be exactly how we can index a vector to map a 2-dimension grid.
        seating_plan[p.seat_id()] = true;
    }

    // Find my free seat: It's the free one with occupied on both sides.
    seating_plan
        .windows(3)
        .position(|w| w[0] && !w[1] && w[2])
        .unwrap()
        + 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let boarding_passes = build(&input);

    println!("Part 1: {}", highest_seat_id(&boarding_passes));
    println!("Part 2: {}", my_seat_id(&boarding_passes));
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
