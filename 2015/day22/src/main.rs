use std::io::{self, Read};

trait Spell: SpellClone {
    fn cost(&self) -> u32;

    // This is to be called on each turn, in case the spell is active and needs to do something.
    fn turn(&mut self, _player: &mut Player, _boss: &mut Boss) {}

    // This is called when the player casts the spell.
    // cast must be called after turn.
    fn cast(&mut self, player: &mut Player, boss: &mut Boss);

    fn is_active(&self) -> bool {
        false
    }
}

// https://stackoverflow.com/a/30353928
trait SpellClone {
    fn clone_box(&self) -> Box<dyn Spell>;
}

impl<T> SpellClone for T
where
    T: 'static + Spell + Clone,
{
    fn clone_box(&self) -> Box<dyn Spell> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Spell> {
    fn clone(&self) -> Box<dyn Spell> {
        self.clone_box()
    }
}

// Magic Missile costs 53 mana. It instantly does 4 damage.
#[derive(Clone)]
struct MagicMissile {}

impl MagicMissile {
    fn new() -> Self {
        Self {}
    }
}

impl Spell for MagicMissile {
    fn cost(&self) -> u32 {
        53
    }

    fn cast(&mut self, _player: &mut Player, boss: &mut Boss) {
        boss.hit_points = boss.hit_points.saturating_sub(4);
    }
}

// Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
#[derive(Clone)]
struct Drain {}

impl Drain {
    fn new() -> Self {
        Self {}
    }
}
impl Spell for Drain {
    fn cost(&self) -> u32 {
        73
    }

    fn cast(&mut self, player: &mut Player, boss: &mut Boss) {
        boss.hit_points = boss.hit_points.saturating_sub(2);
        player.hit_points += 2;
    }
}

// Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
#[derive(Clone)]
struct Shield {
    timer: u32,
}

impl Shield {
    fn new() -> Self {
        Self { timer: 0 }
    }
}

impl Spell for Shield {
    fn cost(&self) -> u32 {
        113
    }

    fn cast(&mut self, player: &mut Player, _boss: &mut Boss) {
        assert!(!self.is_active());
        // effect starts
        assert_eq!(player.armor, 0);
        player.armor += 7;
        self.timer = 6;
    }

    fn turn(&mut self, player: &mut Player, _boss: &mut Boss) {
        if self.is_active() {
            self.timer -= 1;
            if self.timer == 0 {
                // effect ended
                player.armor -= 7;
                assert_eq!(player.armor, 0);
            }
        }
    }

    fn is_active(&self) -> bool {
        self.timer != 0
    }
}

// Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
#[derive(Clone)]
struct Poison {
    timer: u32,
}

impl Poison {
    fn new() -> Self {
        Self { timer: 0 }
    }
}

impl Spell for Poison {
    fn cost(&self) -> u32 {
        173
    }

    fn cast(&mut self, _player: &mut Player, _boss: &mut Boss) {
        assert!(!self.is_active());
        // effect starts
        self.timer = 6;
    }

    fn turn(&mut self, _player: &mut Player, boss: &mut Boss) {
        if self.is_active() {
            boss.hit_points = boss.hit_points.saturating_sub(3);
            self.timer -= 1;
        }
    }

    fn is_active(&self) -> bool {
        self.timer != 0
    }
}

// Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
#[derive(Clone)]
struct Recharge {
    timer: u32,
}

impl Recharge {
    fn new() -> Self {
        Self { timer: 0 }
    }
}

impl Spell for Recharge {
    fn cost(&self) -> u32 {
        229
    }

    fn cast(&mut self, _player: &mut Player, _boss: &mut Boss) {
        assert!(!self.is_active());
        // effect starts
        self.timer = 5;
    }

    fn turn(&mut self, player: &mut Player, _boss: &mut Boss) {
        if self.is_active() {
            player.mana += 101;
            self.timer -= 1;
        }
    }

    fn is_active(&self) -> bool {
        self.timer != 0
    }
}

fn build_spells() -> [Box<dyn Spell>; 5] {
    [
        Box::new(MagicMissile::new()),
        Box::new(Drain::new()),
        Box::new(Shield::new()),
        Box::new(Poison::new()),
        Box::new(Recharge::new()),
    ]
}

fn clone_spells(spells: &mut [Box<dyn Spell>]) -> [Box<dyn Spell>; 5] {
    [
        spells[0].clone(),
        spells[1].clone(),
        spells[2].clone(),
        spells[3].clone(),
        spells[4].clone(),
    ]
}

fn execute_spells_turn(spells: &mut [Box<dyn Spell>], player: &mut Player, boss: &mut Boss) {
    spells.iter_mut().for_each(|spell| spell.turn(player, boss));
}

// Indicates which spells can be used.
// Returns the indexes of the spell table (easier on the borrow checker).
fn usable_spells(spells: &mut [Box<dyn Spell>], player_mana: u32) -> Vec<usize> {
    spells
        .iter()
        .enumerate()
        .filter(|(_, spell)| !spell.is_active() && spell.cost() <= player_mana)
        .map(|(i, _)| i)
        .collect()
}

#[derive(Debug, Clone)]
struct Boss {
    hit_points: u32,
    damage: u32,
}

impl Boss {
    #[cfg(test)]
    fn new(hit_points: u32, damage: u32) -> Self {
        Boss { hit_points, damage }
    }

    fn build(input: &str) -> Boss {
        let mut hit_points: u32 = 0;
        let mut damage: u32 = 0;
        for line in input.lines() {
            if let Ok(val) = line.trim_start_matches("Hit Points: ").parse() {
                hit_points = val;
            } else if let Ok(val) = line.trim_start_matches("Damage: ").parse() {
                damage = val;
            }
        }
        Boss { hit_points, damage }
    }

    fn is_dead(&self) -> bool {
        self.hit_points == 0
    }

    fn attack(&self, player: &mut Player) {
        // at least 1 damage
        let damage_dealt = 1.max(self.damage.saturating_sub(player.armor));
        player.hit_points = player.hit_points.saturating_sub(damage_dealt);
    }
}

#[derive(Debug, Clone)]
struct Player {
    hit_points: u32,
    armor: u32,
    mana: u32,
}

impl Player {
    fn new(hit_points: u32, mana: u32) -> Self {
        Player {
            hit_points,
            armor: 0,
            mana,
        }
    }

    fn is_dead(&self) -> bool {
        self.hit_points == 0
    }

    fn cast(&mut self, spell: &mut Box<dyn Spell>, boss: &mut Boss) {
        // pay the cost
        assert!(self.mana >= spell.cost());
        self.mana -= spell.cost();
        // launch the spell
        spell.cast(self, boss);
    }
}

// Recursive function.
// Taking a player, its spells list and a boss, does the next round of fighting (player cast and boss attack),
// and then calls this function again.
// mana_spent is the the mana spent so far.
fn fight_to_win<const HARD: bool>(
    player: &mut Player,
    spells: &mut [Box<dyn Spell>],
    boss: &mut Boss,
    mana_spent: u32,
    min_mana_spent_for_win: &mut u32,
) {
    // Hard mode
    if HARD {
        player.hit_points -= 1;
        if player.is_dead() {
            return;
        }
    }

    // Player's spell turn
    execute_spells_turn(spells, player, boss);
    assert!(!player.is_dead()); //  only the boss can die on spell turns
    if boss.is_dead() {
        *min_mana_spent_for_win = mana_spent.min(*min_mana_spent_for_win);
        return;
    }

    let usable_spells = usable_spells(spells, player.mana);
    // If we cannot afford to cast any spell, we lose.
    if usable_spells.is_empty() {
        return;
    }
    for spell_to_cast in usable_spells {
        let mut player_copy = player.clone();
        let mut spells_copy = clone_spells(spells);
        let mut boss_copy = boss.clone();

        // Player's casting
        player_copy.cast(&mut spells_copy[spell_to_cast], &mut boss_copy);
        let new_mana_spent = mana_spent + spells_copy[spell_to_cast].cost();
        assert!(!player_copy.is_dead()); //  only the boss can die on spell turns
        if boss_copy.is_dead() {
            *min_mana_spent_for_win = new_mana_spent.min(*min_mana_spent_for_win);
            continue;
        }

        // Boss's spell turn
        execute_spells_turn(&mut spells_copy, &mut player_copy, &mut boss_copy);
        assert!(!player_copy.is_dead()); //  only the boss can die on spell turns
        if boss_copy.is_dead() {
            *min_mana_spent_for_win = new_mana_spent.min(*min_mana_spent_for_win);
            continue;
        }

        // Boss attack
        boss_copy.attack(&mut player_copy);
        if player_copy.is_dead() {
            continue;
        }

        assert!(!boss_copy.is_dead() && !player_copy.is_dead());

        // Optimization: Don't do anything if new_mana_spent is bigger than min already
        if *min_mana_spent_for_win < new_mana_spent {
            continue;
        }

        // nobody died, continue fighting
        fight_to_win::<HARD>(
            &mut player_copy,
            &mut spells_copy,
            &mut boss_copy,
            new_mana_spent,
            min_mana_spent_for_win,
        );
    }
}

fn least_mana_and_win<const HARD: bool>(initial_boss: &Boss, hit_points: u32, mana: u32) -> u32 {
    let mut player = Player::new(hit_points, mana);
    let mut spells = build_spells();
    let mut boss = initial_boss.clone();

    let mut min_mana_spent_for_win = u32::MAX;
    fight_to_win::<HARD>(
        &mut player,
        &mut spells,
        &mut boss,
        0,
        &mut min_mana_spent_for_win,
    );
    min_mana_spent_for_win
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let boss = Boss::build(&input);
    // println!("Boss {:?}", boss);

    println!("Part 1: {}", least_mana_and_win::<false>(&boss, 50, 500));
    println!("Part 2: {}", least_mana_and_win::<true>(&boss, 50, 500));
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAGIC_MISSILE: usize = 0;
    const DRAIN: usize = 1;
    const SHIELD: usize = 2;
    const POISON: usize = 3;
    const RECHARGE: usize = 4;

    fn both_alive(player: &mut Player, boss: &mut Boss) {
        assert!(!player.is_dead());
        assert!(!boss.is_dead());
    }

    #[test]
    fn test_boss_1() {
        let mut spells = build_spells();
        let mut player = Player::new(10, 250);
        let mut boss = Boss::new(13, 8);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        player.cast(&mut spells[POISON], &mut boss);
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 77);
        assert_eq!(boss.hit_points, 13);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        boss.attack(&mut player);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 77);
        assert_eq!(boss.hit_points, 10);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        player.cast(&mut spells[MAGIC_MISSILE], &mut boss);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 24);
        assert_eq!(boss.hit_points, 3);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        // not time for boss to attack
        assert!(boss.is_dead());
        assert!(!player.is_dead());
    }

    #[test]
    fn test_boss_2() {
        let mut spells = build_spells();
        let mut player = Player::new(10, 250);
        let mut boss = Boss::new(14, 8);
        let mut mana_spent = 0;

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        player.cast(&mut spells[RECHARGE], &mut boss);
        mana_spent += spells[RECHARGE].cost();
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 21);
        assert_eq!(boss.hit_points, 14);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        boss.attack(&mut player);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 122);
        assert_eq!(boss.hit_points, 14);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        player.cast(&mut spells[SHIELD], &mut boss);
        mana_spent += spells[SHIELD].cost();
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 110);
        assert_eq!(boss.hit_points, 14);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        boss.attack(&mut player);
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 211);
        assert_eq!(boss.hit_points, 14);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        player.cast(&mut spells[DRAIN], &mut boss);
        mana_spent += spells[DRAIN].cost();
        assert_eq!(player.hit_points, 3);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 239);
        assert_eq!(boss.hit_points, 12);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        boss.attack(&mut player);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 340);
        assert_eq!(boss.hit_points, 12);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        player.cast(&mut spells[POISON], &mut boss);
        mana_spent += spells[POISON].cost();
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 167);
        assert_eq!(boss.hit_points, 12);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        boss.attack(&mut player);
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 167);
        assert_eq!(boss.hit_points, 9);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        both_alive(&mut player, &mut boss);
        player.cast(&mut spells[MAGIC_MISSILE], &mut boss);
        mana_spent += spells[MAGIC_MISSILE].cost();
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 114);
        assert_eq!(boss.hit_points, 2);

        execute_spells_turn(&mut spells, &mut player, &mut boss);
        // not time for boss to attack
        assert!(boss.is_dead());
        assert!(!player.is_dead());
        assert_eq!(mana_spent, 641);
    }

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(
            least_mana_and_win::<false>(&Boss::build(INPUT_TEST), 10, 250),
            641
        );
    }
}
