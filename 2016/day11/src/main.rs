use std::{
    fmt,
    io::{self, Read},
};

use fxhash::FxHashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// Field is the symbol of the element ("H" for hydrogen, etc)
#[derive(Clone, PartialEq, Eq, Hash)]
struct Element(String);

impl Element {
    // For pretty visualizations
    const SYMBOLS: [(&'static str, &'static str); 9] = [
        ("polonium", "Po"),
        ("thulium", "Tm"),
        ("promethium", "Pm"),
        ("ruthenium", "Ru"),
        ("cobalt", "Co"),
        ("hydrogen", "H"),
        ("lithium", "Li"),
        // For part 2, fictional elements
        ("dilithium", "Dt"),
        ("elerium", "El"),
    ];

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
            Self::Generator(e) => write!(f, "{e}G"),
            Self::Microchip(e) => write!(f, "{e}M"),
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

// A better data struct to organize the building than just a list of floors
#[derive(Clone)]
struct Building {
    // List of unique elements in the building. Not needed in algo.
    #[allow(dead_code)]
    elements: Vec<Element>,
    // First index is the floor (first floor is 0).
    // Second index is the element: even indicates the generator, odd the chip
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
            elements,
            locations,
        }
    }

    fn floor_count(&self) -> usize {
        self.locations.len()
    }

    #[allow(dead_code)]
    fn print(&self, elevator: usize) {
        for f in (0..self.floor_count()).rev() {
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
            );
        }
    }

    // All items are on last floor
    fn success(&self) -> bool {
        self.locations[self.floor_count() - 1].iter().all(|&v| v)
    }

    // Checks if the floors below this one are all empty.
    // If so, we don't want to explore those anymore, as there is no point.
    fn all_below_empty(&self, floor: usize) -> bool {
        (0..floor).all(|i| self.locations[i].iter().all(|&v| !v))
    }

    fn is_floor_valid(floor: &[bool]) -> bool {
        // If no generators at all, we're good
        if floor.iter().step_by(2).all(|&v| !v) {
            return true;
        }
        for i in (0..floor.len()).step_by(2) {
            // If element has a chip but not corresponding generator, it's bad
            if floor[i + 1] && !floor[i] {
                return false;
            }
        }
        true
    }

    // Finds what the elevator could take from a floor to another.
    // Elevator can have one or two items (cannot be empty).
    fn can_take_to<const INCLUDE_SINGLETONS: bool, const INCLUDE_PAIRS: bool>(
        &self,
        from: usize,
        to: usize,
    ) -> Vec<Vec<usize>> {
        assert_eq!(from.abs_diff(to), 1);
        let movable_objects_indexes: Vec<_> = self.locations[from]
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .collect();

        let mut elevator_options: Vec<Vec<usize>> = Vec::new();
        if INCLUDE_SINGLETONS {
            elevator_options.extend(movable_objects_indexes.iter().map(|o| vec![*o]));
        }
        if INCLUDE_PAIRS {
            elevator_options.extend(
                movable_objects_indexes
                    .iter()
                    .combinations(2)
                    .map(|v| v.iter().map(|e| **e).collect()),
            );
        }

        // Filter toi exclude all the options that are not allowed.
        elevator_options
            .iter()
            .filter(|&option| {
                let mut from_floor_copy = self.locations[from].clone();
                let mut to_floor_copy = self.locations[to].clone();
                for i in option {
                    from_floor_copy[*i] = false;
                    to_floor_copy[*i] = true;
                }
                Self::is_floor_valid(&from_floor_copy) && Self::is_floor_valid(&to_floor_copy)
            })
            .cloned()
            .collect()
    }

    fn can_take_up<const REAL_INPUT: bool>(&self, elevator: usize) -> Vec<Vec<usize>> {
        if REAL_INPUT {
            // For the real input, we observed that the best path never takes a single item up,
            // always only pairs, so we don't even bother looking at singletons.
            self.can_take_to::<false, true>(elevator, elevator + 1)
        } else {
            self.can_take_to::<true, true>(elevator, elevator + 1)
        }
    }

    fn can_take_down<const REAL_INPUT: bool>(&self, elevator: usize) -> Vec<Vec<usize>> {
        if REAL_INPUT {
            // For the real input, we never need to take pairs down, only singletons.
            self.can_take_to::<true, false>(elevator, elevator - 1)
        } else {
            self.can_take_to::<true, true>(elevator, elevator - 1)
        }
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
fn find_steps<const REAL_INPUT: bool>(
    building: &mut Building,
    current_floor: usize,
    steps: usize,
    min_steps_found: &mut usize,
    done: &mut FxHashMap<(usize, Vec<Vec<bool>>), usize>,
    // best_path: &mut Vec<(String, Vec<usize>)>,
) {
    if let Some(s) = done.get(&(current_floor, building.locations.clone())) {
        if steps >= *s {
            return;
        }
    }
    done.insert((current_floor, building.locations.clone()), steps);

    if steps > *min_steps_found {
        return;
    }

    if current_floor < building.floor_count() - 1 {
        // Can go up
        let up_options = building.can_take_up::<REAL_INPUT>(current_floor);
        for objects in up_options {
            let mut new_building = building.clone();
            new_building.move_up(&objects, current_floor);

            // let mut new_best_path = best_path.clone();
            // new_best_path.push(("up".to_string(), objects.clone()));

            if new_building.success() {
                *min_steps_found = (steps + 1).min(*min_steps_found);
                // println!("Found one: {}. Best path: {:?}", *min_steps_found, best_path);

                // We can return, even if another option works, it would be the same step count
                return;
            }
            find_steps::<REAL_INPUT>(
                &mut new_building,
                current_floor + 1,
                steps + 1,
                min_steps_found,
                done,
                // &mut new_best_path,
            );
        }
    }

    // If below is all empty, don't attempt to go down anymore.
    if current_floor > 0 && !building.all_below_empty(current_floor) {
        // Can go down
        let down_options = building.can_take_down::<REAL_INPUT>(current_floor);
        for objects in down_options {
            let mut new_building = building.clone();
            new_building.move_down(&objects, current_floor);

            // let mut new_best_path = best_path.clone();
            // new_best_path.push(("down".to_string(), objects.clone()));

            if new_building.success() {
                *min_steps_found = (steps + 1).min(*min_steps_found);
                // println!("Found one: {}. Best path: {:?}", *min_steps_found, best_path);
                return;
            }
            find_steps::<REAL_INPUT>(
                &mut new_building,
                current_floor - 1,
                steps + 1,
                min_steps_found,
                done,
                // &mut new_best_path,
            );
        }
    }
}

fn min_number_steps<const REAL_INPUT: bool>(floors: &[Vec<Object>]) -> usize {
    let mut building = Building::new(floors);
    // building.print(0);

    let mut min_steps_found = usize::MAX;
    let mut done: FxHashMap<(usize, Vec<Vec<bool>>), usize> = FxHashMap::default();
    // let mut best_path: Vec<(String, Vec<usize>)> = Vec::new();

    find_steps::<REAL_INPUT>(
        &mut building,
        0,
        0,
        &mut min_steps_found,
        &mut done,
        // &mut best_path,
    );
    min_steps_found
}

fn add_extra_elements(floors: &mut [Vec<Object>]) {
    let floor = floors.get_mut(0).unwrap();
    floor.push(Object::Generator(Element::from_name("elerium")));
    floor.push(Object::Microchip(Element::from_name("elerium")));
    floor.push(Object::Generator(Element::from_name("dilithium")));
    floor.push(Object::Microchip(Element::from_name("dilithium")));
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut floors = build(&input);

    println!("Part 1: {}", min_number_steps::<true>(&floors));

    add_extra_elements(&mut floors);
    println!("Part 2: {}", min_number_steps::<true>(&floors));
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
        let building = Building {
            elements: Vec::new(),
            locations: vec![
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, true, true, true],
            ],
        };
        assert!(building.success());
    }

    #[test]
    fn test_can_take_to() {
        let building = Building {
            elements: Vec::new(),
            locations: vec![
                vec![false, false, false, true],
                vec![false, false, false, false],
                vec![false, true, false, false],
                vec![true, false, true, false],
            ],
        };
        let options = building.can_take_down::<false>(3);
        assert_eq!(options, vec![vec![0], vec![0, 2]]);
    }

    fn up(
        building: &mut Building,
        current_floor: &mut usize,
        steps: &mut usize,
        objects: &[usize],
    ) {
        *steps += 1;
        println!("Step {}", *steps);
        let options = building.can_take_up::<false>(*current_floor);
        assert!(options.contains(&objects.to_vec()));
        building.move_up(objects, *current_floor);
        *current_floor += 1;
        building.print(*current_floor);
    }

    fn down(
        building: &mut Building,
        current_floor: &mut usize,
        steps: &mut usize,
        objects: &[usize],
    ) {
        *steps += 1;
        println!("Step {}", *steps);
        let options = building.can_take_down::<false>(*current_floor);
        assert!(options.contains(&objects.to_vec()));
        building.move_down(objects, *current_floor);
        *current_floor -= 1;
        building.print(*current_floor);
    }

    #[test]
    fn test_moving() {
        let mut building = Building::new(&build(INPUT_TEST));
        let mut current_floor = 0;
        building.print(current_floor);

        let mut steps = 0;
        up(&mut building, &mut current_floor, &mut steps, &[1]);
        assert!(!building.all_below_empty(current_floor));
        up(&mut building, &mut current_floor, &mut steps, &[0, 1]);
        assert!(!building.all_below_empty(current_floor));
        down(&mut building, &mut current_floor, &mut steps, &[1]);
        down(&mut building, &mut current_floor, &mut steps, &[1]);
        up(&mut building, &mut current_floor, &mut steps, &[1, 3]);
        assert!(building.all_below_empty(current_floor));
        up(&mut building, &mut current_floor, &mut steps, &[1, 3]);
        assert!(building.all_below_empty(current_floor));
        up(&mut building, &mut current_floor, &mut steps, &[1, 3]);
        down(&mut building, &mut current_floor, &mut steps, &[1]);
        up(&mut building, &mut current_floor, &mut steps, &[0, 2]);
        down(&mut building, &mut current_floor, &mut steps, &[3]);
        up(&mut building, &mut current_floor, &mut steps, &[1, 3]);

        assert!(building.success());
        assert_eq!(steps, 11);
    }

    #[test]
    fn test_part1() {
        assert_eq!(min_number_steps::<false>(&build(INPUT_TEST)), 11);
    }
}
