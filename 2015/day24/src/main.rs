use std::io::{self, Read};

use itertools::Itertools;

fn build(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

// Verify the assumptions we made about the input.
fn assert_real_input_assumptions(weights: &[u32]) {
    // The weight list is ordered in ascending way
    let mut iter = weights.iter().peekable();
    while let Some(w) = iter.next() {
        if let Some(n) = iter.peek() {
            assert!(w < n);
        }
    }

    // All weights are unique
    assert_eq!(weights.iter().unique().count(), weights.len());

    // Note that it seems all weights are odd, but it's not the case: There is 2 at the beginning of the list.
}

// Get the weight each group must have
fn get_group_weight<const GROUP_COUNT: usize>(weights: &[u32]) -> u32 {
    let total_weight = weights.iter().sum::<u32>();
    assert!(total_weight % GROUP_COUNT as u32 == 0);
    total_weight / GROUP_COUNT as u32
}

// Returns the quantum entanglement aka the product of all the weights.
fn get_qe(weights: &[u32]) -> u64 {
    weights.iter().map(|e| *e as u64).product::<u64>()
}

// Find what is the minimum size the group 1 can have just based on the group weights, not trying to divide them.
// This allows us to know where to start searching and ignore the smaller group sizes that are impossible anyway.
fn get_group1_min_possible_size(weights: &[u32], group_weight: u32) -> usize {
    let mut sum = 0;
    let mut elt_cnt = 0;
    for w in weights.iter().rev() {
        if sum > group_weight {
            break;
        }
        sum += w;
        elt_cnt += 1;
    }
    elt_cnt
}

// How many possible variations are there for the group 1 for a specific group size and required sum
#[allow(dead_code)]
fn group_1_permutations_count(weights: &[u32], group_size: usize, required_sum: u32) -> usize {
    weights
        .iter()
        .permutations(group_size)
        .filter(|v| v.iter().copied().sum::<u32>() == required_sum)
        .count()
}

// Get the valid group 1 options: Their sum matches, and they are ordered by their product (aka quantum entanglement).
fn ordered_valid_groups(weights: &[u32], group_size: usize, required_sum: u32) -> Vec<Vec<u32>> {
    weights
        .iter()
        .permutations(group_size)
        .filter(|v| v.iter().copied().sum::<u32>() == required_sum)
        .map(|v| {
            // not using get_qe as params are not exactly what we need
            let p = v.iter().map(|e| **e as u64).product::<u64>();
            (v, p)
        })
        .sorted_by_key(|(_, p)| *p)
        .map(|(v, _)| v.into_iter().copied().collect())
        .collect()
}

// Create a subset of weights with the specified weights removed
fn subset(weights: &[u32], to_remove: &[u32]) -> Vec<u32> {
    weights
        .iter()
        .filter(|w| !to_remove.contains(w))
        .cloned()
        .collect()
}

// How to split a group of numbers into 2 whose sum is equal? is the Partition Problem https://en.wikipedia.org/wiki/Partition_problem
// Looked at the implementation at https://www.geeksforgeeks.org/partition-problem-dp-18/
//
// Returns true if v subsets of equal sum
#[allow(dead_code)]
fn find_partition(v: &[u32]) -> bool {
    let sum = v.iter().sum::<u32>();
    if sum % 2 == 1 {
        return false;
    }

    let rows = sum as usize / 2 + 1;
    let cols = v.len() + 1;
    let pos = |row, col| row * cols + col;

    // part[pos(i, j)] = true if a subset of [ v[0], v[1], ..v[j-1]] has sum equal to i, otherwise false
    let mut part = vec![false; rows * cols];

    // initialize top row as true
    for i in 0..cols {
        part[pos(0, i)] = true;
    }
    // initialize leftmost column, except part[0][0], as false
    for i in 1..rows {
        part[pos(i, 0)] = false;
    }
    // Fill the partition table in bottom up manner
    for i in 1..rows {
        for j in 1..cols {
            part[pos(i, j)] = part[pos(i, j - 1)];
            if i >= v[j - 1] as usize {
                part[pos(i, j)] = part[pos(i, j)] || part[pos(i - v[j - 1] as usize, j - 1)];
            }
        }
    }

    part[pos(rows - 1, cols - 1)]
}

fn find_partition_optimized(v: &[u32]) -> bool {
    // We can space optimize the above approach as for calculating the values of the current row we require only previous row
    let sum = v.iter().sum::<u32>();
    if sum % 2 == 1 {
        return false;
    }

    // part[i] = true if a sum equal to i can be reached with numbers from the set.
    let part_len = sum as usize / 2 + 1;
    let mut part = vec![false; part_len];

    // Fill the partition table in bottom up manner
    for i in 0..v.len() {
        let mut j = sum as usize / 2;
        // The element to be included in the sum cannot be greater than the sum
        while j >= v[i] as usize {
            // Check if sum - arr[i] could be formed from a subset using elements before index i
            if part[j - v[i] as usize] || j == v[i] as usize {
                part[j] = true;
            }
            j -= 1;
        }
    }

    part[part_len - 1]
}

// We only need to look at configurations that work with smallest group 1.
// The smallest group we can make needs at least 5 items, and we know how much it should weight (512).
// So we can try to find the 5 items group that weight 512, and see if the rest can be divided into 2 or 3.
//
// Find the 6 group number whose sum is 512 and with the smallest smallest quantum entanglement (product of them),
// and see if the remaining numbers can be divided into 2 or 3.
fn group1_qe<const GROUP_COUNT: usize>(weights: &[u32]) -> u64 {
    println!("Into {} groups:", GROUP_COUNT);
    assert_real_input_assumptions(weights);

    let group_weight = get_group_weight::<GROUP_COUNT>(weights);
    let min_group_1_size = get_group1_min_possible_size(weights, group_weight);
    println!(
        "Need at least {} elements in a group to reach {}",
        min_group_1_size, group_weight
    );

    // note that the max should be decreased there to a more reasonable value
    for group_1_size in min_group_1_size..weights.len() {
        let valid_group1s = ordered_valid_groups(weights, group_1_size, group_weight);
        if valid_group1s.is_empty() {
            println!("Group size {} doesn't work", group_1_size);
            continue;
        }
        // let permutations_count = group_1_permutations_count(weights, group_1_size, group_weight);
        // println!("Number of permutations for group 1 with {} elements: {}", group_1_size, permutations_count);

        for perm in valid_group1s {
            let subset = subset(weights, &perm);
            if GROUP_COUNT - 1 == 2 {
                if find_partition_optimized(&subset) {
                    // Found it!
                    return get_qe(&perm);
                }
            } else {
                panic!("Group count of {} not supported", GROUP_COUNT);
            }
        }
    }

    panic!("Didn't find any valid QE");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let weights = build(&input);

    println!("Part 1: {}", group1_qe::<3>(&weights));
    // println!("Part 2: {}", group1_qe::<4>(&weights));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_partition() {
        let v = vec![3, 1, 1, 2, 2, 1];
        assert!(find_partition(&v));
        assert!(find_partition_optimized(&v));
    }
}
