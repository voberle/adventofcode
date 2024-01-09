use std::io::{self, Read};

fn is_abba(s: &[char]) -> bool {
    assert_eq!(s.len(), 4);
    s[0] == s[3] && s[1] == s[2] && s[0] != s[1]
}

fn is_aba(s: &[char]) -> bool {
    assert_eq!(s.len(), 3);
    s[0] == s[2] && s[0] != s[1]
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

    // Returns all the ABA, if there is any.
    fn get_aba(&self) -> Vec<Vec<char>> {
        let v = match self {
            Part::External(v) | Part::Internal(v) => v,
        };
        if v.len() < 3 {
            return Vec::new();
        }
        let mut res: Vec<Vec<char>> = Vec::new();
        for i in 0..v.len() - 2 {
            if is_aba(&[v[i], v[i + 1], v[i + 2]]) {
                res.push(v[i..=i + 2].to_vec());
            }
        }
        res
    }

    fn contains_aba_as_bab(&self, bab: &[char]) -> bool {
        let v = match self {
            Part::External(v) | Part::Internal(v) => v,
        };
        if v.len() < 3 {
            return false;
        }
        (0..v.len() - 2).any(|i| v[i] == bab[1] && v[i + 1] == bab[0] && v[i + 2] == bab[1])
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

fn support_ssl(ip: &[Part]) -> bool {
    for aba in ip
        .iter()
        .filter(|p| matches!(p, Part::External(_)))
        .flat_map(Part::get_aba)
    {
        if ip
            .iter()
            .filter(|p| matches!(p, Part::Internal(_)))
            .any(|p| p.contains_aba_as_bab(&aba))
        {
            return true;
        }
    }
    false
}

fn support_count(ip_list: &[Vec<Part>], support: fn(&[Part]) -> bool) -> usize {
    ip_list.iter().filter(|ip| support(ip)).count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ip_list = build(&input);

    println!("Part 1: {}", support_count(&ip_list, support_tls));
    println!("Part 2: {}", support_count(&ip_list, support_ssl));
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
    fn test_is_aba() {
        assert_eq!(is_aba(&['a', 'b', 'a']), true);
        assert_eq!(is_aba(&['a', 'a', 'a']), false);
    }

    #[test]
    fn test_aba_bab() {
        let e = Part::External(vec!['a', 'b', 'a']);
        assert_eq!(e.get_aba(), vec![vec!['a', 'b', 'a']]);
        let i = Part::Internal(vec!['b', 'a', 'b']);
        assert_eq!(i.contains_aba_as_bab(&['a', 'b', 'a']), true);
    }

    #[test]
    fn test_support_ssl() {
        assert_eq!(support_ssl(&build_parts("aba[bab]xyz")), true);
        assert_eq!(support_ssl(&build_parts("xyx[xyx]xyx")), false);
        assert_eq!(support_ssl(&build_parts("aaa[kek]eke")), true);
        assert_eq!(support_ssl(&build_parts("zazbz[bzb]cdb")), true);
    }
}
