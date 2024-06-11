use std::{
    hash::{Hash, Hasher},
    io::{self, Read},
};

use fxhash::{FxHashMap, FxHasher};

// The debug version
#[cfg(feature = "my_debug")]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(feature = "my_debug"))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {};
}

enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(value: char) -> Self {
        match value {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Invalid direction character"),
        }
    }
}

fn build(input: &str) -> Vec<Jet> {
    input.chars().map(Into::into).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: usize,
    height: usize,
}

impl Pos {
    fn new(x: usize, height: usize) -> Self {
        Self { x, height }
    }

    fn left(&self) -> Self {
        Self::new(self.x - 1, self.height)
    }

    fn right(&self) -> Self {
        Self::new(self.x + 1, self.height)
    }

    fn down(&self) -> Self {
        Self::new(self.x, self.height - 1)
    }
}

const CHAMBER_WIDTH: usize = 7;

struct Chamber {
    units: Vec<[bool; CHAMBER_WIDTH]>,
    height: usize,
}

impl Chamber {
    fn new() -> Self {
        let mut chamber = Self {
            units: Vec::new(),
            height: 0,
        };
        chamber.ensure_size();
        chamber
    }

    fn height(&self) -> usize {
        self.height
    }

    fn is_free(&self, pos: Pos) -> bool {
        if self.units.len() > pos.height {
            !self.units[pos.height][pos.x]
        } else {
            true
        }
    }

    // Makes sure the chamber is high enough.
    fn ensure_size(&mut self) {
        const MIN_SIZE: usize = 7;
        if self.units.len() < self.height + MIN_SIZE {
            self.units
                .resize_with(self.height + MIN_SIZE, || [false; 7]);
        }
    }

    fn set(&mut self, positions: &[Pos]) {
        self.ensure_size();
        for p in positions {
            assert!(!self.units[p.height][p.x]);
            self.units[p.height][p.x] = true;
            self.height = self.height.max(p.height + 1);
        }
        self.ensure_size();
    }

    #[allow(dead_code)]
    fn print_falling(&self, falling_rock: &[Pos]) {
        let mut something_printed = false;
        for (height, line) in self.units.iter().enumerate().rev() {
            let s: String = line
                .iter()
                .enumerate()
                .map(|(x, v)| {
                    if falling_rock.contains(&Pos::new(x, height)) {
                        '@'
                    } else if *v {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            if !something_printed && s == "......." {
                continue;
            }
            println!("|{}|", s);
            something_printed = true;
        }
        println!("+-------+");
    }

    #[cfg(feature = "my_debug")]
    fn debug_print_falling(&self, falling_rock: &[Pos]) {
        self.print_falling(falling_rock);
    }

    #[cfg(not(feature = "my_debug"))]
    #[allow(clippy::unused_self)]
    fn debug_print_falling(&self, _: &[Pos]) {}

    fn debug_print(&self) {
        self.debug_print_falling(&[]);
    }
}

// The position passed to the methods is always the on the left one on the first row.
trait Rock {
    fn units(&self, pos: &Pos) -> Vec<Pos>;
    fn get_initial_position(&self, chamber: &Chamber) -> Pos;
    fn move_left(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos>;
    fn move_right(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos>;
    fn move_down(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos>;
}

// ####
struct HorizontalBar;

impl Rock for HorizontalBar {
    fn units(&self, pos: &Pos) -> Vec<Pos> {
        vec![
            Pos::new(pos.x, pos.height),
            Pos::new(pos.x + 1, pos.height),
            Pos::new(pos.x + 2, pos.height),
            Pos::new(pos.x + 3, pos.height),
        ]
    }

    fn get_initial_position(&self, chamber: &Chamber) -> Pos {
        Pos::new(2, chamber.height() + 3)
    }

    fn move_left(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x > 0 && chamber.is_free(Pos::new(pos.x - 1, pos.height)) {
            Some(pos.left())
        } else {
            None
        }
    }

    fn move_right(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x + 4 < CHAMBER_WIDTH && chamber.is_free(Pos::new(pos.x + 4, pos.height)) {
            Some(pos.right())
        } else {
            None
        }
    }

    fn move_down(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.height > 0
            && (pos.x..pos.x + 4).all(|x| chamber.is_free(Pos::new(x, pos.height - 1)))
        {
            Some(pos.down())
        } else {
            None
        }
    }
}

// .#.
// ###
// .#.
struct Cross;

impl Rock for Cross {
    fn units(&self, pos: &Pos) -> Vec<Pos> {
        vec![
            Pos::new(pos.x, pos.height),
            Pos::new(pos.x - 1, pos.height - 1),
            Pos::new(pos.x, pos.height - 1),
            Pos::new(pos.x + 1, pos.height - 1),
            Pos::new(pos.x, pos.height - 2),
        ]
    }

    fn get_initial_position(&self, chamber: &Chamber) -> Pos {
        Pos::new(2 + 1, chamber.height() + 3 + 2)
    }

    fn move_left(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x > 1
            && chamber.is_free(Pos::new(pos.x - 1, pos.height))
            && chamber.is_free(Pos::new(pos.x - 2, pos.height - 1))
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 2))
        {
            Some(pos.left())
        } else {
            None
        }
    }

    fn move_right(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x + 2 < CHAMBER_WIDTH
            && chamber.is_free(Pos::new(pos.x + 1, pos.height))
            && chamber.is_free(Pos::new(pos.x + 2, pos.height - 1))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 2))
        {
            Some(pos.right())
        } else {
            None
        }
    }

    fn move_down(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.height > 2
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 2))
            && chamber.is_free(Pos::new(pos.x, pos.height - 3))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 2))
        {
            Some(pos.down())
        } else {
            None
        }
    }
}

