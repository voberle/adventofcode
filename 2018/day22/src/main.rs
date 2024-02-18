use std::{
    fmt,
    io::{self, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    const ZERO: Pos = Pos { x: 0, y: 0 };

    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone)]
struct Cave {
    depth: u32,
    target: Pos,
}

impl Cave {
    fn build(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let depth = lines[0].strip_prefix("depth: ").unwrap().parse().unwrap();
        let target: Vec<_> = lines[1]
            .strip_prefix("target: ")
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        Self {
            depth,
            target: Pos::new(target[0], target[1]),
        }
    }
}

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

// Map with erosion levels of all regions
struct Map {
    cave: Cave,
    erosion_levels: Vec<Option<u32>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(cave: Cave) -> Self {
        let cols = cave.target.x + 1;
        let rows = cave.target.y + 1;
        Self {
            cave,
            erosion_levels: vec![None; cols * rows],
            cols,
            rows,
        }
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
        if pos == &Pos::ZERO || pos == &self.cave.target {
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
        (geologic_index + self.cave.depth) % 20183
    }

    fn build(cave: Cave) -> Self {
        let mut map = Map::new(cave);

        for y in 0..map.rows {
            for x in 0..map.cols {
                map.set_erosion_level(x, y);
            }
        }
        println!("{}", map);
        map
    }

    fn set_erosion_level(&mut self, x: usize, y: usize) {
        let pos = Pos::new(x, y);
        // println!("Set erosion level for {:?}", pos);
        let geologic_index = self.calc_geologic_index(&pos);
        let erosion_level = self.calc_erosion_level(geologic_index);
        self.set(&pos, erosion_level);
    }

    fn risk_level(&self) -> u32 {
        self.erosion_levels
            .iter()
            .map(|erosion_level| RegionType::new(erosion_level.unwrap()).risk_level())
            .sum()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target = self.pos(&Pos::new(self.cave.target.x, self.cave.target.y));
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

fn risk_level(cave: &Cave) -> u32 {
    let map = Map::build(cave.clone());
    map.risk_level()
}

fn part2(cave: &Cave) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cave = Cave::build(&input);

    println!("Part 1: {}", risk_level(&cave));
    println!("Part 2: {}", part2(&cave));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_map_building() {
        const TEST_MAP: &str = r"M=.|=.|.|=.
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
        let map = Map::build(Cave::build(INPUT_TEST));
        assert_eq!(map.to_string(), TEST_MAP);
    }

    #[test]
    fn test_part1() {
        assert_eq!(risk_level(&Cave::build(INPUT_TEST)), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Cave::build(INPUT_TEST)), 0);
    }
}
