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

impl Object {
    fn elt(&self) -> &Element {
        match self {
            Object::Generator(e) | Object::Microchip(e) => e,
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
                floor.push(Object::Generator(Element::from_name(p)));
            }
            for (_, [p]) in RE_CHIP.captures_iter(line).map(|c| c.extract()) {
                floor.push(Object::Microchip(Element::from_name(p)));
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

#[derive(Debug, Clone, Copy)]
enum ObjectId {
    Generator(usize),
    Microchip(usize),
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
                    Object::Generator(_) => {
                        generators[level][elt_idx] = true;
                    }
                    Object::Microchip(_) => {
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

    fn print(&self, elevator: usize) {
        for f in (0..self.floor_count).rev() {
            println!(
                "F{}: {}  {}",
                f + 1,
                if elevator == f { "E" } else { "." },
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

    // All items on last floor
    fn success(&self) -> bool {
        let last_floor = self.floor_count - 1;
        self.generators[last_floor].iter().all(|&v| v)
            && self.microchips[last_floor].iter().all(|&v| v)
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

    // Finds what the elevator could take from a floor to another.
    fn can_take_to(&self, from: usize, to: usize) -> Vec<Vec<ObjectId>> {
        assert_eq!(from.abs_diff(to), 1);
        // Elevator can have one or two items (cannot be empty).
        let mut movable_objects: Vec<_> = self.microchips[from]
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .filter(|&chip| self.can_chip_go_to(chip, to))
            .map(|i| ObjectId::Microchip(i))
            .collect();
        movable_objects.extend(
            self.generators[from]
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .map(|i| ObjectId::Generator(i))
        );

        let mut elevator_options: Vec<Vec<ObjectId>> = Vec::new();
        elevator_options.extend(movable_objects.iter().map(|o| vec![*o]));
        elevator_options.extend(movable_objects.iter().permutations(2)
        .map(|v| v.iter().map(|e| **e).collect())
        );
        elevator_options
    }

    fn move_objects(&mut self, objects: &[ObjectId], from: usize, to: usize) {
        assert_eq!(from.abs_diff(to), 1);
        assert!(objects.len() == 1 || objects.len() == 2);

        for o in objects {
            match o {
                ObjectId::Generator(i) => {
                    self.generators[from][*i] = false;
                    self.generators[to][*i] = true;
                }
                ObjectId::Microchip(i) => {
                    self.microchips[from][*i] = false;
                    self.microchips[to][*i] = true;
                }
            }
        }
    }
}

fn part1(floors: &[Vec<Object>]) -> i64 {
    let mut building = Building::new(floors);
    let mut elevator = 0;
    building.print(elevator);

    let o = building.can_take_to(0, 1);
    println!("{:?}", o);
    0
}

fn part2(floors: &[Vec<Object>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let floors = build(&input);
    // print_floors(&floors);

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
        building.print(0);

        let h_index = building.elt_idx("H");
        println!("Index of {}: {}", "H", h_index);
        assert!(building.is_chip_on(h_index, 0));
        assert!(building.can_chip_go_to(h_index, 1));
        assert!(!building.can_chip_go_to(h_index, 2));
    }

    #[test]
    fn test_moving() {
        let mut building = Building::new(&build(INPUT_TEST));
        let mut elevator = 0;
        building.print(elevator);

        let o = building.can_take_to(elevator, 1);
        println!("{:?}", o);

        building.move_objects(&o[0], elevator, elevator + 1);
        elevator += 1;
        building.print(elevator);

        let o1 = building.can_take_to(elevator, 2);
        println!("{:?}", o1);
        assert!(false)
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
