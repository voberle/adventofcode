// For each position in the burrow, returns the next possible positions and the distance to it.
//
// #################
// #01. 2. 3. 4. 56#
// ###7 #8 #9 #10###
//   #11#12#13#14#
//   #############
//
// #################
// #01. 2. 3. 4. 56#
// ###7 #8 #9 #10###
//   #11#12#13#14#
//   #15#16#17#18#
//   #19#20#21#22#
//   #############
pub fn get_next_possible_positions(burrow: &[Option<char>], pos: usize) -> Vec<(usize, u32)> {
    let amphipod = burrow[pos].expect("Position must be occupied");
    next_positions(burrow, pos, amphipod)
}

#[allow(clippy::too_many_lines)]
fn next_positions(burrow: &[Option<char>], pos: usize, amphipod: char) -> Vec<(usize, u32)> {
    let mut next_pos: Vec<(usize, u32)> = Vec::new();
    match pos {
        // 0-6: All the hallway positions.
        // We can only go down into rooms from there, and we can only to our final destination.
        0 => {
            pass_by(burrow, amphipod, &mut next_pos, 1);
        }
        1 => {
            // Right of 1
            add_down(burrow, amphipod, &mut next_pos, 7, 2);
            if is_free(burrow, 2) {
                // Right of 2
                add_down(burrow, amphipod, &mut next_pos, 8, 4);
                if is_free(burrow, 3) {
                    add_down(burrow, amphipod, &mut next_pos, 9, 6);
                    if is_free(burrow, 4) {
                        add_down(burrow, amphipod, &mut next_pos, 10, 8);
                    }
                }
            }
        }
        2 => {
            // Left of 2
            add_down(burrow, amphipod, &mut next_pos, 7, 2);
            // Right of 2
            add_down(burrow, amphipod, &mut next_pos, 8, 2);
            if is_free(burrow, 3) {
                add_down(burrow, amphipod, &mut next_pos, 9, 4);
                if is_free(burrow, 4) {
                    add_down(burrow, amphipod, &mut next_pos, 10, 6);
                }
            }
        }
        3 => {
            // Left of 3
            add_down(burrow, amphipod, &mut next_pos, 8, 2);
            if is_free(burrow, 2) {
                add_down(burrow, amphipod, &mut next_pos, 7, 4);
            }
            // Right of 3
            add_down(burrow, amphipod, &mut next_pos, 9, 2);
            if is_free(burrow, 4) {
                add_down(burrow, amphipod, &mut next_pos, 10, 4);
            }
        }
        4 => {
            // Left of 4
            add_down(burrow, amphipod, &mut next_pos, 9, 2);
            if is_free(burrow, 3) {
                add_down(burrow, amphipod, &mut next_pos, 8, 4);
                if is_free(burrow, 2) {
                    add_down(burrow, amphipod, &mut next_pos, 7, 6);
                }
            }
            // Right of 4
            add_down(burrow, amphipod, &mut next_pos, 10, 2);
        }
        5 => {
            // Left of 5
            add_down(burrow, amphipod, &mut next_pos, 10, 2);
            if is_free(burrow, 4) {
                add_down(burrow, amphipod, &mut next_pos, 9, 4);
                if is_free(burrow, 3) {
                    add_down(burrow, amphipod, &mut next_pos, 8, 6);
                    if is_free(burrow, 2) {
                        add_down(burrow, amphipod, &mut next_pos, 7, 8);
                    }
                }
            }
        }
        6 => {
            pass_by(burrow, amphipod, &mut next_pos, 5);
        }
        // 7-14: All the rooms, we can only go to the hallway.
        // TODO: Another possible optimization is to not allow to move again from a room, once we already moved into one.
        7 => {
            // Left
            if is_free(burrow, 1) {
                next_pos.push((1, 2));
                if is_free(burrow, 0) {
                    next_pos.push((0, 3));
                }
            }
            // Right
            if is_free(burrow, 2) {
                next_pos.push((2, 2));
                if is_free(burrow, 3) {
                    next_pos.push((3, 4));
                    if is_free(burrow, 4) {
                        next_pos.push((4, 6));
                        if is_free(burrow, 5) {
                            next_pos.push((5, 8));
                            if is_free(burrow, 6) {
                                next_pos.push((6, 9));
                            }
                        }
                    }
                }
            }
        }
        8 => {
            // Left
            if is_free(burrow, 2) {
                next_pos.push((2, 2));
                if is_free(burrow, 1) {
                    next_pos.push((1, 4));
                    if is_free(burrow, 0) {
                        next_pos.push((0, 5));
                    }
                }
            }
            // Right
            if is_free(burrow, 3) {
                next_pos.push((3, 2));
                if is_free(burrow, 4) {
                    next_pos.push((4, 4));
                    if is_free(burrow, 5) {
                        next_pos.push((5, 6));
                        if is_free(burrow, 6) {
                            next_pos.push((6, 7));
                        }
                    }
                }
            }
        }
        9 => {
            // Left
            if is_free(burrow, 3) {
                next_pos.push((3, 2));
                if is_free(burrow, 2) {
                    next_pos.push((2, 4));
                    if is_free(burrow, 1) {
                        next_pos.push((1, 6));
                        if is_free(burrow, 0) {
                            next_pos.push((0, 7));
                        }
                    }
                }
            }
            // Right
            if is_free(burrow, 4) {
                next_pos.push((4, 2));
                if is_free(burrow, 5) {
                    next_pos.push((5, 4));
                    if is_free(burrow, 6) {
                        next_pos.push((6, 5));
                    }
                }
            }
        }
        10 => {
            // Left
            if is_free(burrow, 4) {
                next_pos.push((4, 2));
                if is_free(burrow, 3) {
                    next_pos.push((3, 4));
                    if is_free(burrow, 2) {
                        next_pos.push((2, 6));
                        if is_free(burrow, 1) {
                            next_pos.push((1, 8));
                            if is_free(burrow, 0) {
                                next_pos.push((0, 9));
                            }
                        }
                    }
                }
            }
            // Right
            if is_free(burrow, 5) {
                next_pos.push((5, 2));
                if is_free(burrow, 6) {
                    next_pos.push((6, 3));
                }
            }
        }
        // Level 2
        11 => {
            pass_by(burrow, amphipod, &mut next_pos, 7);
        }
        12 => {
            pass_by(burrow, amphipod, &mut next_pos, 8);
        }
        13 => {
            pass_by(burrow, amphipod, &mut next_pos, 9);
        }
        14 => {
            pass_by(burrow, amphipod, &mut next_pos, 10);
        }
        // Level 3
        15 => {
            pass_by(burrow, amphipod, &mut next_pos, 11);
        }
        16 => {
            pass_by(burrow, amphipod, &mut next_pos, 12);
        }
        17 => {
            pass_by(burrow, amphipod, &mut next_pos, 13);
        }
        18 => {
            pass_by(burrow, amphipod, &mut next_pos, 14);
        }
        // Level 4
        19 => {
            // If bottom is already correct, don't move it.
            if amphipod != 'A' {
                pass_by(burrow, amphipod, &mut next_pos, 15);
            }
        }
        20 => {
            if amphipod != 'B' {
                pass_by(burrow, amphipod, &mut next_pos, 16);
            }
        }
        21 => {
            if amphipod != 'C' {
                pass_by(burrow, amphipod, &mut next_pos, 17);
            }
        }
        22 => {
            if amphipod != 'D' {
                pass_by(burrow, amphipod, &mut next_pos, 18);
            }
        }
        _ => panic!("Invalid position"),
    }
    next_pos
}

