use std::io::{self, Read};

#[rustfmt::skip]
mod shop {
    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Item {
        pub cost: u32,
        pub damage: u32,
        pub armor: u32,
    }

    #[derive(Debug, Clone)]
    #[allow(clippy::module_name_repetitions)]
    pub struct ShopItem {
        name: String,
        pub item: Item,
    }

    impl ShopItem {
        fn new(name: &str, item: Item) -> Self {
            Self { name: name.to_string(), item }
        }

        fn empty() -> Self {
            Self { name: String::new(), item: Item { cost: 0, damage: 0, armor: 0 } }
        }
    }

    fn merge_items(items: &[&ShopItem]) -> ShopItem {
        ShopItem {
            name: items.iter().map(|i| i.name.clone()).join(", "),
            item: Item {
                cost: items.iter().map(|i| i.item.cost).sum(), 
                damage: items.iter().map(|i| i.item.damage).sum(), 
                armor: items.iter().map(|i| i.item.armor).sum()
            }
        }
    }

    fn weapons() -> Vec<ShopItem> {
        // Must buy exactly 1
        let weapons: [ShopItem; 5] = [
            ShopItem::new("Dagger",     Item { cost:  8, damage: 4, armor: 0 }),
            ShopItem::new("Shortsword", Item { cost: 10, damage: 5, armor: 0 }),
            ShopItem::new("Warhammer",  Item { cost: 25, damage: 6, armor: 0 }),
            ShopItem::new("Longsword",  Item { cost: 40, damage: 7, armor: 0 }),
            ShopItem::new("Greataxe",   Item { cost: 74, damage: 8, armor: 0 }),
        ];
        weapons.to_vec()
    }

    fn armor() -> Vec<ShopItem> {
        // Optional: 0 or 1
        let armor: [ShopItem; 5] = [
            ShopItem::new("Leather",    Item { cost:  13, damage: 0, armor: 1 }),
            ShopItem::new("Chainmail",  Item { cost:  31, damage: 0, armor: 2 }),
            ShopItem::new("Splintmail", Item { cost:  53, damage: 0, armor: 3 }),
            ShopItem::new("Bandedmail", Item { cost:  75, damage: 0, armor: 4 }),
            ShopItem::new("Platemail",  Item { cost: 102, damage: 0, armor: 5 }),
        ];
        let mut o: Vec<_> = armor.to_vec();
        o.push(ShopItem::empty());
        o
    }

    fn rings() -> Vec<ShopItem> {
        // 0, 1 or 2
        let rings: [ShopItem; 6] = [
            ShopItem::new("Damage +1",  Item { cost:  25, damage: 1, armor: 0 }),
            ShopItem::new("Damage +2",  Item { cost:  50, damage: 2, armor: 0 }),
            ShopItem::new("Damage +3",  Item { cost: 100, damage: 3, armor: 0 }),
            ShopItem::new("Defense +1", Item { cost:  20, damage: 0, armor: 1 }),
            ShopItem::new("Defense +2", Item { cost:  40, damage: 0, armor: 2 }),
            ShopItem::new("Defense +3", Item { cost:  80, damage: 0, armor: 3 }),
        ];   
        let mut o: Vec<_> = rings.to_vec();
        o.extend(rings.iter().permutations(2)
        .map(|v| merge_items(&v)));
        o.push(ShopItem::empty());
        o
    }

    pub fn shopping_options() -> Vec<ShopItem> {
        let w = weapons();
        let a = armor();
        let r = rings();
        [w, a, r].iter().map(|x| x.iter())
        .multi_cartesian_product()
        .map(|v| merge_items(&v))
        .unique_by(|i| i.item)
        .sorted_by_key(|k| k.item.cost)
        .collect()
        // .for_each(|v| println!("{:?}", v));
    }
}
use shop::ShopItem;

#[derive(Debug, Clone)]
struct Character {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

impl Character {
    fn new(hit_points: u32, shop_item: &ShopItem) -> Self {
        Character {
            hit_points,
            damage: shop_item.item.damage,
            armor: shop_item.item.armor,
        }
    }

    fn attack(&self, other: &mut Character) {
        // at least 1 damage
        let damage_dealt = 1.max(self.damage.saturating_sub(other.armor));
        other.hit_points = other.hit_points.saturating_sub(damage_dealt);
    }

    fn is_dead(&self) -> bool {
        self.hit_points == 0
    }
}

fn build_boss(input: &str) -> Character {
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
    Character {
        hit_points,
        damage,
        armor,
    }
}

// Plays until someone dies. Returns true if the player won.
fn play_round(player: &mut Character, boss: &mut Character) -> bool {
    loop {
        player.attack(boss);
        if boss.is_dead() {
            return true;
        }
        boss.attack(player);
        if player.is_dead() {
            return false;
        }
    }
}

fn least_gold_and_win(initial_boss: &Character, hit_points: u32) -> u32 {
    let options = shop::shopping_options();

    let mut cheapest_win = u32::MAX;
    // sorted in increasing cost, we look at most expensive first
    for option in options.into_iter().rev() {
        let mut player = Character::new(hit_points, &option);
        let mut boss: Character = initial_boss.clone();
        if play_round(&mut player, &mut boss) {
            // println!("{:?} won, spent {:?}", player, option);
            cheapest_win = option.item.cost;
        }
    }
    cheapest_win
}

fn most_gold_and_loose(initial_boss: &Character, hit_points: u32) -> u32 {
    let options = shop::shopping_options();

    let mut most_expensive_loss = u32::MAX;
    // sorted in increasing cost, we look at cheapest expensive first
    for option in options {
        let mut player = Character::new(hit_points, &option);
        let mut boss: Character = initial_boss.clone();
        if !play_round(&mut player, &mut boss) {
            // println!("{:?} lost, spent {:?}", player, option);
            most_expensive_loss = option.item.cost;
        }
    }
    most_expensive_loss
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let boss = build_boss(&input);
    println!("Boss {boss:?}");

    println!("Part 1: {}", least_gold_and_win(&boss, 100));
    println!("Part 2: {}", most_gold_and_loose(&boss, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_play_round() {
        let mut player = Character {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let mut boss = Character {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        assert!(play_round(&mut player, &mut boss));
    }

    #[test]
    fn test_part1() {
        assert_eq!(least_gold_and_win(&build_boss(INPUT_TEST), 8), 65);
    }
}
