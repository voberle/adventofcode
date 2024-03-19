use std::cmp::Ordering;

/// Takes a list of ranges and simplifies it into an ordered non-overlapping list.
/// Works only with ranges that have inclusive start and exclusive end.
fn simplify_ranges<T>(ranges: &[(T, T)]) -> Vec<(T, T)>
where
    T: PartialOrd + Copy,
{
    // Algorith from https://cs.stackexchange.com/a/106978
    // List of beginning and ending points. The boolean tells the type (true is start, false is end).
    let mut positions: Vec<(bool, T)> = ranges
        .iter()
        .flat_map(|r| [(true, r.0), (false, r.1)])
        .collect();

    // Sort by position, with starting points comparing below ending points when positions are equal.
    positions.sort_by(|a, b| {
        if a.1 == b.1 {
            if a.0 && !b.0 {
                // a is start, b is end
                Ordering::Less
            } else if !a.0 && b.0 {
                // a is end, b is start
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    });
    assert!(positions.len() > 1 && positions.first().unwrap().0);

    let mut simplified_ranges = Vec::new();
    let mut start: T = positions.first().unwrap().1; // we need a value to initialize start with.
    let mut c = 0;
    for (pos_type, pos) in positions {
        if pos_type {
            // 0->1 transition
            if c == 0 {
                start = pos;
            }
            c += 1;
        } else {
            // 1->0 transition
            if c == 1 {
                let end = pos;
                simplified_ranges.push((start, end));
            }
            c -= 1;
        }
    }
    simplified_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_ranges() {
        let ranges = [(5, 9), (0, 3), (4, 8)];
        assert_eq!(simplify_ranges(&ranges), &[(0, 3), (4, 9)]);
    }
}
