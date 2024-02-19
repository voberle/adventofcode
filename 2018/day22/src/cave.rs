use std::fmt;

use crate::Pos;

enum RegionType {
    Rocky,
    Narrow,
    Wet,
}
use RegionType::{Narrow, Rocky, Wet};

impl RegionType {
    fn new(erosion_level: u32) -> Self {
        match erosion_level % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => panic!("Impossible modulo"),
        }
    }

    fn risk_level(self) -> u32 {
        match self {
            Rocky => 0,
            Wet => 1,
            Narrow => 2,
        }
    }
}

impl fmt::Display for RegionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Rocky => '.',
                Wet => '=',
                Narrow => '|',
            }
        )
    }
}

pub struct Cave {
    depth: u32,
    target: Pos,
    // Erosion levels of all regions
    erosion_levels: Vec<Option<u32>>,
    rows: usize,
    cols: usize,
}

impl Cave {
    pub fn new(input: &str) -> Self {
        let (depth, target) = Self::parse_input(input);
        let cols = target.x + 1;
        let rows = target.y + 1;
        let mut cave = Self {
            depth,
            target,
            erosion_levels: vec![None; cols * rows],
            cols,
            rows,
        };
        cave.fill_erosion_levels();
        cave
    }

    fn parse_input(input: &str) -> (u32, Pos) {
        let lines: Vec<_> = input.lines().collect();
        let depth = lines[0].strip_prefix("depth: ").unwrap().parse().unwrap();
        let target: Vec<_> = lines[1]
            .strip_prefix("target: ")
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        (depth, Pos::new(target[0], target[1]))
    }

    fn pos(&self, pos: &Pos) -> usize {
        pos.y * self.cols + pos.x
    }

    fn get(&self, pos: &Pos) -> Option<u32> {
        self.erosion_levels[self.pos(pos)]
    }

    fn set(&mut self, pos: &Pos, val: u32) {
        let p = self.pos(pos);
        self.erosion_levels[p] = Some(val);
    }

    fn calc_geologic_index(&self, pos: &Pos) -> u32 {
        if pos == &Pos::ZERO || pos == &self.target {
            0
        } else if pos.y == 0 {
            u32::try_from(pos.x).unwrap() * 16807
        } else if pos.x == 0 {
            u32::try_from(pos.y).unwrap() * 48271
        } else {
            self.get(&Pos::new(pos.x - 1, pos.y)).unwrap()
                * self.get(&Pos::new(pos.x, pos.y - 1)).unwrap()
        }
    }

    fn calc_erosion_level(&self, geologic_index: u32) -> u32 {
        (geologic_index + self.depth) % 20183
    }

    fn fill_erosion_levels(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                self.set_erosion_level(x, y);
            }
        }
        // println!("{}", self);
    }

    fn set_erosion_level(&mut self, x: usize, y: usize) {
        let pos = Pos::new(x, y);
        let geologic_index = self.calc_geologic_index(&pos);
        let erosion_level = self.calc_erosion_level(geologic_index);
        self.set(&pos, erosion_level);
    }

    pub fn risk_level(&self) -> u32 {
        self.erosion_levels
            .iter()
            .map(|erosion_level| RegionType::new(erosion_level.unwrap()).risk_level())
            .sum()
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target = self.pos(&Pos::new(self.target.x, self.target.y));
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                write!(
                    f,
                    "{}",
                    if p == 0 {
                        "M".to_string()
                    } else if p == target {
                        "T".to_string()
                    } else if let Some(erosion_level) = self.erosion_levels[p] {
                        RegionType::new(erosion_level).to_string()
                    } else {
                        "?".to_string()
                    }
                )?;
            }
            if row < self.rows - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::INPUT_TEST;

    use super::*;

    #[test]
    fn test_cave_building() {
        const TEST_CAVE: &str = r"M=.|=.|.|=.
.|=|=|||..|
.==|....||=
=.|....|.==
=|..==...=.
=||.=.=||=|
|.=.===|||.
|..==||=.|=
.=..===..=|
.======|||=
.===|=|===T";
        let cave = Cave::new(INPUT_TEST);
        assert_eq!(cave.to_string(), TEST_CAVE);
    }
}
