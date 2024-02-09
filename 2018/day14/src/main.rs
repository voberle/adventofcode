use std::{
    io::{self, Read},
    usize,
};

fn get_digits(n: usize) -> Vec<usize> {
    fn inner(n: usize, xs: &mut Vec<usize>) {
        if n >= 10 {
            inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    inner(n, &mut xs);
    xs
}

fn next_ten_scores(recipes_count: usize) -> String {
    let target_count = recipes_count + 10;
    let mut recipes: Vec<usize> = Vec::with_capacity(target_count);
    recipes.push(3);
    recipes.push(7);

    let mut r1 = 0;
    let mut r2 = 1;

    while recipes.len() < target_count {
        let score1 = recipes[r1];
        let score2 = recipes[r2];
        recipes.extend(get_digits(score1 + score2));
        r1 = (r1 + 1 + score1).rem_euclid(recipes.len());
        r2 = (r2 + 1 + score2).rem_euclid(recipes.len());
    }

    // println!("{:?}", recipes);
    recipes[recipes_count..recipes_count + 10]
        .iter()
        .map(ToString::to_string)
        .collect()
}

fn part2(recipes_count: usize) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let recipes_count = input.trim().parse().unwrap();

    println!("Part 1: {}", next_ten_scores(recipes_count));
    println!("Part 2: {}", part2(recipes_count));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(next_ten_scores(9), "5158916779");
        assert_eq!(next_ten_scores(5), "0124515891");
        assert_eq!(next_ten_scores(18), "9251071085");
        assert_eq!(next_ten_scores(2018), "5941429882");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(0), 0);
    }
}
