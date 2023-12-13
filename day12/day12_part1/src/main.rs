// https://adventofcode.com/2023/day/12

use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead, BufReader}, usize,
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
        // ???.### 1,1,3
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

    fn to_string(&self) -> String {
        format!(
            "{} {}",
            &self.states,
            self.damaged_cont_group_sizes
                .iter()
                .map(usize::to_string)
                .join(",")
        )
    }

    // Calculates the size of the damaged continuous groups.
    // This function needs to be fairly fast, it will be called a lot.
    fn calc_state_group_sizes(states: &String) -> Vec<usize> {
        // Counting repeated characters with coalesce
        states
            .chars()
            .map(|c| (c, 1))
            .coalesce(|(c, n), (d, m)| {
                if c == d {
                    Ok((c, n + m))
                } else {
                    Err(((c, n), (d, m)))
                }
            })
            .filter_map(|(c, n)| if c == DAMAGED { Some(n) } else { None })
            .collect()
    }

    // Validates that the set of states is correct for the corresponding group sizes
    fn validate(&self) -> bool {
        Self::calc_state_group_sizes(&self.states) == self.damaged_cont_group_sizes
    }

    fn unknown_positions(&self) -> Vec<usize> {
        self.states
            .chars()
            .enumerate()
            .filter_map(|(i, s)| if s == UNKNOWN { Some(i) } else { None })
            .collect()
    }

    fn spring_count(&self) -> usize {
        self.states.len()
    }

    // Finds the total number of damaged states by adding the group counts
    fn damaged_count(&self) -> usize {
        self.damaged_cont_group_sizes.iter().sum()
    }

    // We know that the total of springs is the sum of the number of operational ones and the number of damaged ones.
    // states.len() = total_operational + total_damaged
    fn operational_count(&self) -> usize {
        self.spring_count() - self.damaged_count()
    }

    fn adjust_states(states: &str, rep: &[&char]) -> String {
        let mut i = 0;
        states
            .chars()
            .map(|c| {
                if c == UNKNOWN {
                    i += 1;
                    *rep[i - 1]
                } else {
                    c
                }
            })
            .collect()
    }

    fn arrangements_count(&self) -> usize {
        let known_operation_count = self.states.chars().filter(|c| *c == OPERATIONAL).count();
        let known_damaged_count = self.states.chars().filter(|c| *c == DAMAGED).count();
        let unknown_operation_count = self.operational_count() - known_operation_count;
        let unknown_damaged_count = self.damaged_count() - known_damaged_count;
        println!("{OPERATIONAL} known={known_operation_count} unknow={unknown_operation_count}");
        println!("{DAMAGED} known={known_damaged_count} unknow={unknown_damaged_count}");

        let mut v = vec![OPERATIONAL; unknown_operation_count];
        v.extend(vec![DAMAGED; unknown_damaged_count]);
        println!("{:?}", v);

        let t = v
            .iter()
            .permutations(unknown_operation_count + unknown_damaged_count)
            .unique()
            .filter(|rep| {
                let adj_s = Self::adjust_states(&self.states, &rep);
                Self::calc_state_group_sizes(&adj_s) == self.damaged_cont_group_sizes
            })
            .count();
        println!("Sum for {}: {}", self.to_string(), t);
        t
    }

    fn operational_combinations_iter(n: usize, k: usize) -> Vec<Vec<usize>> {
        itertools::repeat_n((0..=n), k).multi_cartesian_product()
            .filter(|comb| comb[1..comb.len()-1].iter().any(|n| *n != 0))
            .filter(|comb| comb.iter().sum::<usize>() == n)
            .collect()
    }

    fn create_string(operational_sizes: &Vec<usize>, damaged_sizes: &Vec<usize>) -> String {
        assert_eq!(operational_sizes.len(), damaged_sizes.len() + 1);
        std::iter::zip(operational_sizes, damaged_sizes)
            .map(|(o, d)| OPERATIONAL.to_string().repeat(*o) + &DAMAGED.to_string().repeat(*d))
            .join("")
        + &OPERATIONAL.to_string().repeat(*operational_sizes.last().unwrap())    
    }

    fn compare_string_against_state(&self, s: &str) -> bool {
        assert_eq!(self.states.len(), s.len());
        std::iter::zip(self.states.chars(), s.chars())
            .all(|(state, c)| {
                if state == UNKNOWN {
                    true
                } else {
                    state == c
                }
            })
    }

    fn arrangements_count_2(&self) -> usize {
        let n = self.operational_count();
        let k = self.damaged_cont_group_sizes.len() + 1;
        // println!("n={n} k={k}");
        let v: Vec<_> = 
        itertools::repeat_n((0..=n), k).multi_cartesian_product()
            .filter(|comb| comb[1..comb.len()-1].iter().any(|n| *n != 0))
            .filter(|comb| comb.iter().sum::<usize>() == n)
            .map(|operational_sizes| Self::create_string(&operational_sizes, &self.damaged_cont_group_sizes))
            .filter(|s| self.compare_string_against_state(s))
            .filter(|s| Self::calc_state_group_sizes(s) == self.damaged_cont_group_sizes)
            .collect()
        ;
        // println!("{}: {:#?}", v.len(), v);
        // TODO replace with it.count()
        println!("Sum for {}: {}", self.to_string(), v.len());
        v.len()
    }
}

