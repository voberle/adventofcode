use std::io::{self, Read};

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

fn get_digits_from_str(s: &str) -> Vec<usize> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

// Main algo, shared between part 1 and 2.
fn adding_recipes(
    recipes: &mut Vec<usize>,
    mut end_condition_fn: impl FnMut(&mut Vec<usize>) -> bool,
) {
    recipes.push(3);
    recipes.push(7);

    let mut r1 = 0;
    let mut r2 = 1;

    loop {
        let score1 = recipes[r1];
        let score2 = recipes[r2];
        recipes.extend(get_digits(score1 + score2));
        r1 = (r1 + 1 + score1).rem_euclid(recipes.len());
        r2 = (r2 + 1 + score2).rem_euclid(recipes.len());

        if end_condition_fn(recipes) {
            break;
        }
    }
}

fn next_ten_scores(recipes_count: usize) -> String {
    let target_count = recipes_count + 10;

    let mut recipes: Vec<usize> = Vec::with_capacity(target_count);

    adding_recipes(&mut recipes, |recipes| recipes.len() > target_count);

    recipes[recipes_count..recipes_count + 10]
        .iter()
        .map(ToString::to_string)
        .collect()
}

fn puzzle_input_position(puzzle_input: &str) -> usize {
    // Taking a string as input, to avoid loosing leading zeros.
    let looking_for = get_digits_from_str(puzzle_input);
    let looking_for_len = looking_for.len();

    let mut recipes: Vec<usize> = Vec::with_capacity(1_000_000);

    let mut search_idx = 0;
    let end_condition_fn = |recipes: &mut Vec<usize>| {
        let recipes_len = recipes.len();
        if recipes_len >= looking_for_len {
            while search_idx < recipes_len - looking_for_len {
                let slice_to_check = &recipes[search_idx..search_idx + looking_for_len];
                if slice_to_check == looking_for {
                    return true;
                }
                search_idx += 1;
            }
        }
        false
    };

    adding_recipes(&mut recipes, end_condition_fn);

    search_idx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", next_ten_scores(input.trim().parse().unwrap()));
    println!("Part 2: {}", puzzle_input_position(input.trim()));
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
        assert_eq!(puzzle_input_position("51589"), 9);
        assert_eq!(puzzle_input_position("01245"), 5);
        assert_eq!(puzzle_input_position("92510"), 18);
        assert_eq!(puzzle_input_position("59414"), 2018);
    }
}
