// https://adventofcode.com/2023/day/12

use memoize::memoize;
use std::{
    fmt,
    io::{self, BufRead},
};

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

#[derive(Debug, PartialEq, Clone)]
struct Record {
    states: String,
    damaged_cont_group_sizes: Vec<usize>,
}

impl Record {
    fn new(states: String, groups: Vec<usize>) -> Self {
        Self {
            states,
            damaged_cont_group_sizes: groups,
        }
    }

    fn build(r: &str) -> Self {
        let mut it = r.split_whitespace();
        Self {
            states: it.next().unwrap().to_string(),
            damaged_cont_group_sizes: it
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    // For part 2
    fn unfold(&self) -> Self {
        Record::new(
            self.states.clone()
                + "?"
                + &self.states
                + "?"
                + &self.states
                + "?"
                + &self.states
                + "?"
                + &self.states,
            self.damaged_cont_group_sizes.repeat(5),
        )
    }

    fn arrangements_count(&self) -> usize {
        find_arrangements(self.states.clone(), self.damaged_cont_group_sizes.clone())
    }
}

#[memoize]
fn find_arrangements(states: String, damaged_cont_group_sizes: Vec<usize>) -> usize {
    let group_size = damaged_cont_group_sizes[0];
    if damaged_cont_group_sizes.len() == 1 {
        let arrangements_count = find_arrangements_for_last_group(&states, group_size);
        return arrangements_count;
    }

    let arrangements: Vec<String> = find_arrangements_for_group(&states, group_size);

    arrangements
        .iter()
        .map(|s| find_arrangements(s.to_string(), damaged_cont_group_sizes[1..].to_vec()))
        .sum()
}

// Find all the ways that group can be placed in the states string, by assuming this is the first group.
// Returns the list of states with the group replaced.
fn find_arrangements_for_group(states: &str, group_size: usize) -> Vec<String> {
    let damaged_string = DAMAGED.to_string().repeat(group_size);

    let mut res = Vec::new();
    for i in 0..states.len() {
        let s = OPERATIONAL.to_string().repeat(i) + &damaged_string + &OPERATIONAL.to_string();
        if compare_string_to_state(states, &s) {
            let mut new_state = states.to_string();
            let nb_to_replace = i + group_size + 1;
            new_state.replace_range(
                0..nb_to_replace,
                &OPERATIONAL.to_string().repeat(nb_to_replace),
            );
            res.push(new_state);
        }
    }
    res
}

fn find_arrangements_for_last_group(states: &str, group_size: usize) -> usize {
    let damaged_string = DAMAGED.to_string().repeat(group_size);

    let mut count = 0;
    for i in 0..states.len() {
        let s = OPERATIONAL.to_string().repeat(i) + &damaged_string;
        if compare_string_to_state(states, &s) {
            // If there are still # in the remaining chars, it doesn't work
            if states[i + group_size..].contains(DAMAGED) {
                continue;
            }
            count += 1;
        }
    }
    count
}

// Check if s matches the beginning of states.
// Note that states might be longer that s.
fn compare_string_to_state(states: &str, s: &str) -> bool {
    if states.len() < s.len() {
        return false;
    }
    std::iter::zip(states.chars(), s.chars()).all(
        |(state, c)| {
            if state == UNKNOWN {
                true
            } else {
                state == c
            }
        },
    )
}

#[test]
fn test_find_arrangements_for_group() {
    assert_eq!(
        find_arrangements_for_group("????.?#?????.??", 3),
        vec![
            ".....?#?????.??",
            ".....?#?????.??",
            ".........???.??",
            "..........??.??"
        ]
    );
}

#[test]
fn test_arrangements_count_3() {
    assert_eq!(Record::build("???.### 1,1,3").arrangements_count(), 1);
    assert_eq!(
        Record::build(".??..??...?##. 1,1,3").arrangements_count(),
        4
    );
    assert_eq!(
        Record::build("?#?#?#?#?#?#?#? 1,3,1,6").arrangements_count(),
        1
    );
    assert_eq!(Record::build("????.#...#... 4,1,1").arrangements_count(), 1);
    assert_eq!(
        Record::build("????.######..#####. 1,6,5").arrangements_count(),
        4
    );
    assert_eq!(Record::build("?###???????? 3,2,1").arrangements_count(), 10);

    assert_eq!(
        Record::build("?????#????#? 2,1,1,1").arrangements_count(),
        7
    );
}

fn sum_of_arrangements(records: &[Record]) -> usize {
    records.iter().map(|r| r.arrangements_count()).sum()
}

fn sum_of_unfolded_arrangements(records: &[Record]) -> usize {
    records
        .iter()
        .map(|r| r.unfold().arrangements_count())
        .sum()
}

fn build_records<R>(reader: &mut R) -> Vec<Record>
where
    R: BufRead,
{
    reader
        .lines()
        .map(|row| Record::build(&row.unwrap()))
        .collect::<Vec<Record>>()
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            &self.states,
            self.damaged_cont_group_sizes
                .iter()
                .map(usize::to_string)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

fn main() {
    let stdin = io::stdin();

    let records: Vec<Record> = build_records(&mut stdin.lock());
    // records.iter().for_each(|r| println!("{}", r));

    println!("Part 1: {}", sum_of_arrangements(&records));
    println!("Part 2: {}", sum_of_unfolded_arrangements(&records));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    fn part1(filename: &str) -> usize {
        let mut reader = BufReader::new(File::open(filename).unwrap());
        let records: Vec<Record> = build_records(&mut reader);
        sum_of_arrangements(&records)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("day12_part1/resources/input_test1"), 6);
        assert_eq!(part1("day12_part1/resources/input_test2"), 21);
    }
}
