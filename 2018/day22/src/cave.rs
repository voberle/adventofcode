use fxhash::FxHashMap;
use std::{cell::RefCell, fmt};

use crate::Pos;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegionType {
    Rocky(u32),
    Narrow(u32),
    Wet(u32),
}
use RegionType::{Narrow, Rocky, Wet};

impl RegionType {
    fn new(erosion_level: u32) -> Self {
        match erosion_level % 3 {
            0 => Rocky(erosion_level),
            1 => Wet(erosion_level),
            2 => Narrow(erosion_level),
            _ => panic!("Impossible modulo"),
        }
    }

    fn get_erosion_level(self) -> u32 {
        match self {
            Rocky(e) | Wet(e) | Narrow(e) => e,
        }
    }

    fn risk_level(self) -> u32 {
        match self {
            Rocky(_) => 0,
            Wet(_) => 1,
            Narrow(_) => 2,
        }
    }
}

impl fmt::Display for RegionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Rocky(_) => '.',
                Wet(_) => '=',
                Narrow(_) => '|',
            }
        )
    }
}

pub struct Cave {
    depth: u32,
    target: Pos,
    // Regions and erosion levels of all regions.
    // Wrapping it in a RefCell, as we will need to mutate it when auto-growing the map.
    regions: RefCell<FxHashMap<Pos, RegionType>>,
}

impl Cave {
    pub fn new(input: &str) -> Self {
        let (depth, target) = Self::parse_input(input);
        let mut cave = Self {
            depth,
            target,
            regions: RefCell::new(FxHashMap::default()),
        };
        cave.fill_regions(target.y + 1, target.x + 1);
        cave
    }

    pub fn get_target(&self) -> Pos {
        self.target
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

    fn get(&self, pos: &Pos) -> Option<RegionType> {
        self.regions.borrow().get(pos).copied()
    }

    fn set(&self, pos: &Pos, val: RegionType) {
        self.regions.borrow_mut().insert(*pos, val);
    }

    // Returns the geologic index for the position, recursively filling the map if needed.
    fn calc_geologic_index(&self, pos: &Pos) -> u32 {
        if pos == &Pos::ZERO || pos == &self.target {
            0
        } else if pos.y == 0 {
            u32::try_from(pos.x).unwrap() * 16807
        } else if pos.x == 0 {
            u32::try_from(pos.y).unwrap() * 48271
        } else {
            let v1 = self
                .get_region(&Pos::new(pos.x - 1, pos.y))
                .get_erosion_level();
            let v2 = self
                .get_region(&Pos::new(pos.x, pos.y - 1))
                .get_erosion_level();
            v1 * v2
        }
    }

    fn calc_erosion_level(&self, geologic_index: u32) -> u32 {
        (geologic_index + self.depth) % 20183
    }

    // Sets the erosion level for the position, recursively filling the map if needed.
    fn set_erosion_level(&self, pos: &Pos) {
        let geologic_index = self.calc_geologic_index(pos);
        let erosion_level = self.calc_erosion_level(geologic_index);
        self.set(pos, RegionType::new(erosion_level));
    }

    // Fill regions up to the target position.
    // If the map is empty, this is more efficient than calling just `set_erosion_level(target)`.
    fn fill_regions(&mut self, rows: usize, cols: usize) {
        for y in 0..rows {
            for x in 0..cols {
                let pos = Pos::new(x, y);
                self.set_erosion_level(&pos);
            }
        }
    }

    // Returns the position's region.
    // Fills the map if needed.
    pub fn get_region(&self, pos: &Pos) -> RegionType {
        if let Some(region) = self.get(pos) {
            region
        } else {
            self.set_erosion_level(pos);
            self.get(pos).unwrap()
        }
    }

    pub fn risk_level(&self) -> u32 {
        self.regions
            .borrow()
            .iter()
            .filter_map(|(pos, region)| {
                if pos.x <= self.target.x && pos.y <= self.target.y {
                    Some(region.risk_level())
                } else {
                    None
                }
            })
            .sum()
    }
}

// Gets a string with the cave until the target.
impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let pos = Pos::new(x, y);
                write!(
                    f,
                    "{}",
                    if pos == Pos::ZERO {
                        "M".to_string()
                    } else if pos == self.target {
                        "T".to_string()
                    } else if let Some(region) = self.get(&pos) {
                        region.to_string()
                    } else {
                        "?".to_string()
                    }
                )?;
            }
            if y < self.target.y {
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

    #[test]
    fn test_map_growing() {
        let cave = Cave::new(INPUT_TEST);
        assert_eq!(cave.get_region(&Pos::new(15, 15)), RegionType::Narrow(3002));
    }
}
