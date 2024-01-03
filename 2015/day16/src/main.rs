use std::io::{self, Read};

use fxhash::FxHashMap;

const MFCSAM_MESSAGE: &str = r"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

fn mfcsam_msg() -> FxHashMap<String, u32> {
    MFCSAM_MESSAGE
        .lines()
        .map(|line| {
            let en: Vec<&str> = line.split(": ").collect();
            (en[0].to_string(), en[1].parse().unwrap())
        })
        .collect()
}

// Things we remember about each aunt
type AuntSue = Vec<(String, u32)>;

fn build(input: &str) -> Vec<AuntSue> {
    // Remember that aunt at index i is the Sue i + 1
    // Sue 1: goldfish: 6, trees: 9, akitas: 0
    input
        .lines()
        .map(|line| {
            // Drop the annoying prefix
            let i = line.find(':').unwrap() + 2;
            let parts: Vec<&str> = line[i..].split(", ").collect();
            parts
                .iter()
                .map(|p| {
                    let en: Vec<&str> = p.split(": ").collect();
                    (en[0].to_string(), en[1].parse().unwrap())
                })
                .collect::<Vec<(String, u32)>>()
        })
        .collect()
}

fn find_correct_sue_with_cmp(
    list: &[AuntSue],
    mfcsam_msg: &FxHashMap<String, u32>,
    clue_check_fn: fn(&str, u32, u32) -> bool,
) -> usize {
    let matching: Vec<usize> = list
        .iter()
        .enumerate()
        .filter(|(_, aunt)| {
            aunt.iter().all(|remember_item| {
                if let Some(mfcsam_val) = mfcsam_msg.get(&remember_item.0) {
                    clue_check_fn(&remember_item.0, remember_item.1, *mfcsam_val)
                } else {
                    // don't remember, can't say.. so assuming true then
                    true
                }
            })
        })
        .map(|(idx, _)| idx)
        .collect();
    // hoping it gave us only one match
    assert_eq!(matching.len(), 1);
    matching[0] + 1
}

fn clue_check_1(_remember_elt: &str, remember_val: u32, mfcsam_val: u32) -> bool {
    mfcsam_val == remember_val
}

fn find_correct_sue(list: &[AuntSue], mfcsam_msg: &FxHashMap<String, u32>) -> usize {
    find_correct_sue_with_cmp(list, mfcsam_msg, clue_check_1)
}

fn clue_check_2(remember_elt: &str, remember_val: u32, mfcsam_val: u32) -> bool {
    if remember_elt == "cats" || remember_elt == "trees" {
        // greater than:
        mfcsam_val < remember_val
    } else if remember_elt == "pomeranians" || remember_elt == "goldfish" {
        // lower
        mfcsam_val > remember_val
    } else {
        mfcsam_val == remember_val
    }
}

fn real_correct_sue(list: &[AuntSue], mfcsam_msg: &FxHashMap<String, u32>) -> usize {
    find_correct_sue_with_cmp(list, mfcsam_msg, clue_check_2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let list = build(&input);
    let mfcsam_msg = mfcsam_msg();

    println!("Part 1: {}", find_correct_sue(&list, &mfcsam_msg));
    println!("Part 2: {}", real_correct_sue(&list, &mfcsam_msg));
}
