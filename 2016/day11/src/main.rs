use std::{
    fmt,
    io::{self, Read}, usize,
};

use fxhash::{FxHashSet, FxHashMap};
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

// A better data struct to organize the building than just a list of floors
#[derive(Clone)]
struct Building {
    // List of unique elements in the building. Not needed in algo.
    elements: Vec<Element>,
    floor_count: usize,
    // First index is the floor (first floor is 0).
    // Second index is the element: even indicates the generator, odd the chip
    // TODO change this into a 1-dim vector
    locations: Vec<Vec<bool>>,
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
        let mut locations = vec![vec![false; elements.len() * 2]; floors.len()];
        for (level, floor) in floors.iter().enumerate() {
            for object in floor {
                let elt_idx = elements.iter().position(|e| e == object.elt()).unwrap();
                match object {
                    Object::Generator(_) => {
                        locations[level][elt_idx * 2] = true;
                    }
                    Object::Microchip(_) => {
                        locations[level][elt_idx * 2 + 1] = true;
                    }
                }
            }
        }
        Self {
            floor_count: floors.len(),
            elements,
            locations,
        }
    }

    fn print(&self, elevator: usize) {
        for f in (0..self.floor_count).rev() {
            println!(
                "F{}: {}  {}",
                f + 1,
                if elevator == f { "E" } else { " " },
                self.elements
                    .iter()
                    .enumerate()
                    .map(|(c, e)| {
                        format!(
                            "{:3} {:3}",
                            if self.locations[f][c * 2] {
                                e.to_string() + "G"
                            } else {
                                ".".to_string()
                            },
                            if self.locations[f][c * 2 + 1] {
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
        self.locations[self.floor_count - 1].iter().all(|&v| v)
    }

    fn is_floor_valid(floor: &[bool]) -> bool {
        // If no generators at all, we're good
        if floor.iter().step_by(2).all(|&v| !v) {
            return true;
        }
        for i in (0..floor.len()).step_by(2) {
            // If element has a chip but not corresponding generator, it's bad
            if floor[i+1] && !floor[i] {
                return false;
            }
        }
        return true;
    }

    // Finds what the elevator could take from a floor to another.
    // Elevator can have one or two items (cannot be empty).
    fn can_take_to(&self, from: usize, to: usize) -> Vec<Vec<usize>> {
        assert_eq!(from.abs_diff(to), 1);
        let movable_objects_indexes: Vec<_> = self.locations[from]
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .collect();

        let mut elevator_options: Vec<Vec<usize>> = Vec::new();
        elevator_options.extend(movable_objects_indexes.iter().map(|o| vec![*o]));
        elevator_options.extend(movable_objects_indexes.iter().combinations(2)
        .map(|v| v.iter().map(|e| **e).collect())
        );

        elevator_options.iter().filter(|&option| {
            let mut from_floor_copy = self.locations[from].clone();
            let mut to_floor_copy = self.locations[to].clone();
            for i in option {
                from_floor_copy[*i] = false;
                to_floor_copy[*i] = true;
            }
            Self::is_floor_valid(&from_floor_copy) && Self::is_floor_valid(&to_floor_copy)

        }).cloned()
        .collect()
    }

    fn can_take_up(&self, elevator: usize) -> Vec<Vec<usize>> {
        self.can_take_to(elevator, elevator + 1)
    }

    fn can_take_down(&self, elevator: usize) -> Vec<Vec<usize>> {
        self.can_take_to(elevator, elevator - 1)
    }

    fn move_objects(&mut self, objects: &[usize], from: usize, to: usize) {
        assert_eq!(from.abs_diff(to), 1);
        assert!(objects.len() == 1 || objects.len() == 2);

        for i in objects {
            self.locations[from][*i] = false;
            self.locations[to][*i] = true;
        }
    }

    fn move_up(&mut self, objects: &[usize], elevator: usize) {
        self.move_objects(objects, elevator, elevator + 1);
    }

    fn move_down(&mut self, objects: &[usize], elevator: usize) {
        self.move_objects(objects, elevator, elevator - 1);
    }
}

// Recursive function
fn find_steps(
    building: &mut Building,
    current_floor: usize,
    origin: (i8, Vec<usize>),  // what got us there, to avoid going back
    steps: usize,
    min_steps_found: &mut usize,
    done: &mut FxHashMap<(usize, Vec<Vec<bool>>), usize>
) {
    // println!("{} steps:", steps);
    // building.print(current_floor);

    if steps > 100 {
        return;
    }
    if let Some(s) = done.get(&(current_floor, building.locations.clone())) {
        if steps >= *s {
            // println!("Already been here");
            return;
        }
    }
    done.insert((current_floor, building.locations.clone()), steps);

    if steps > *min_steps_found {
        return;
    }

    if current_floor < building.floor_count - 1 {
        // Can go up
        let mut up_options = building.can_take_up(current_floor);

        // We also want to exclude what we just did
        if origin.0 == -1 {
            if let Some(index) = up_options.iter().position(|o| *o == origin.1) {
                up_options.remove(index);
            }
        }

        // println!("UP options: {:?}", up_options);
        for objects in up_options {
            // println!("Up: {:?}", objects);
            let mut new_building = building.clone();
            new_building.move_up(&objects, current_floor);

            if new_building.success() {
                *min_steps_found = (steps + 1).min(*min_steps_found);
                println!("Found one: {}", *min_steps_found);
                // new_building.print(current_floor + 1);
                // We can return, even if another option works, it would be the same step count
                return;
            }
            find_steps(&mut new_building.clone(), current_floor + 1, (1, objects.clone()), steps + 1, min_steps_found, done);
        }
    }
    
    if current_floor > 0 {
        // Can go down
        let mut down_options = building.can_take_down(current_floor);

        // We also want to exclude what we just did
        if origin.0 == 1 {
            if let Some(index) = down_options.iter().position(|o| *o == origin.1) {
                down_options.remove(index);
            }
        }

        // println!("DOWN options: {:?}", down_options);
        for objects in down_options {
            // println!("Down: {:?}", objects);
            let mut new_building = building.clone();
            new_building.move_down(&objects, current_floor);

            if new_building.success() {
                *min_steps_found = (steps + 1).min(*min_steps_found);
                println!("Found one: {}", *min_steps_found);
                // new_building.print(current_floor + 1);
                // We can return, even if another option works, it would be the same step count
                return;
            }
            find_steps(&mut new_building, current_floor - 1, (-1, objects.clone()), steps + 1, min_steps_found, done);
        }
    }


}


fn min_number_steps(floors: &[Vec<Object>]) -> usize {
    let mut building = Building::new(floors);
    building.print(0);

    let mut min_steps_found = usize::MAX;
    let mut done: FxHashMap<(usize, Vec<Vec<bool>>), usize> = FxHashMap::default();
    find_steps(&mut building, 0, (0, Vec::new()), 0, &mut min_steps_found, &mut done);
    min_steps_found
}

fn part2(floors: &[Vec<Object>]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let floors = build(&input);
    // print_floors(&floors);

    println!("Part 1: {}", min_number_steps(&floors));
    println!("Part 2: {}", part2(&floors));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_is_floor_valid() {
        // Empty
        assert!(Building::is_floor_valid(&[false, false, false, false]));
        // Only one microchip
        assert!(Building::is_floor_valid(&[false, false, false, true]));
        // Only one generator
        assert!(Building::is_floor_valid(&[false, false, true, false]));
        // Microchip + corresponding generator
        assert!(Building::is_floor_valid(&[false, false, true, true]));
        // Microchip + corresponding generator + other generator
        assert!(Building::is_floor_valid(&[true, false, true, true]));
        assert!(Building::is_floor_valid(&[true, true, true, false]));
        // Two generators, no microchip
        assert!(Building::is_floor_valid(&[true, false, true, false]));
        // Full
        assert!(Building::is_floor_valid(&[true, true, true, true]));

        // Invalid:
        // Microchip + other generator
        assert!(!Building::is_floor_valid(&[true, false, false, true]));
    }

    #[test]
    fn test_is_success() {
        let building = Building { elements: Vec::new(), floor_count: 4, locations: vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![true, true, true, true],
        ] };
        assert!(building.success());
    }

    #[test]
    fn test_can_take_to() {
        let building = Building { elements: Vec::new(), floor_count: 4, locations: vec![
            vec![false, false, false, true],
            vec![false, false, false, false],
            vec![false, true, false, false],
            vec![true, false, true, false],
        ] };
        let options = building.can_take_down(3);
        assert_eq!(options, vec![vec![0], vec![0, 2]]);
    }

    fn up(building: &mut Building, current_floor: &mut usize, steps: &mut usize, objects: Vec<usize>) {
        *steps += 1;
        println!("Step {}", *steps);
        let options = building.can_take_up(*current_floor);
        assert!(options.contains(&objects));
        building.move_up(&objects, *current_floor);
        *current_floor += 1;
        building.print(*current_floor);
    }

    fn down(building: &mut Building, current_floor: &mut usize, steps: &mut usize, objects: Vec<usize>) {
        *steps += 1;
        println!("Step {}", *steps);
        let options = building.can_take_down(*current_floor);
        assert!(options.contains(&objects));
        building.move_down(&objects, *current_floor);
        *current_floor -= 1;
        building.print(*current_floor);
    }

    #[test]
    fn test_moving() {
        let mut building = Building::new(&build(INPUT_TEST));
        let mut current_floor = 0;
        building.print(current_floor);

        let mut steps = 0;
        up(&mut building, &mut current_floor, &mut steps, vec![1]);
        up(&mut building, &mut current_floor, &mut steps, vec![0, 1]);
        down(&mut building, &mut current_floor, &mut steps, vec![1]);
        down(&mut building, &mut current_floor, &mut steps, vec![1]);
        up(&mut building, &mut current_floor, &mut steps, vec![1, 3]);
        up(&mut building, &mut current_floor, &mut steps, vec![1, 3]);
        up(&mut building, &mut current_floor, &mut steps, vec![1, 3]);
        down(&mut building, &mut current_floor, &mut steps, vec![1]);
        up(&mut building, &mut current_floor, &mut steps, vec![0, 2]);
        down(&mut building, &mut current_floor, &mut steps, vec![3]);
        up(&mut building, &mut current_floor, &mut steps, vec![1, 3]);

        assert_eq!(steps, 11);
    }

    #[test]
    fn test_part1() {
        assert_eq!(min_number_steps(&build(INPUT_TEST)), 11);
    }
}
