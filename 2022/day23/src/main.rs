use std::{
    collections::VecDeque,
    fmt::Display,
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
    E,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn go(self, dir: Dir) -> Self {
        match dir {
            Dir::N => Pos::new(self.x, self.y - 1),
            Dir::NW => Pos::new(self.x - 1, self.y - 1),
            Dir::NE => Pos::new(self.x + 1, self.y - 1),
            Dir::S => Pos::new(self.x, self.y + 1),
            Dir::SW => Pos::new(self.x - 1, self.y + 1),
            Dir::SE => Pos::new(self.x + 1, self.y + 1),
            Dir::W => Pos::new(self.x - 1, self.y),
            Dir::E => Pos::new(self.x + 1, self.y),
        }
    }
}

type Directions = VecDeque<([Dir; 3], Dir)>;

#[derive(Debug, Clone)]
struct Groove(FxHashSet<Pos>);

impl From<&str> for Groove {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, e)| {
                        if e == '#' {
                            Some(Pos::new(
                                i32::try_from(x).unwrap(),
                                i32::try_from(y).unwrap(),
                            ))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        )
    }
}

impl Display for Groove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_pos, max_pos) = self.borders();
        for y in min_pos.y..=max_pos.y {
            for x in min_pos.x..=max_pos.x {
                let pos = Pos::new(x, y);
                if self.0.contains(&pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            if y < max_pos.y {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Groove {
    fn new() -> Self {
        Self(FxHashSet::default())
    }

    fn borders(&self) -> (Pos, Pos) {
        let mut min_pos = Pos::new(i32::MAX, i32::MAX);
        let mut max_pos = Pos::new(i32::MIN, i32::MIN);
        for pos in &self.0 {
            min_pos.x = min_pos.x.min(pos.x);
            max_pos.x = max_pos.x.max(pos.x);
            min_pos.y = min_pos.y.min(pos.y);
            max_pos.y = max_pos.y.max(pos.y);
        }
        (min_pos, max_pos)
    }

    fn is_elf(&self, pos: Pos, dir: Dir) -> bool {
        self.0.contains(&pos.go(dir))
    }

    fn is_none_around(&self, pos: Pos) -> bool {
        [
            Dir::N,
            Dir::NE,
            Dir::NW,
            Dir::S,
            Dir::SE,
            Dir::SW,
            Dir::E,
            Dir::W,
        ]
        .iter()
        .all(|d| !self.is_elf(pos, *d))
    }

    fn consider(&self, pos: Pos, dirs: &[Dir], target_dir: Dir) -> Option<Pos> {
        if dirs.iter().all(|d| !self.is_elf(pos, *d)) {
            Some(pos.go(target_dir))
        } else {
            None
        }
    }

    fn get_move_proposal(&self, pos: Pos, directions: &Directions) -> Option<Pos> {
        for direction in directions {
            if let Some(p) = self.consider(pos, &direction.0, direction.1) {
                return Some(p);
            }
        }
        None
    }

    fn get_all_move_proposals(
        &self,
        directions: &Directions,
    ) -> (Vec<Pos>, FxHashMap<Pos, Vec<Pos>>) {
        let mut not_moving: Vec<Pos> = Vec::new();
        // Key is target position.
        // Value is list of source positions.
        let mut proposals: FxHashMap<Pos, Vec<Pos>> = FxHashMap::default();

        for pos in &self.0 {
            if self.is_none_around(*pos) {
                not_moving.push(*pos);
            } else if let Some(p) = self.get_move_proposal(*pos, directions) {
                proposals
                    .entry(p)
                    .and_modify(|list| list.push(*pos))
                    .or_insert(vec![*pos]);
            } else {
                not_moving.push(*pos);
            }
        }
        (not_moving, proposals)
    }

    fn apply_moves(not_moving: &[Pos], proposals: &FxHashMap<Pos, Vec<Pos>>) -> Self {
        let mut new_groove = Groove::new();
        new_groove.0.extend(not_moving);

        for (target, from) in proposals {
            assert!(!from.is_empty());
            if from.len() == 1 {
                new_groove.0.insert(*target);
            } else {
                new_groove.0.extend(from);
            }
        }
        new_groove
    }

    fn apply_round(&self, directions: &mut Directions) -> Self {
        let (not_moving, proposals) = self.get_all_move_proposals(directions);
        let new_groove = Self::apply_moves(&not_moving, &proposals);
        directions.rotate_left(1);
        new_groove
    }

    fn count_empty_grounds(&self) -> usize {
        let (min, max) = self.borders();
        let mut count = 0;
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                if !self.0.contains(&Pos::new(x, y)) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn exec_rounds(groove: &Groove, rounds: usize) -> Groove {
    let mut groove = groove.clone();

    let mut directions: Directions = VecDeque::from([
        ([Dir::N, Dir::NE, Dir::NW], Dir::N),
        ([Dir::S, Dir::SE, Dir::SW], Dir::S),
        ([Dir::W, Dir::NW, Dir::SW], Dir::W),
        ([Dir::E, Dir::NE, Dir::SE], Dir::E),
    ]);

    // println!("== Initial State ==");
    // println!("{}", groove);

    for _round in 1..=rounds {
        groove = groove.apply_round(&mut directions);

        // println!("== End of Round {} ==", _round);
        // println!("{}", groove);
    }
    groove
}

fn empty_grounds_after_10(groove: &Groove) -> usize {
    const ROUNDS: usize = 10;
    let groove = exec_rounds(groove, ROUNDS);
    groove.count_empty_grounds()
}

fn part2(groove: &Groove) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let groove: Groove = input.as_str().into();

    println!("Part 1: {}", empty_grounds_after_10(&groove));
    println!("Part 2: {}", part2(&groove));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST_0: &str = include_str!("../resources/input_test_0");
    const INPUT_TEST_1: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_simple_example() {
        let groove: Groove = INPUT_TEST_0.into();
        let new_groove = exec_rounds(&groove, 3);
        assert_eq!(
            new_groove.to_string(),
            "..#..
....#
#....
....#
.....
..#.."
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(empty_grounds_after_10(&INPUT_TEST_1.into()), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT_TEST_1.into()), 0);
    }
}
