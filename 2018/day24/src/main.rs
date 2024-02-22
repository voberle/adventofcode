use std::{
    fmt,
    io::{self, Read},
    str::Lines,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

type AttackType = String;

#[derive(Debug, Clone, Copy)]
enum ArmyName {
    ImmuneSystem,
    Infection,
}

impl fmt::Display for ArmyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ArmyName::ImmuneSystem => "Immune System",
                ArmyName::Infection => "Infection",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Group {
    army_name: ArmyName,
    units_count: u32,
    hit_points: u32,
    immunities: Vec<AttackType>,
    weaknesses: Vec<AttackType>,
    attack_damage: u32,
    attack_type: AttackType,
    initiative: u32,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units_count * self.attack_damage
    }

    fn has_units(&self) -> bool {
        self.units_count > 0
    }

    // By how much the attacker effective power must be multiplied
    // to get the damage an attack would deal on this group.
    fn damage_ratio(&self, attack_type: &AttackType) -> u32 {
        if self.immunities.contains(attack_type) {
            0
        } else if self.weaknesses.contains(attack_type) {
            2
        } else {
            1
        }
    }
}

#[derive(Debug, Clone)]
struct Army {
    name: ArmyName,
    groups: Vec<Group>,
}

impl Army {
    fn build(name: ArmyName, it: &mut Lines<'_>) -> Self {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"(?<units_count>\d+) units each with (?<hit_points>\d+) hit points *\(?(?<immunities_weaknesses>[\w, ;]*)\)? *with an attack that does (?<attack_damage>\d+) (?<attack_type>\w+) damage at initiative (?<initiative>\d+)").unwrap();
        }
        let mut groups: Vec<Group> = Vec::new();
        for line in it.by_ref() {
            let Some(caps) = LINE_RE.captures(line) else {
                break;
            };
            let mut immunities: Vec<String> = Vec::new();
            let mut weaknesses: Vec<String> = Vec::new();
            if let Some(iw) = caps.name("immunities_weaknesses") {
                let parts = if iw.as_str().contains(';') {
                    iw.as_str().split("; ").collect()
                } else {
                    vec![iw.as_str()]
                };
                for p in parts {
                    if p.starts_with("immune to ") {
                        immunities.extend(
                            p.trim_start_matches("immune to ")
                                .split(", ")
                                .map(ToString::to_string)
                                .collect::<Vec<_>>(),
                        );
                    }
                    if p.starts_with("weak to ") {
                        weaknesses.extend(
                            p.trim_start_matches("weak to ")
                                .split(", ")
                                .map(ToString::to_string)
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
            let group = Group {
                army_name: name,
                units_count: caps["units_count"].parse().unwrap(),
                hit_points: caps["hit_points"].parse().unwrap(),
                immunities,
                weaknesses,
                attack_damage: caps["attack_damage"].parse().unwrap(),
                attack_type: caps["attack_type"].to_string(),
                initiative: caps["initiative"].parse().unwrap(),
            };
            groups.push(group);
        }
        Self { name, groups }
    }

    fn print_groups(&self) {
        if self.has_units() {
            for (i, g) in self.groups.iter().enumerate() {
                if g.has_units() {
                    println!("Group {} contains {} units", i + 1, g.units_count);
                }
            }
        } else {
            println!("No groups remain.");
        }
    }

    fn has_units(&self) -> bool {
        self.groups.iter().any(Group::has_units)
    }

    fn units_total(&self) -> u32 {
        self.groups.iter().map(|g| g.units_count).sum()
    }

    fn boost(&mut self, boost: u32) {
        for g in &mut self.groups {
            g.attack_damage += boost;
        }
    }
}

// Returns Immune System army followed by Infection one.
fn build_armies(input: &str) -> (Army, Army) {
    let mut it = input.lines();

    assert_eq!(it.next().unwrap(), "Immune System:");
    let immune_system = Army::build(ArmyName::ImmuneSystem, &mut it);

    assert_eq!(it.next().unwrap(), "Infection:");
    let infection = Army::build(ArmyName::Infection, &mut it);

    (immune_system, infection)
}

fn calc_damage(attacker: &Group, defender: &Group) -> u32 {
    defender.damage_ratio(&attacker.attack_type) * attacker.effective_power()
}

// Returns the list of targets, in a vector indexed by attackers.
fn target_selection<const DEBUG: bool>(
    attacking_army: &Army,
    defending_army: &Army,
) -> Vec<Option<usize>> {
    let mut targets: Vec<Option<usize>> = vec![None; attacking_army.groups.len()];
    for (attacker_idx, attacker) in attacking_army
        .groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.has_units())
        .sorted_unstable_by_key(|(_, g)| (g.effective_power(), g.initiative))
        .rev()
    {
        let possible_targets = defending_army
            .groups
            .iter()
            .enumerate()
            .filter(|(i, d)|
                    // target group must have units left
                    d.has_units()
                    // target must not have been selected before
                    && !targets.contains(&Some(*i))
                    // Damage has to be positive
                    && calc_damage(attacker, d) > 0)
            .map(|(i, d)| {
                (
                    i,
                    calc_damage(attacker, d),
                    d.effective_power(),
                    d.initiative,
                )
            })
            .collect_vec();

        if DEBUG {
            for t in &possible_targets {
                println!(
                    "{} group {} would deal defending group {} {} damage",
                    attacking_army.name,
                    attacker_idx + 1,
                    t.0 + 1,
                    t.1,
                );
            }
        }

        if let Some(target) =
            possible_targets
                .iter()
                .max_by_key(|(_, damage, effective_power, initiative)| {
                    (damage, effective_power, initiative)
                })
        {
            targets[attacker_idx] = Some(target.0);
        }
    }
    targets
}

fn attacking_phase<const DEBUG: bool>(
    immune_system: &mut Army,
    infection: &mut Army,
    immune_to_infection: &[Option<usize>],
    infection_to_immune: &[Option<usize>],
) -> Result<(), &'static str> {
    let attacking_order: Vec<(ArmyName, usize)> = immune_system
        .groups
        .iter()
        .enumerate()
        .zip(infection.groups.iter().enumerate())
        .flat_map(|((gr_idx1, gr1), (gr_idx2, gr2))| {
            [
                (gr1.army_name, gr_idx1, gr1.initiative),
                (gr2.army_name, gr_idx2, gr2.initiative),
            ]
        })
        .sorted_by_key(|(_, _, initiative)| *initiative)
        .rev()
        .map(|(name, gr_idx, _)| (name, gr_idx))
        .collect();

    let mut killed_total = 0;
    for (attacker_army_name, attacker_group_idx) in attacking_order {
        let immune_system_is_attacker = matches!(attacker_army_name, ArmyName::ImmuneSystem);
        let targets = if immune_system_is_attacker {
            immune_to_infection
        } else {
            infection_to_immune
        };
        let Some(defender_group_idx) = targets[attacker_group_idx] else {
            continue;
        };

        let (attacker, defender) = if immune_system_is_attacker {
            (
                &immune_system.groups[attacker_group_idx],
                &mut infection.groups[defender_group_idx],
            )
        } else {
            (
                &infection.groups[attacker_group_idx],
                &mut immune_system.groups[defender_group_idx],
            )
        };

        if !attacker.has_units() {
            continue;
        }

        let damage = calc_damage(attacker, defender);
        let units_killed = damage / defender.hit_points;
        let kill_count = if defender.units_count > units_killed {
            units_killed
        } else {
            defender.units_count
        };
        killed_total += kill_count;

        if DEBUG {
            println!(
                "{} group {} attacks defending group {}, killing {} units",
                attacker_army_name,
                attacker_group_idx + 1,
                defender_group_idx + 1,
                kill_count,
            );
        }
        defender.units_count = defender.units_count.saturating_sub(units_killed);
    }

    if killed_total == 0 {
        return Err("No kills");
    }
    Ok(())
}

#[allow(dead_code)]
fn print_groups(immune_system: &Army, infection: &Army) {
    println!("-------------");
    println!("Immune System:");
    immune_system.print_groups();
    println!("Infection:");
    infection.print_groups();
    println!();
}

fn fight<const DEBUG: bool>(
    immune_system: &mut Army,
    infection: &mut Army,
) -> Result<(), &'static str> {
    while immune_system.has_units() && infection.has_units() {
        if DEBUG {
            print_groups(immune_system, infection);
        }

        let infection_to_immune = target_selection::<DEBUG>(infection, immune_system);
        let immune_to_infection = target_selection::<DEBUG>(immune_system, infection);
        if DEBUG {
            println!();
        }

        attacking_phase::<DEBUG>(
            immune_system,
            infection,
            &immune_to_infection,
            &infection_to_immune,
        )?;
    }
    if DEBUG {
        print_groups(immune_system, infection);
    }
    Ok(())
}