#[test]
fn test_counts() {
    let r = Record::build("???.### 1,1,3");
    assert_eq!(r.spring_count(), 7);
    assert_eq!(r.damaged_count(), 5);
    assert_eq!(r.operational_count(), 2);
}

#[test]
fn test_validate() {
    assert!(Record::build("#.#.### 1,1,3").validate());
    assert!(Record::build(".#...#....###. 1,1,3").validate());
    assert!(Record::build(".#.###.#.###### 1,3,1,6").validate());
    assert!(Record::build("####.#...#... 4,1,1").validate());
    assert!(Record::build("#....######..#####. 1,6,5").validate());
    assert!(Record::build(".###.##....# 3,2,1").validate());
    // Doesn't validate:
    assert!(!Record::build(".###.#....# 3,2,1").validate());
}

#[test]
fn test_adjust_states() {
    assert_eq!(
        Record::adjust_states("..?.?#.??#?", &vec![&'.', &'#', &'#', &'.', &'#']),
        "....##.#.##"
    );
}

// #[test]
fn test_arrangements_count() {
    assert!(Record::build("??????#???????? 7,2").arrangements_count() > 0);
}

#[test]
fn test_create_string() {
    assert_eq!(Record::create_string(&vec![0, 1, 5], &vec![7, 2]), "#######.##.....");
    assert_eq!(Record::create_string(&vec![0, 6, 0], &vec![7, 2]), "#######......##");
    assert_eq!(Record::create_string(&vec![2, 3, 1], &vec![7, 2]), "..#######...##.");
}

#[test]
fn test_compare_string_against_state() {
    let r = Record::build("??????#???????? 7,2");
    assert!(r.compare_string_against_state("#######.##....."));
    assert!(!r.compare_string_against_state(".##.....#######"));
}

#[test]
fn test_t() {
    Record::build("??????#???????? 7,2").t();
    assert!(false);
}
fn cartesian_product() {
    let v: Vec<_> = 
    // (0..6).combinations_with_replacement(3)
    // (0..6).permutations(3)
    itertools::repeat_n((0..=6), 3).multi_cartesian_product()
    .filter(|comb| comb[1..comb.len()-1].iter().any(|n| *n != 0))
    .filter(|comb| comb.iter().sum::<usize>() == 6)
    .collect();
    println!("{}: {:?}", v.len(), v);
    assert!(false);

// itertools::assert_equal(it, vec![
//     vec![1, 1],
//     vec![1, 2],
//     vec![1, 3],
//     vec![2, 2],
//     vec![2, 3],
//     vec![3, 3],
// ]);
}

fn sum_of_arrangements(records: &Vec<Record>) -> usize {
    records.iter().map(Record::arrangements_count_2).sum()
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

fn main() {
    let stdin = io::stdin();

    let records: Vec<Record> = build_records(&mut stdin.lock());

    // Record::build("??????#???????? 7,2").t();
    // for r in &records {
    //     println!("{}: {}/{}", r.to_string(), r.damaged_count(), r.spring_count());
    // }
    println!("Part 1: {}", sum_of_arrangements(&records));
}

#[test]
fn test_sum() {}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let records: Vec<Record> = build_records(&mut reader);
    sum_of_arrangements(&records)
}

// #[test]
fn test_part1() {
    assert_eq!(part1("resources/input_test1"), 6);
    assert_eq!(part1("resources/input_test2"), 21);
}
