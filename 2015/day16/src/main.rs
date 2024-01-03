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

fn find_correct_sue(list: &[AuntSue]) -> usize {
    let indices = mfcsam_msg();

    let matching: Vec<usize> = list
        .iter()
        .enumerate()
        .filter(|(_, aunt)| {
            aunt.iter().all(|clue| {
                if let Some(v) = indices.get(&clue.0) {
                    *v == clue.1
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

fn part2(list: &[AuntSue]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let list = build(&input);

    println!("Part 1: {}", find_correct_sue(&list));
    println!("Part 2: {}", part2(&list));
}
