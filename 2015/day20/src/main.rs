use std::io::{self, Read};

fn house_gets_presents_from_elf(house_nb: u32, elf_nb: u32) -> bool {
    house_nb % elf_nb == 0
}

fn house_presents_count<const MULT: u32>(house_nb: u32) -> u32 {
    // we can only do until half and add house nb at the end
    let c = (1..=house_nb / 2)
        .filter_map(|elf| {
            if house_gets_presents_from_elf(house_nb, elf) {
                Some(elf * MULT)
            } else {
                None
            }
        })
        .sum::<u32>()
        + house_nb * MULT;
    // if house_nb % 10_000 == 0 { println!("House {house_nb}: {c}"); }
    c
}

fn lowest_house_to_get(presents_count: u32) -> u32 {
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

fn part2(input: &str) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!(
        "Part 1: {}",
        lowest_house_to_get(input.trim().parse().unwrap())
    );
    println!("Part 2: {}", part2(&input));
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
        assert_eq!(part2(""), 0);
    }
}
