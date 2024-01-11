use std::{
    fmt,
    io::{self, Read},
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// Field is the symbol of the element ("H" for hydrogen, etc)
#[derive(Clone, PartialEq, Eq, Hash)]
struct Element(String);

impl Element {
    // For pretty visualizations
    const SYMBOLS: [(&'static str, &'static str); 7] = [
        ("polonium", "Po"),
        ("thulium", "Tm"),
        ("promethium", "Pm"),
        ("ruthenium", "Ru"),
        ("cobalt", "Co"),
        ("hydrogen", "H"),
        ("lithium", "Li"),
    ];

    fn new(symbol: &str) -> Self {
        Self(symbol.to_string())
    }

    fn from_name(name: &str) -> Self {
        let symbol = Self::SYMBOLS
            .iter()
            .find(|(n, _)| *n == name)
            .expect("Symbol not found")
            .1;
        Self(symbol.to_string())
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum Object {
    Generator(Element),
    Microchip(Element),
}
use Object::{Generator, Microchip};

impl Object {
    fn elt(&self) -> &Element {
        match self {
            Generator(e) | Microchip(e) => e,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generator(e) => write!(f, "{}G", e),
            Self::Microchip(e) => write!(f, "{}M", e),
        }
    }
}

// First floor is index 0
fn build(input: &str) -> Vec<Vec<Object>> {
    lazy_static! {
        static ref RE_GEN: Regex = Regex::new(r"\W(\w+) generator").unwrap();
        static ref RE_CHIP: Regex = Regex::new(r"\W(\w+)-compatible microchip").unwrap();
    }
    input
        .lines()
        .map(|line| {
            let mut floor = Vec::new();
            for (_, [p]) in RE_GEN.captures_iter(line).map(|c| c.extract()) {
                floor.push(Generator(Element::from_name(p)));
            }
            for (_, [p]) in RE_CHIP.captures_iter(line).map(|c| c.extract()) {
                floor.push(Microchip(Element::from_name(p)));
            }
            floor
        })
        .collect()
}

fn print_floors(floors: &[Vec<Object>]) {
    for (level, floor) in floors.iter().rev().enumerate() {
        println!(
            "F{}: {}",
            floors.len() - level,
            floor.iter().map(ToString::to_string).join(" ")
        )
    }
}

// A better data struct to organize the building than just a list of floors
struct Building {
    // List of unique elements in the building. Not needed in algo.
    elements: Vec<Element>,
    elements_count: usize,
    floor_count: usize,
    // First index is the floor (first floor is 0).
    // Second index is the element.
    generators: Vec<Vec<bool>>,
    microchips: Vec<Vec<bool>>,
}

impl Building {
    fn new(floors: &[Vec<Object>]) -> Self {
        // List of unique elements in the building
        let elements: Vec<Element> = floors
            .iter()
            .flatten()
            .map(Object::elt)
            .unique()
            .cloned()
            .collect();
        let mut generators = vec![vec![false; elements.len()]; floors.len()];
        let mut microchips = vec![vec![false; elements.len()]; floors.len()];
        for (level, floor) in floors.iter().enumerate() {
            for object in floor {
                let elt_idx = elements.iter().position(|e| e == object.elt()).unwrap();
                match object {
                    Generator(_) => {
                        generators[level][elt_idx] = true;
                    }
                    Microchip(_) => {
                        microchips[level][elt_idx] = true;
                    }
                }
            }
        }
        Self {
            elements_count: elements.len(),
            floor_count: floors.len(),
            elements,
            generators,
            microchips,
        }
    }

    // Should only be needed for testing.
    fn elt_idx(&self, symbol: &str) -> usize {
        self.elements.iter().position(|e| e.0 == symbol).unwrap()
    }

    fn print(&self) {
        for f in (0..self.floor_count).rev() {
            println!(
                "F{}: {}",
                f + 1,
                self.elements
                    .iter()
                    .enumerate()
                    .map(|(c, e)| {
                        format!(
                            "{:3} {:3}",
                            if self.generators[f][c] {
                                e.to_string() + "G"
                            } else {
                                ".".to_string()
                            },
                            if self.microchips[f][c] {
                                e.to_string() + "M"
                            } else {
                                ".".to_string()
                            }
                        )
                    })
                    .join(" ")
            )
        }
    }

    fn any_generator_on_floor(&self, floor: usize) -> bool {
        self.generators[floor].iter().any(|&v| v)
    }

    fn is_chip_on(&self, chip: usize, floor: usize) -> bool {
        self.microchips[floor][chip]
    }

    fn is_generator_on(&self, chip: usize, floor: usize) -> bool {
        self.generators[floor][chip]
    }

    // Rules
    // Chip cannot be on a floor with Generators without having its own Generator as well
    fn can_chip_go_to(&self, chip: usize, floor: usize) -> bool {
        // Either no generators on floor, or our own is there.
        !self.any_generator_on_floor(floor) || self.is_generator_on(chip, floor)
    }
}

fn part1(floors: &[Vec<Object>]) -> i64 {
    0
}

fn part2(floors: &[Vec<Object>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let floors = build(&input);
    print_floors(&floors);

    println!("Part 1: {}", part1(&floors));
    println!("Part 2: {}", part2(&floors));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_building() {
        let building = Building::new(&build(INPUT_TEST));
        building.print();
        println!("{:?}", building.microchips);

        let h_index = building.elt_idx("H");
        println!("Index of {}: {}", "H", h_index);
        assert!(building.is_chip_on(h_index, 0));
        assert!(building.can_chip_go_to(h_index, 1));
        assert!(!building.can_chip_go_to(h_index, 2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&build(INPUT_TEST)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
