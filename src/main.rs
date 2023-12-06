// https://adventofcode.com/2023/day/6
// Part 1 test: 288

static INPUT_TEST: &str = "\
Time:      7  15   30
Distance:  9  40  200\
";

static INPUT: &str = "\
Time:        58     81     96     76
Distance:   434   1041   2219   1218\
";

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }

    fn traveled(&self, hold: u32) -> u32 {
        let time_for_moving = self.time - hold;
        let speed = hold;
        time_for_moving * speed
    }

    fn count_ways_to_win(&self) -> u32 {
        let mut c = 0;
        for h in 0..self.time {
            let t = self.traveled(h);
            if t > self.distance {
                c += 1;
            }
        }
        c
    }
}

#[test]
fn check_races() {
    let race = Race::new(7, 9);
    assert_eq!(race.traveled(0), 0);
    assert_eq!(race.traveled(1), 6);
    assert_eq!(race.traveled(2), 10);
    assert_eq!(race.traveled(3), 12);
    assert_eq!(race.traveled(4), 12);
    assert_eq!(race.traveled(5), 10);
    assert_eq!(race.traveled(6), 6);
    assert_eq!(race.traveled(7), 0);
    assert_eq!(race.count_ways_to_win(), 4);

    assert_eq!(Race::new(15, 40).count_ways_to_win(), 8);
    assert_eq!(Race::new(30, 200).count_ways_to_win(), 9);
}

fn find_nb_ways(input: &str) -> u32 {
    let mut it = input.lines();
    let games: Vec<Race> = it
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .zip(
            it.next()
                .unwrap()
                .strip_prefix("Distance: ")
                .unwrap()
                .split_whitespace()
                .map(|i| i.parse().unwrap()),
        )
        .map(|(t, d)| Race::new(t, d))
        .collect();
    //println!("{:#?}", games);
    
    games.iter().map(Race::count_ways_to_win).fold(1, |n, i| n * i)
}

fn check_part1() {
    assert_eq!(find_nb_ways(INPUT_TEST), 288);
}

fn main() {
    println!("Part 1: {}", find_nb_ways(INPUT));
}
