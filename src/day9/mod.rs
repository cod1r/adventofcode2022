const MAP_SIZE: usize = 1000;
#[inline(always)]
fn do_movement_p2(acc: &mut [i32; 20], map: &mut [[i32; MAP_SIZE]; MAP_SIZE], part: &mut usize) {
    for knot_idx in (0..acc.len() - 3).rev().step_by(2) {
        let (mut x1, mut y1, x2, y2): (i32, i32, i32, i32) = (
            acc[knot_idx],
            acc[knot_idx + 1],
            acc[knot_idx + 2],
            acc[knot_idx + 3],
        );
        let dist = (x1 - x2).abs() + (y1 - y2).abs();
        if dist >= 2 {
            if x1 != x2 && y1 != y2 && dist == 2 {
                continue;
            }
            if x1 < x2 {
                x1 += 1;
            } else if x1 > x2 {
                x1 -= 1;
            }

            if y1 < y2 {
                y1 += 1;
            } else if y1 > y2 {
                y1 -= 1;
            }
            while y1 < y2 - 1 && x1 == x2 {
                if map[acc[1] as usize][acc[0] as usize] != 1 {
                    *part += 1;
                    map[acc[1] as usize][acc[0] as usize] = 1;
                }
                y1 += 1;
            }
            while y1 > y2 + 1 && x1 == x2 {
                if map[acc[1] as usize][acc[0] as usize] != 1 {
                    *part += 1;
                    map[acc[1] as usize][acc[0] as usize] = 1;
                }
                y1 -= 1;
            }
            while x1 < x2 - 1 && y1 == y2 {
                if map[acc[1] as usize][acc[0] as usize] != 1 {
                    *part += 1;
                    map[acc[1] as usize][acc[0] as usize] = 1;
                }
                x1 += 1;
            }
            while x1 > x2 + 1 && y1 == y2 {
                if map[acc[1] as usize][acc[0] as usize] != 1 {
                    *part += 1;
                    map[acc[1] as usize][acc[0] as usize] = 1;
                }
                x1 -= 1;
            }
            if map[acc[1] as usize][acc[0] as usize] != 1 {
                *part += 1;
                map[acc[1] as usize][acc[0] as usize] = 1;
            }
            acc[knot_idx] = x1;
            acc[knot_idx + 1] = y1;
            assert!((x1 - x2).abs() + (y1 - y2).abs() <= 2);
        }
    }
}
pub fn day9() {
    let input_str = include_str!("input.txt");
    let mut map = [[0; MAP_SIZE]; MAP_SIZE];
    map[MAP_SIZE / 2 - 1][MAP_SIZE / 2 - 1] = 1;
    let mut part1: usize = 1;
    input_str
        .trim()
        .lines()
        .fold([MAP_SIZE as i32 / 2 - 1; 4], |mut acc, elem| {
            let parts = elem.split_once(" ").unwrap();
            let move_num = parts.1.parse::<i32>().unwrap();
            match parts.0 {
                "L" => {
                    acc[acc.len() - 2] -= move_num;
                }
                "R" => {
                    acc[acc.len() - 2] += move_num;
                }
                "U" => {
                    acc[acc.len() - 1] -= move_num;
                }
                "D" => {
                    acc[acc.len() - 1] += move_num;
                }
                _ => unreachable!(),
            }
            assert!((acc[0] - acc[2]).abs() <= 1 || (acc[1] - acc[3]).abs() <= 1);
            let manhatten_dist = (acc[0] - acc[2]).abs() + (acc[1] - acc[3]).abs();
            if manhatten_dist >= 2 {
                // for the case that the tail has to move diagonally
                if acc[0] != acc[2] && acc[1] != acc[3] && manhatten_dist > 2 {
                    if acc[0] < acc[2] {
                        acc[0] += 1;
                    } else if acc[0] > acc[2] {
                        acc[0] -= 1;
                    }
                    if acc[1] < acc[3] {
                        acc[1] += 1;
                    } else if acc[1] > acc[3] {
                        acc[1] -= 1;
                    }
                    assert!(acc[0] == acc[2] || acc[1] == acc[3]);
                }
                // this moves the tail until it touches the head
                if acc[0] == acc[2] {
                    while acc[1] < acc[3] - 1 {
                        if map[acc[1] as usize][acc[0] as usize] != 1 {
                            part1 += 1;
                            map[acc[1] as usize][acc[0] as usize] = 1;
                        }
                        acc[1] += 1;
                    }
                    while acc[3] + 1 < acc[1] {
                        if map[acc[1] as usize][acc[0] as usize] != 1 {
                            part1 += 1;
                            map[acc[1] as usize][acc[0] as usize] = 1;
                        }
                        acc[1] -= 1;
                    }
                } else if acc[1] == acc[3] {
                    while acc[0] < acc[2] - 1 {
                        if map[acc[1] as usize][acc[0] as usize] != 1 {
                            part1 += 1;
                            map[acc[1] as usize][acc[0] as usize] = 1;
                        }
                        acc[0] += 1;
                    }
                    while acc[2] + 1 < acc[0] {
                        if map[acc[1] as usize][acc[0] as usize] != 1 {
                            part1 += 1;
                            map[acc[1] as usize][acc[0] as usize] = 1;
                        }
                        acc[0] -= 1;
                    }
                }
                if map[acc[1] as usize][acc[0] as usize] != 1 {
                    part1 += 1;
                    map[acc[1] as usize][acc[0] as usize] = 1;
                }
            }
            assert!((acc[0] - acc[2]).abs() + (acc[1] - acc[3]).abs() <= 2);
            acc
        });
    let mut map2 = [[0; MAP_SIZE]; MAP_SIZE];
    map2[MAP_SIZE / 2 - 1][MAP_SIZE / 2 - 1] = 1;
    let mut part2: usize = 1;
    input_str
        .trim()
        .lines()
        .fold([MAP_SIZE as i32 / 2 - 1; 20], |mut acc, elem| {
            let parts = elem.split_once(" ").unwrap();
            let move_num = parts.1.parse::<i32>().unwrap();
            match parts.0 {
                "L" => {
                    for _ in 0..move_num {
                        acc[acc.len() - 2] -= 1;
                        do_movement_p2(&mut acc, &mut map2, &mut part2);
                    }
                }
                "R" => {
                    for _ in 0..move_num {
                        acc[acc.len() - 2] += 1;
                        do_movement_p2(&mut acc, &mut map2, &mut part2);
                    }
                }
                "U" => {
                    for _ in 0..move_num {
                        acc[acc.len() - 1] -= 1;
                        do_movement_p2(&mut acc, &mut map2, &mut part2);
                    }
                }
                "D" => {
                    for _ in 0..move_num {
                        acc[acc.len() - 1] += 1;
                        do_movement_p2(&mut acc, &mut map2, &mut part2);
                    }
                }
                _ => unreachable!(),
            }
            acc
        });
    println!("day9");
    println!("{part1}");
    println!("{part2}");
}