// ..#
// ..#
// ###
struct RightUp;

impl Rock for RightUp {
    fn units(&self, pos: &Pos) -> Vec<Pos> {
        vec![
            Pos::new(pos.x, pos.height),
            Pos::new(pos.x, pos.height - 1),
            Pos::new(pos.x, pos.height - 2),
            Pos::new(pos.x - 1, pos.height - 2),
            Pos::new(pos.x - 2, pos.height - 2),
        ]
    }

    fn get_initial_position(&self, chamber: &Chamber) -> Pos {
        Pos::new(2 + 2, chamber.height() + 3 + 2)
    }

    fn move_left(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x > 2
            && chamber.is_free(Pos::new(pos.x - 1, pos.height))
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 1))
            && chamber.is_free(Pos::new(pos.x - 3, pos.height - 2))
        {
            Some(pos.left())
        } else {
            None
        }
    }

    fn move_right(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x + 1 < CHAMBER_WIDTH
            && chamber.is_free(Pos::new(pos.x + 1, pos.height))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 1))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 2))
        {
            Some(pos.right())
        } else {
            None
        }
    }

    fn move_down(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.height > 2
            && chamber.is_free(Pos::new(pos.x, pos.height - 3))
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 3))
            && chamber.is_free(Pos::new(pos.x - 2, pos.height - 3))
        {
            Some(pos.down())
        } else {
            None
        }
    }
}

// #
// #
// #
// #
struct VerticalBar;

impl Rock for VerticalBar {
    fn units(&self, pos: &Pos) -> Vec<Pos> {
        vec![
            Pos::new(pos.x, pos.height),
            Pos::new(pos.x, pos.height - 1),
            Pos::new(pos.x, pos.height - 2),
            Pos::new(pos.x, pos.height - 3),
        ]
    }

    fn get_initial_position(&self, chamber: &Chamber) -> Pos {
        Pos::new(2, chamber.height() + 3 + 3)
    }

    fn move_left(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x > 0
            && chamber.is_free(Pos::new(pos.x - 1, pos.height))
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 1))
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 2))
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 3))
        {
            Some(pos.left())
        } else {
            None
        }
    }

    fn move_right(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x + 1 < CHAMBER_WIDTH
            && chamber.is_free(Pos::new(pos.x + 1, pos.height))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 1))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 2))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 3))
        {
            Some(pos.right())
        } else {
            None
        }
    }

    fn move_down(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.height > 3 && chamber.is_free(Pos::new(pos.x, pos.height - 4)) {
            Some(pos.down())
        } else {
            None
        }
    }
}

// ##
// ##
struct Square;

impl Rock for Square {
    fn units(&self, pos: &Pos) -> Vec<Pos> {
        vec![
            Pos::new(pos.x, pos.height),
            Pos::new(pos.x + 1, pos.height),
            Pos::new(pos.x, pos.height - 1),
            Pos::new(pos.x + 1, pos.height - 1),
        ]
    }

    fn get_initial_position(&self, chamber: &Chamber) -> Pos {
        Pos::new(2, chamber.height() + 3 + 1)
    }

    fn move_left(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x > 0
            && chamber.is_free(Pos::new(pos.x - 1, pos.height))
            && chamber.is_free(Pos::new(pos.x - 1, pos.height - 1))
        {
            Some(pos.left())
        } else {
            None
        }
    }

    fn move_right(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.x + 2 < CHAMBER_WIDTH
            && chamber.is_free(Pos::new(pos.x + 2, pos.height))
            && chamber.is_free(Pos::new(pos.x + 2, pos.height - 1))
        {
            Some(pos.right())
        } else {
            None
        }
    }

