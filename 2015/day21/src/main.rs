use std::io::{self, Read};

struct Boss {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

fn build_boss(input: &str) -> Boss {
    let mut hit_points: u32 = 0;
    let mut damage: u32 = 0;
    let mut armor: u32 = 0;
    for line in input.lines() {
        if let Ok(val) = line.trim_start_matches("Hit Points: ").parse() {
            hit_points = val;
        } else if let Ok(val) = line.trim_start_matches("Damage: ").parse() {
            damage = val;
        } else if let Ok(val) = line.trim_start_matches("Armor: ").parse() {
            armor = val;
        }
    }
    Boss { hit_points, damage, armor }
}

#[rustfmt::skip]
mod shop {
    struct ShopItem {
        name: &'static str,
        cost: u32,
        damage: u32,
        armor: u32,
    }

    const WEAPONS: [ShopItem; 5] = [
        ShopItem { name: "Dagger",     cost: 8, damage: 4, armor: 0 },
        ShopItem { name: "Shortsword", cost:10, damage: 5, armor: 0 },
        ShopItem { name: "Warhammer",  cost:25, damage: 6, armor: 0 },
        ShopItem { name: "Longsword",  cost:40, damage: 7, armor: 0 },
        ShopItem { name: "Greataxe",   cost:74, damage: 8, armor: 0 },
    ];

    const ARMOR: [ShopItem; 5] = [
        ShopItem { name: "Leather",    cost: 13, damage: 0, armor: 1 },
        ShopItem { name: "Chainmail",  cost: 31, damage: 0, armor: 2 },
        ShopItem { name: "Splintmail", cost: 53, damage: 0, armor: 3 },
        ShopItem { name: "Bandedmail", cost: 75, damage: 0, armor: 4 },
        ShopItem { name: "Platemail",  cost:102, damage: 0, armor: 5 },
    ];

    const RINGS: [ShopItem; 6] = [
        ShopItem { name: "Damage +1",  cost:  25, damage: 1, armor: 0 },
        ShopItem { name: "Damage +2",  cost:  50, damage: 2, armor: 0 },
        ShopItem { name: "Damage +3",  cost: 100, damage: 3, armor: 0 },
        ShopItem { name: "Defense +1", cost:  20, damage: 0, armor: 1 },
        ShopItem { name: "Defense +2", cost:  40, damage: 0, armor: 2 },
        ShopItem { name: "Defense +3", cost:  80, damage: 0, armor: 3 },
    ];
}

fn least_gold_and_win(boss: &Boss, hit_points: u32) -> u32 {
    0
}

fn part2(input: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let boss = build_boss(&input);

    println!("Part 1: {}", least_gold_and_win(&boss, 100));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(least_gold_and_win(&build_boss(INPUT_TEST), 8), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_TEST), 0);
    }
}