fn is_free(burrow: &[Option<char>], pos: usize) -> bool {
    burrow[pos].is_none()
}

fn add_down(
    burrow: &[Option<char>],
    amphipod: char,
    next_pos: &mut Vec<(usize, u32)>,
    level1: usize,
    dist_to_level1: u32,
) {
    // We can only go to our final room.
    match level1 {
        7 => {
            if amphipod != 'A' {
                return;
            }
        }
        8 => {
            if amphipod != 'B' {
                return;
            }
        }
        9 => {
            if amphipod != 'C' {
                return;
            }
        }
        10 => {
            if amphipod != 'D' {
                return;
            }
        }
        _ => panic!("Unsupported level 1"),
    }

    let level2 = level1 + 4;
    let level3 = level2 + 4;
    let level4 = level3 + 4;

    // We only move in a room if it contains only our own amphipods.
    if burrow[level1].is_some_and(|a| a != amphipod)
        || burrow[level2].is_some_and(|a| a != amphipod)
    {
        return;
    }
    if burrow.len() > 15
        && (burrow[level3].is_some_and(|a| a != amphipod)
            || burrow[level4].is_some_and(|a| a != amphipod))
    {
        return;
    }

    if is_free(burrow, level1) {
        next_pos.push((level1, dist_to_level1));
        if is_free(burrow, level2) {
            next_pos.push((level2, dist_to_level1 + 1));
            if burrow.len() == 15 {
                return;
            }
            if is_free(burrow, level3) {
                next_pos.push((level3, dist_to_level1 + 2));
                if is_free(burrow, level4) {
                    next_pos.push((level4, dist_to_level1 + 3));
                }
            }
        }
    }
}

fn pass_by(burrow: &[Option<char>], amphipod: char, next_pos: &mut Vec<(usize, u32)>, pos: usize) {
    if is_free(burrow, pos) {
        next_pos.extend(
            next_positions(burrow, pos, amphipod)
                .iter()
                .map(|(p, c)| (*p, c + 1)),
        );
    }
}
