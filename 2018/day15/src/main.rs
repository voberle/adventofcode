use std::io::{self, Read};

use itertools::Itertools;

mod grid;
use grid::{find_shortest_path, Grid};

mod visualization;

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitType {
    Elf,
    Goblin,
}

impl UnitType {
    fn new(c: char) -> Self {
        match c {
            'E' => Self::Elf,
            'G' => Self::Goblin,
            _ => panic!("Invalid unit char: {c}"),
        }
    }

    fn is(self, c: char) -> bool {
        match self {
            UnitType::Elf => c == 'E',
            UnitType::Goblin => c == 'G',
        }
    }

    fn opponent(self) -> Self {
        match self {
            UnitType::Elf => UnitType::Goblin,
            UnitType::Goblin => UnitType::Elf,
        }
    }

    fn name(self) -> char {
        match self {
            UnitType::Elf => 'E',
            UnitType::Goblin => 'G',
        }
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::struct_field_names)]
struct Unit {
    unit_type: UnitType,
    position: usize,
    attack_power: i32,
    hit_points: i32,
}

impl Unit {
    fn new(c: char, position: usize) -> Self {
        Self {
            unit_type: UnitType::new(c),
            position,
            attack_power: 3,
            hit_points: 200,
        }
    }

    fn hit(&mut self, attack_power: i32) {
        self.hit_points -= attack_power;
    }

    fn is_dead(&self) -> bool {
        self.hit_points <= 0
    }

    fn move_unit(&mut self, new_pos: usize) {
        self.position = new_pos;
    }

    fn sort_key(&self) -> (i32, usize) {
        (self.hit_points, self.position)
    }
}

// Finds all the units in the map, and builds a list of Unit out of them.
fn build_units_list(map: &Grid) -> Vec<Unit> {
    map.values
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if ['E', 'G'].contains(c) {
                Some(Unit::new(*c, i))
            } else {
                None
            }
        })
        .collect()
}

fn is_open(c: char) -> bool {
    c == '.'
}

// Find all positions of a specific type of units.
fn find_positions_of(map: &Grid, unit: UnitType) -> Vec<usize> {
    map.values
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if unit.is(*c) { Some(i) } else { None })
        .collect()
}

// Finds all the positions around the targets list.
fn in_range_positions(map: &Grid, targets: &[usize]) -> Vec<usize> {
    targets
        .iter()
        .flat_map(|t| {
            map.adjacent_pos(*t)
                .iter()
                .filter(|p| is_open(map.values[**p]))
                .copied()
                .collect::<Vec<_>>()
        })
        .collect()
}

// For a specific target, if there are multiple shortest path,
// we want the one in reading order.
// But as we care only about the first step, we can simplify the problem
// by trying to find the path for the 4 positions around, in reading order.
// If a path is found, returns a tuple with the length + the next step.
fn next_step_to_shortest_path(map: &Grid, start: usize, end: usize) -> Option<(usize, usize)> {
    map.adjacent_pos(start)
        .iter()
        .filter(|adj_pos| is_open(map.values[**adj_pos]))
        .filter_map(|adj_pos| {
            find_shortest_path(map, *adj_pos, end, is_open)
                .map(|shortest_path_len| (shortest_path_len + 1, *adj_pos))
        })
        .sorted()
        .next()
}

fn choose_next_move(map: &Grid, attacker_pos: usize) -> Option<usize> {
    let attacker = UnitType::new(map.values[attacker_pos]);
    let opponent = attacker.opponent();
    let targets = find_positions_of(map, opponent);
    let in_range = in_range_positions(map, &targets);

    in_range
        .iter()
        .filter_map(|t| next_step_to_shortest_path(map, attacker_pos, *t))
        .min()
        .map(|(_, next_step)| next_step)
}

fn move_unit(map: &mut Grid, units: &mut [Unit], unit_idx: usize, to: usize) {
    assert!(is_open(map.values[to]));
    // println!("Moving {} to {}", map.pos_as_str(units[unit_idx].position), map.pos_as_str(to));
    // map.print();

    // move on the map
    let from = units[unit_idx].position;
    let u = map.values[from];
    map.values[from] = '.';
    map.values[to] = u;
    // and the unit
    units[unit_idx].move_unit(to);
}

// Returns the position in the units vector.
fn choose_target(map: &Grid, units: &[Unit], attacker_pos: usize) -> Option<usize> {
    let attacker = UnitType::new(map.values[attacker_pos]);
    let opponent = attacker.opponent();

    // Choose target with fewest hit points
    map.adjacent_pos(attacker_pos)
        .iter()
        .filter_map(|adj_pos| {
            units
                .iter()
                .position(|u| u.position == *adj_pos && u.unit_type == opponent && !u.is_dead())
        })
        .min_by(|a, b| {
            let x = &units[*a];
            let y = &units[*b];
            x.sort_key().cmp(&y.sort_key())
        })
}

fn attack(map: &mut Grid, units: &mut [Unit], target_unit_index: usize, attack_power: i32) {
    assert!(!units[target_unit_index].is_dead());
    // println!("Attacking {}", map.pos_as_str(units[target_unit_index].position));

    units[target_unit_index].hit(attack_power);

    if units[target_unit_index].is_dead() {
        // println!("Killed {}", map.pos_as_str(units[target_unit_index].position));

        // Remove it from the map
        map.values[units[target_unit_index].position] = '.';
        // But keep it in the units list, to avoid messing the looping.
        // This means we always need to filter with is_dead().
    }
}

