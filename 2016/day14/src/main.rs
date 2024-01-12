use std::io::{self, Read};

fn calc_hash(salt: &str, i: usize) -> String {
    let digest = md5::compute(format!("{}{}", salt, i).as_bytes());
    format!("{:x}", digest)
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
fn get_hash<'a, const STRETCHED: bool>(
    salt: &str,
    index: usize,
    cache: &'a mut Vec<Vec<char>>,
) -> &'a Vec<char> {
    if index < cache.len() {
        return &cache[index];
    }
    // Fill the cache until the value we need
    (cache.len()..=index).for_each(|i| {
        let mut hash = calc_hash(salt, i);
        if STRETCHED {
            for _ in 0..2016 {
                hash = format!("{:x}", md5::compute(hash));
            }
        }
        cache.push(hash.chars().collect());
    });
    &cache[index]
}

fn index_of_64th_key<const STRETCHED: bool>(salt: &str) -> usize {
    let mut hashes: Vec<Vec<char>> = Vec::new();
    let mut keys_found = 0;
    let mut index = 0;
    while keys_found != 64 {
        let hash = get_hash::<STRETCHED>(salt, index, &mut hashes);
        if let Some(triple) = has_three_char_in_row(hash) {
            // check if next 1000 hashes contain the triple 5 times
            let has_triple_5_times = (index + 1..=index + 1000).any(|i| {
                let h = get_hash::<STRETCHED>(salt, i, &mut hashes);
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

fn main() {
    let mut salt = String::new();
    io::stdin().read_to_string(&mut salt).unwrap();

    println!("Part 1: {}", index_of_64th_key::<false>(salt.trim()));
    println!("Part 2: {}", index_of_64th_key::<true>(salt.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(index_of_64th_key::<false>("abc"), 22728);
    }

    #[test]
    fn test_part2() {
        assert_eq!(index_of_64th_key::<true>("abc"), 22859);
    }
}
