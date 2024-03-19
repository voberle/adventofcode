/// Helpers to deal with directions in 2D grids.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::{East, North, South, West};

impl Direction {
    pub fn index(self) -> usize {
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

pub const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                North => 'N',
                East => 'E',
                South => 'S',
                West => 'W',
            }
        )
    }
}
