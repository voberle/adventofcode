use std::io::{self, Read};

fn is_abba(s: &[char]) -> bool {
    assert_eq!(s.len(), 4);
    s[0] == s[3] && s[1] == s[2] && s[0] != s[1]
}

#[derive(Debug, PartialEq)]
enum Part {
    External(Vec<char>),
    Internal(Vec<char>),
}

impl Part {
    fn external() -> Self {
        Self::External(Vec::new())
    }

    fn internal() -> Self {
        Self::Internal(Vec::new())
    }

    fn push(&mut self, c: char) {
        match self {
            Part::External(v) | Part::Internal(v) => v.push(c),
        }
    }

    // Checks if the corresponding vector of chars contains an ABBA string.
    fn is_abba(&self) -> bool {
        let v = match self {
            Part::External(v) | Part::Internal(v) => v,
        };
        if v.len() < 4 {
            return false;
        }
        (0..v.len() - 3).any(|i| is_abba(&[v[i], v[i + 1], v[i + 2], v[i + 3]]))
    }
}

fn build_parts(line: &str) -> Vec<Part> {
    // Lines never start or end with a bracket, using that assumption.
    assert!(!line.starts_with('['));
    assert!(!line.ends_with(']'));

    let mut parts: Vec<Part> = Vec::new();
    let mut p = Part::external();
    for c in line.chars() {
        if c == '[' {
            parts.push(p);
            p = Part::internal();
        } else if c == ']' {
            parts.push(p);
            p = Part::external();
        } else {
            p.push(c);
        }
    }
    parts.push(p);
    parts
}

fn build(input: &str) -> Vec<Vec<Part>> {
    input.lines().map(build_parts).collect()
}

fn support_tls(ip: &[Part]) -> bool {
    let at_least_one_abba_in_external = ip
        .iter()
        .filter(|p| matches!(p, Part::External(_)))
        .any(Part::is_abba);
    let no_abba_in_internal = ip
        .iter()
        .filter(|p| matches!(p, Part::Internal(_)))
        .all(|p| !p.is_abba());
    at_least_one_abba_in_external && no_abba_in_internal
}

fn support_tls_count(ip_list: &[Vec<Part>]) -> usize {
    ip_list.iter().filter(|ip| support_tls(ip)).count()
}

fn part2(ip_list: &[Vec<Part>]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ip_list = build(&input);

    println!("Part 1: {}", support_tls_count(&ip_list));
    println!("Part 2: {}", part2(&ip_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abba() {
        assert_eq!(is_abba(&['a', 'b', 'b', 'a']), true);
        assert_eq!(is_abba(&['a', 'a', 'a', 'a']), false);
    }

    #[test]
    fn test_support_tls() {
        assert_eq!(support_tls(&build_parts("abba[mnop]qrst")), true);
        assert_eq!(support_tls(&build_parts("abcd[bddb]xyyx")), false);
        assert_eq!(support_tls(&build_parts("aaaa[qwer]tyui")), false);
        assert_eq!(support_tls(&build_parts("ioxxoj[asdfgh]zxcvbn")), true);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build("")), 0);
    }
}
