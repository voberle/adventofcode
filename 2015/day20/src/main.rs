use std::io::{self, Read};

fn house_gets_presents_from_elf(house_nb: usize, elf_nb: usize) -> bool {
    house_nb % elf_nb == 0
}

fn house_presents_count<const MULT: usize>(house_nb: usize) -> usize {
    // we can only do until half and add house nb at the end
    (1..=house_nb / 2)
        .filter_map(|elf| {
            if house_gets_presents_from_elf(house_nb, elf) {
                Some(elf * MULT)
            } else {
                None
            }
        })
        .sum::<usize>()
        + house_nb * MULT
    // if house_nb % 10_000 == 0 { println!("House {house_nb}: {c}"); }
}

#[allow(clippy::maybe_infinite_iter)]
fn lowest_house_to_get(presents_count: usize) -> usize {
    let present_count_adjusted = presents_count / 10;
    (1..)
        .find(|i| {
            let house = *i;
            // we can only look at even numbers, odd ones will be smaller
            // if house % 2 == 1 {
            //     return false;
            // }
            house_presents_count::<1>(house) >= present_count_adjusted
        })
        .unwrap()
}

// Alternative algo found on Reddit
// https://www.reddit.com/r/adventofcode/comments/3xjpp2/comment/cy59zd9/?utm_source=reddit&utm_medium=web2x&context=3
#[allow(dead_code)]
fn lowest_house_to_get_alt(n: usize) -> usize {
    let n1 = n / 10;
    let mut houses = vec![0; n1];
    for elf in 1..=n1 {
        for h in (elf..n1).step_by(elf) {
            houses[h] += elf * 10;
        }
    }
    houses.iter().enumerate().find(|(_, v)| **v >= n).unwrap().0
}

fn house_presents_count_lazy_elfs<const MULT: usize>(
    house_nb: usize,
    elves: &mut [usize],
) -> usize {
    (1..=house_nb)
        .filter_map(|elf| {
            if elves[elf - 1] <= 50 && house_gets_presents_from_elf(house_nb, elf) {
                elves[elf - 1] += 1;
                Some(elf * MULT)
            } else {
                None
            }
        })
        .sum::<usize>()
    // if house_nb % 10_000 == 0 { println!("House {house_nb}: {c}"); }
}

#[allow(clippy::maybe_infinite_iter)]
fn lowest_house_to_get_lazy_elfs(presents_count: usize) -> usize {
    // Tracks how many presents each elf delivered.
    let mut elves: Vec<usize> = Vec::new();
    (1..)
        .find(|i| {
            let house = *i;
            elves.push(0);
            house_presents_count_lazy_elfs::<11>(house, &mut elves) >= presents_count
        })
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let presents_count = input.trim().parse().unwrap();

    // println!("Part 1 alt: {}", lowest_house_to_get_alt(presents_count));

    println!("Part 1: {}", lowest_house_to_get(presents_count));
    println!("Part 2: {}", lowest_house_to_get_lazy_elfs(presents_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_got() {
        assert_eq!(house_presents_count::<10>(1), 10);
        assert_eq!(house_presents_count::<10>(2), 30);
        assert_eq!(house_presents_count::<10>(3), 40);
        assert_eq!(house_presents_count::<10>(4), 70);
        assert_eq!(house_presents_count::<10>(5), 60);
        assert_eq!(house_presents_count::<10>(6), 120);
        assert_eq!(house_presents_count::<10>(7), 80);
        assert_eq!(house_presents_count::<10>(8), 150);
        assert_eq!(house_presents_count::<10>(9), 130);
    }

    #[test]
    fn test_part1() {
        assert_eq!(lowest_house_to_get(40), 3);
        assert_eq!(lowest_house_to_get(50), 4);
        assert_eq!(lowest_house_to_get(100), 6);
    }

    #[test]
    fn test_part2() {
        // House 1 got 11 presents.
        // House 2 got 11 + 22 = 33 presents.
        // House 3 got 11 + 33 = 44 presents.
        // House 4 got 11 + 22 + 44 = 77 presents.
        assert_eq!(lowest_house_to_get_lazy_elfs(70), 4);
    }
}