fn do_action(map: &mut Grid, units: &mut [Unit], unit_idx: usize) {
    // Skip units that were killed previously in this round
    if units[unit_idx].is_dead() {
        return;
    }

    let attacker_pos = units[unit_idx].position;
    let attacker_attack_power = units[unit_idx].attack_power;

    // println!("Action for {}", map.pos_as_str(attacker_pos));
    // map.print();

    // Does the attacker have an opponent that is just next?
    if let Some(target_unit_index) = choose_target(map, units, attacker_pos) {
        // Attack directly
        attack(map, units, target_unit_index, attacker_attack_power);
    } else if let Some(next_pos) = choose_next_move(map, attacker_pos) {
        // Move
        move_unit(map, units, unit_idx, next_pos);
        let attacker_pos = next_pos;

        // and attack if possible
        if let Some(target_unit_index) = choose_target(map, units, attacker_pos) {
            attack(map, units, target_unit_index, attacker_attack_power);
        }
    }
}

fn units_remaining(units: &[Unit], unit_type: UnitType) -> bool {
    units
        .iter()
        .any(|u| !u.is_dead() && u.unit_type == unit_type)
}

fn is_full_unit_dead(units: &[Unit]) -> bool {
    !units_remaining(units, UnitType::Elf) || !units_remaining(units, UnitType::Goblin)
}

fn count_alive_elves(units: &[Unit]) -> usize {
    units
        .iter()
        .filter(|u| !u.is_dead() && u.unit_type == UnitType::Elf)
        .count()
}

// Returning None means the battle was interrupted before the end.
fn full_battle(map: &mut Grid, units: &mut Vec<Unit>, stop_on_dead_elf: bool) -> Option<usize> {
    let mut combat_rounds = 0;
    let initial_elves_count = count_alive_elves(units);

    'outer: loop {
        units.sort_by_key(|u| u.position);

        // Execute a full round
        for i in 0..units.len() {
            if stop_on_dead_elf && count_alive_elves(units) < initial_elves_count {
                return None;
            }

            if is_full_unit_dead(units) {
                break 'outer;
            }
            do_action(map, units, i);
        }
        combat_rounds += 1;

        // println!("Round {} completed", combat_rounds); map.print();

        // Remove dead units
        units.retain(|u| !u.is_dead());
    }

    // Remove dead units before counting
    units.retain(|u| !u.is_dead());
    #[allow(clippy::cast_sign_loss)]
    let remaining_hps: usize = units.iter().map(|u| u.hit_points).sum::<i32>() as usize;

    if false {
        map.print();
        units.sort_by_key(|u| u.position);
        for u in units {
            println!("{}({})", u.unit_type.name(), u.hit_points);
        }
        println!(
            "Combat ends after {} full rounds, remaining HPs {}, outcome {}",
            combat_rounds,
            remaining_hps,
            combat_rounds * remaining_hps
        );
    }

    Some(combat_rounds * remaining_hps)
}

fn outcome(map: &Grid) -> usize {
    let mut map = map.clone();
    let mut units = build_units_list(&map);
    full_battle(&mut map, &mut units, false).unwrap()
}

fn set_elves_attack(units: &mut [Unit], attack_power: i32) {
    units
        .iter_mut()
        .filter(|u| u.unit_type == UnitType::Elf)
        .for_each(|u| u.attack_power = attack_power);
}

fn outcome_no_dead_elves(map: &Grid) -> usize {
    let mut attack_power = 4;

    loop {
        let mut map = map.clone();
        let mut units = build_units_list(&map);
        set_elves_attack(&mut units, attack_power);

        if let Some(res) = full_battle(&mut map, &mut units, true) {
            return res;
        }
        attack_power += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    let param = std::env::args().nth(1).unwrap_or_default();
    if param == "visu" {
        visualization::fancy(&map).unwrap();
        return;
    }

    println!("Part 1: {}", outcome(&map));
    println!("Part 2: {}", outcome_no_dead_elves(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_target() {
        let map = Grid::build(
            r"#######
#E..G.#
#...#.#
#.G.#G#
#######",
        );
        assert_eq!(choose_next_move(&map, map.pos(1, 1)), Some(map.pos(1, 2)));

        let map = Grid::build(
            r"#########
#...E...#
#.G.....#
#########",
        );
        assert_eq!(choose_next_move(&map, map.pos(2, 2)), Some(map.pos(1, 2)));
    }

    const INPUT0: &str = include_str!("../resources/input_test_0");
    const INPUT1: &str = include_str!("../resources/input_test_1");
    const INPUT2: &str = include_str!("../resources/input_test_2");
    const INPUT3: &str = include_str!("../resources/input_test_3");
    const INPUT4: &str = include_str!("../resources/input_test_4");
    const INPUT5: &str = include_str!("../resources/input_test_5");

    #[test]
    fn test_part1() {
        assert_eq!(outcome(&Grid::build(INPUT0)), 27730);
        assert_eq!(outcome(&Grid::build(INPUT1)), 36334);
        assert_eq!(outcome(&Grid::build(INPUT2)), 39514);
        assert_eq!(outcome(&Grid::build(INPUT3)), 27755);
        assert_eq!(outcome(&Grid::build(INPUT4)), 28944);
        assert_eq!(outcome(&Grid::build(INPUT5)), 18740);
    }

    #[test]
    fn test_part2() {
        assert_eq!(outcome_no_dead_elves(&Grid::build(INPUT0)), 4988);
        assert_eq!(outcome_no_dead_elves(&Grid::build(INPUT2)), 31284);
        assert_eq!(outcome_no_dead_elves(&Grid::build(INPUT3)), 3478);
        assert_eq!(outcome_no_dead_elves(&Grid::build(INPUT4)), 6474);
        assert_eq!(outcome_no_dead_elves(&Grid::build(INPUT5)), 1140);
    }
}