fn winner_remaining_units(immune_system: &Army, infection: &Army) -> u32 {
    let mut immune_system = immune_system.clone();
    let mut infection = infection.clone();

    fight::<false>(&mut immune_system, &mut infection).unwrap();

    if immune_system.has_units() {
        immune_system.units_total()
    } else {
        infection.units_total()
    }
}

fn immune_system_units_with_boost(immune_system: &Army, infection: &Army) -> u32 {
    for boost in 1.. {
        let mut immune_system = immune_system.clone();
        let mut infection = infection.clone();
        immune_system.boost(boost);

        let stalemate = fight::<false>(&mut immune_system, &mut infection).is_err();
        if stalemate {
            continue;
        }

        if immune_system.has_units() && !infection.has_units() {
            // println!("Immune won, boost {}", boost);
            return immune_system.units_total();
        }
    }
    panic!("Failed to find a winning boost");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (immune_system, infection) = build_armies(&input);
    // println!("{:#?}", immune_system);
    // println!("{:#?}", infection);

    println!(
        "Part 1: {}",
        winner_remaining_units(&immune_system, &infection)
    );
    println!(
        "Part 2: {}",
        immune_system_units_with_boost(&immune_system, &infection)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        let (immune_system, infection) = build_armies(INPUT_TEST);
        assert_eq!(winner_remaining_units(&immune_system, &infection), 5216);
    }

    #[test]
    fn test_fight_with_boost() {
        let (mut immune_system, mut infection) = build_armies(INPUT_TEST);
        immune_system.boost(1570);
        let _ = fight::<false>(&mut immune_system, &mut infection);

        assert!(immune_system.has_units());
        assert!(!infection.has_units());
        assert_eq!(immune_system.units_total(), 51);
    }
}