    fn move_down(&self, chamber: &Chamber, pos: &Pos) -> Option<Pos> {
        if pos.height > 1
            && chamber.is_free(Pos::new(pos.x, pos.height - 2))
            && chamber.is_free(Pos::new(pos.x + 1, pos.height - 2))
        {
            Some(pos.down())
        } else {
            None
        }
    }
}

fn next_rock(i: usize) -> Box<dyn Rock> {
    match i % 5 {
        0 => Box::new(HorizontalBar),
        1 => Box::new(Cross),
        2 => Box::new(RightUp),
        3 => Box::new(VerticalBar),
        4 => Box::new(Square),
        _ => panic!("Bug"),
    }
}

// Calculates a hash for the set of lines.
fn hash_lines(lines: &[[bool; CHAMBER_WIDTH]]) -> u64 {
    let mut hasher = FxHasher::default();
    lines.hash(&mut hasher);
    hasher.finish()
}

fn fall_rocks<const USE_PATTERN_DETECTION: bool>(
    movements: &[Jet],
    total_rocks: usize,
) -> usize {
    let mut chamber = Chamber::new();

    let mut rock_number = 0;
    let mut rock: Box<dyn Rock> = next_rock(rock_number);
    let mut pos = rock.get_initial_position(&chamber);

    // Pattern detection: We try to find a top of the chamber that repeats at the same rock type.
    // - Key: Hash of the top of the chamber.
    // - Value: Rock number and chamber height.
    let mut patterns: FxHashMap<u64, (usize, usize)> = FxHashMap::default();
    let mut height_to_add: Option<usize> = None;

    debug_print!("First rock begins falling");
    chamber.debug_print_falling(&rock.units(&pos));

    for m in movements.iter().cycle() {
        // Pushing rock to the side.
        match m {
            Jet::Left => {
                if let Some(next) = rock.move_left(&chamber, &pos) {
                    debug_print!("Pushing left.");
                    pos = next;
                } else {
                    debug_print!("Pushing left but nothing happens.");
                }
            }
            Jet::Right => {
                if let Some(next) = rock.move_right(&chamber, &pos) {
                    debug_print!("Pushing right.");
                    pos = next;
                } else {
                    debug_print!("Pushing right but nothing happens.");
                }
            }
        }
        chamber.debug_print_falling(&rock.units(&pos));

        // Rock falling down.
        if let Some(next) = rock.move_down(&chamber, &pos) {
            debug_print!("Falling one unit");
            pos = next;
            chamber.debug_print_falling(&rock.units(&pos));
        } else {
            chamber.set(&rock.units(&pos));
            debug_print!("Resting (height {})", chamber.height());
            chamber.debug_print();

            // Pattern detection
            if USE_PATTERN_DETECTION
                && height_to_add.is_none()
                && chamber.height() > 20
                && rock_number % 5 == 0
            {
                let hash = hash_lines(&chamber.units[chamber.height() - 20..=chamber.height()]);
                if let Some((prevous_rock_number, previous_height)) =
                    patterns.insert(hash, (rock_number, chamber.height()))
                {
                    // Found pattern
                    let rock_diff = rock_number - prevous_rock_number;
                    let height_diff = chamber.height() - previous_height;
                    // println!("Found pattern, rock diff={}, height diff={}", rock_diff, height_diff);

                    let period_count = (total_rocks - rock_number) / rock_diff;
                    let rocks_to_jump_ahead = period_count * rock_diff;
                    height_to_add = Some(period_count * height_diff);

                    rock_number += rocks_to_jump_ahead;
                }
            }

            rock_number += 1;
            if rock_number >= total_rocks {
                break;
            }

            debug_print!("Getting new rock (number {})", rock_number);
            rock = next_rock(rock_number);
            pos = rock.get_initial_position(&chamber);
            chamber.debug_print_falling(&rock.units(&pos));
        }
    }

    if !USE_PATTERN_DETECTION {
        // chamber.print_falling(&[]);
        assert_eq!(
            chamber.height(),
            chamber
                .units
                .iter()
                .filter(|line| line.contains(&true))
                .count()
        );
    }

    chamber.height() + height_to_add.unwrap_or_default()
}

fn column_height_after_2022(movements: &[Jet]) -> usize {
    const TOTAL_ROCKS: usize = 2022;
    fall_rocks::<false>(movements, TOTAL_ROCKS)
}

fn column_height_after_trillion(movements: &[Jet]) -> usize {
    const TOTAL_ROCKS: usize = 1_000_000_000_000;
    fall_rocks::<true>(movements, TOTAL_ROCKS)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let movements = build(&input);

    println!("Part 1: {}", column_height_after_2022(&movements));
    println!("Part 2: {}", column_height_after_trillion(&movements));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(column_height_after_2022(&build(INPUT_TEST)), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            column_height_after_trillion(&build(INPUT_TEST)),
            1514285714288
        );
    }
}
