use std::io::{self, Read};

fn hash(salt: &str, i: usize) -> Vec<char> {
    let digest = md5::compute(format!("{}{}", salt, i).as_bytes());
    format!("{:x}", digest).chars().collect()
}

fn has_three_char_in_row(s: &[char]) -> Option<char> {
    for i in 0..s.len() - 2 {
        if s[i] == s[i + 1] && s[i] == s[i + 2] {
            return Some(s[i]);
        }
    }
    None
}

fn contains_char_times_five(s: &[char], c: char) -> bool {
    for i in 0..s.len() - 4 {
        if s[i] == c && s[i] == s[i + 1] && s[i] == s[i + 2] && s[i] == s[i + 3] && s[i] == s[i + 4]
        {
            return true;
        }
    }
    false
}

// Returns the hash for the corresponding index, caching them in `cache`.
fn hash_cached<'a>(salt: &str, index: usize, cache: &'a mut Vec<Vec<char>>) -> &'a Vec<char> {
    if index < cache.len() {
        return &cache[index];
    }
    (cache.len()..=index).for_each(|i| {
        let hash = hash(salt, i);
        cache.push(hash);
    });
    &cache[index]
}

fn index_of_nth_key<const NTH: usize>(salt: &str) -> usize {
    let mut hashes: Vec<Vec<char>> = Vec::new();
    let mut keys_found = 0;
    let mut index = 0;
    while keys_found != NTH {
        let hash = hash_cached(salt, index, &mut hashes);
        if let Some(triple) = has_three_char_in_row(hash) {
            // check if next 1000 hashes contain the triple 5 times
            let has_triple_5_times = (index + 1..=index + 1000).any(|i| {
                let h = hash_cached(salt, i, &mut hashes);
                contains_char_times_five(h, triple)
            });
            if has_triple_5_times {
                keys_found += 1;
            }
        }
        index += 1;
    }
    index - 1
}

fn part2(salt: &str) -> usize {
    0
}

fn main() {
    let mut salt = String::new();
    io::stdin().read_to_string(&mut salt).unwrap();

    println!("Part 1: {}", index_of_nth_key::<64>(salt.trim()));
    println!("Part 2: {}", part2(salt.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(index_of_nth_key::<64>("abc"), 22728);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("abc"), 0);
    }
}
