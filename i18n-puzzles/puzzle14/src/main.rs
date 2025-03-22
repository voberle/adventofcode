use std::io::{self, Read};

use itertools::Itertools;

fn to_digit(c: char) -> u64 {
    match c {
        '一' => 1,
        '二' => 2,
        '三' => 3,
        '四' => 4,
        '五' => 5,
        '六' => 6,
        '七' => 7,
        '八' => 8,
        '九' => 9,
        _ => panic!("Invalid digit"),
    }
}

struct Number(Vec<char>);

impl Number {
    // Convert the list of Japanese characters into the metric system number.
    #[allow(clippy::cast_possible_truncation)]
    fn metric(&self) -> u64 {
        // Function to handle a sub-part of a myriad part (see below).
        fn sub_part(power_of_ten_char: char, myriad: &&[char], pos: &mut usize) -> Vec<char> {
            if let Some(power_of_ten_pos) = myriad.iter().position(|&c| c == power_of_ten_char) {
                let s = &myriad[*pos..power_of_ten_pos];
                *pos = power_of_ten_pos + 1;

                if s.is_empty() {
                    // If the power of ten character is alone, it means we just have 1 x that.
                    vec!['一']
                } else {
                    s.to_vec()
                }
            } else {
                Vec::new()
            }
        }

        // Split the number into myriads (ten thousands).
        let myriads: Vec<&[char]> = self.0.split(|c| ['億', '万'].contains(c)).collect();

        // We don't support infinite numbers.
        assert!(myriads.len() <= 3);

        myriads
            .iter()
            .rev()
            .enumerate()
            .map(|(power, myriad)| {
                // We need to divide the myriad into 4 sub-parts, for the 1000, 100, 10 and 1.
                // Some parts can be empty, that's why we handle each power of ten char separately.
                let mut subs = Vec::with_capacity(4);

                let mut pos = 0;
                subs.push(sub_part('千', myriad, &mut pos)); // 1000s
                subs.push(sub_part('百', myriad, &mut pos)); // 100s
                subs.push(sub_part('十', myriad, &mut pos)); // 10s
                // If there is something left, it's the 1s.
                subs.push(if pos < myriad.len() {
                    myriad[pos..].to_vec()
                } else {
                    Vec::new()
                });

                // Combine the parts into the value.
                let myriad_result: u64 = subs
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(sub_power, s)| {
                        assert!(s.len() <= 1);

                        if let Some(c) = s.first() {
                            let d = to_digit(*c);
                            d * 10_u64.pow(sub_power as u32)
                        } else {
                            0
                        }
                    })
                    .sum();

                myriad_result * 10_000_u64.pow(power as u32)
            })
            .sum()
    }
}

impl From<&str> for Number {
    fn from(s: &str) -> Self {
        Number(s.chars().collect())
    }
}

struct Unit(char);

impl Unit {
    const MO_SHAKU_RATIO: u64 = 10_000;

    // Returns the unit normalized into Mo.
    // Mo is the smallest unit. This allows us to use only integers.
    fn as_mo(&self) -> u64 {
        // larger units:
        match self.0 {
            '尺' => Self::MO_SHAKU_RATIO,
            // Larger units.
            '間' => 6 * Self::MO_SHAKU_RATIO,
            '丈' => 10 * Self::MO_SHAKU_RATIO,
            '町' => 360 * Self::MO_SHAKU_RATIO,
            '里' => 12960 * Self::MO_SHAKU_RATIO,
            // Smaller units.
            '毛' => 1,
            '厘' => Self::MO_SHAKU_RATIO / 1000,
            '分' => Self::MO_SHAKU_RATIO / 100,
            '寸' => Self::MO_SHAKU_RATIO / 10,
            _ => panic!("Unknown unit"),
        }
    }
}

// Represents a number + unit.
struct Dimension {
    number: Number,
    unit: Unit,
}

impl Dimension {
    fn build(s: &str) -> Self {
        let mut number: Vec<char> = s.chars().collect();
        let unit = number.pop().unwrap();
        Self {
            number: Number(number),
            unit: Unit(unit),
        }
    }

    // Returns the length of the dimension in Mos.
    fn length_as_mo(&self) -> u64 {
        self.number.metric() * self.unit.as_mo()
    }
}

// Builds an array of two dimensions (so an area).
fn build_area(line: &str) -> [Dimension; 2] {
    line.split(" × ")
        .map(Dimension::build)
        .collect_array()
        .unwrap()
}

fn build(input: &str) -> Vec<[Dimension; 2]> {
    input.lines().map(build_area).collect()
}

// Calculates the area size in square meters.
fn area_m2(dims: &[Dimension; 2]) -> u64 {
    const MO_SHAKU_RATIO: u128 = Unit::MO_SHAKU_RATIO as u128;

    let d0 = dims[0].length_as_mo();
    let d1 = dims[1].length_as_mo();

    // Since we are dealing in "Mo units", the square meters become too big for u64.
    let mo_sq = u128::from(d0) * u128::from(d1);

    u64::try_from(mo_sq * 10 * 10 / MO_SHAKU_RATIO / MO_SHAKU_RATIO / 33 / 33).unwrap()
}

fn total_area(land_registry: &[[Dimension; 2]]) -> u64 {
    land_registry.iter().map(area_m2).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let land_registry = build(&input);

    println!("Answer: {}", total_area(&land_registry));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_numbers() {
        assert_eq!(Number::from("三百").metric(), 300);
        assert_eq!(Number::from("三百二十一").metric(), 321);
        assert_eq!(Number::from("四千").metric(), 4_000);
        assert_eq!(Number::from("五万").metric(), 50_000);
        assert_eq!(Number::from("九万九千九百九十九").metric(), 99_999);
        assert_eq!(Number::from("四十二万四十二").metric(), 420_042);
        assert_eq!(
            Number::from("九億八千七百六十五万四千三百二十一").metric(),
            987_654_321
        );
    }

    #[test]
    fn test_area_meter() {
        let area = build_area("二百四十二町 × 三百五十一丈");
        assert_eq!(area_m2(&area), 28080000); // = 242 Cho × 351 Jo = 28080000 m²

        let area = build_area("七十八寸 × 二十一万七千八百厘");
        assert_eq!(area_m2(&area), 156); // = 78 Sun × 217800 Rin = 156 m²

        let area = build_area("七万二千三百五十八町 × 六百十二分");
        assert_eq!(area_m2(&area), 14639040); // = 72358 Cho × 612 Bu = 14639040 m²

        let area = build_area("六寸 × 三十万七千九十八尺");
        assert_eq!(area_m2(&area), 16920); // = 6 Sun × 307098 Shaku = 16920 m²

        let area = build_area("九間 × 三万三千百五十四里");
        assert_eq!(area_m2(&area), 2130624000); // = 9 Ken × 33154 Ri = 2130624000 m²

        let area = build_area("六百毛 × 七百四十四万千五百厘");
        assert_eq!(area_m2(&area), 41); // = 600 Mo × 7441500 Rin = 41 m²

        let area = build_area("七十八億二千八十三万五千毛 × 二十八万八千六百毛");
        assert_eq!(area_m2(&area), 2072629); // = 7820835000 Mo × 288600 Mo = 2072629 m²

        let area = build_area("三百七十四万二千五百三十厘 × 六百七十一万七千厘");
        assert_eq!(area_m2(&area), 2308409); // = 3742530 Rin × 6717000 Rin = 2308409 m²
    }

    #[test]
    fn test_answer() {
        let land_registry = build(INPUT_TEST);
        assert_eq!(total_area(&land_registry), 2177741195);
    }
}
