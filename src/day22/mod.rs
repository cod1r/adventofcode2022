enum Instruction {
    Direction(String),
    Forward(usize),
}
pub fn day22() {
    let input_str = include_str!("input.txt");
    let mut parts = input_str[..input_str.len() - 1].split("\n\n");
    assert!(input_str.chars().last().unwrap() == '\n');
    let board = parts.next().unwrap().lines();
    let path = parts.next().unwrap();
    let mut instructions = Vec::new();
    let mut str_buffer = String::new();
    for (i, c) in path.as_bytes().iter().enumerate() {
        if c.is_ascii_digit() {
            str_buffer.push(*c as char);
        } else {
            if str_buffer.len() > 0 {
                instructions.push(Instruction::Forward(str_buffer.parse::<usize>().unwrap()));
            }
            instructions.push(Instruction::Direction((*c as char).to_string()));
            str_buffer.clear();
        }
        if i == path.as_bytes().len() - 1 && str_buffer.len() > 0 {
            instructions.push(Instruction::Forward(str_buffer.parse::<usize>().unwrap()));
        }
    }
    let mut lines = Vec::new();
    for line in board {
        let bytes = line.as_bytes();
        lines.push(bytes);
    }
    let mut location = (0, 0);
    for b in 0..lines[0].len() {
        if lines[0][b] == b'.' {
            location.1 = b;
            break;
        }
    }
    let mut rows = Vec::new();
    let mut cols = Vec::new();
    let mut max_col = 0;
    for (_, line) in lines.iter().enumerate() {
        let mut rng = (0, 0);
        max_col = max_col.max(line.len());
        for (j, b) in line.iter().enumerate() {
            if *b == b'.' || *b == b'#' {
                rng.0 = j;
                break;
            }
        }
        rng.1 = line.len() - 1;
        rows.push(rng);
    }
    assert!(rows.len() == lines.len());
    for col_idx in 0..max_col {
        let mut rng = (0, 0);
        for row_idx in 0..lines.len() {
            if col_idx < lines[row_idx].len()
                && (lines[row_idx][col_idx] == b'.' || lines[row_idx][col_idx] == b'#')
            {
                rng.0 = row_idx;
                break;
            }
        }
        for row_idx in rng.0..lines.len() {
            if (row_idx + 1 < lines.len()
                && col_idx < lines[row_idx + 1].len()
                && lines[row_idx + 1][col_idx] == b' ')
                || row_idx + 1 == lines.len()
                || col_idx >= lines[row_idx + 1].len()
            {
                rng.1 = row_idx;
                break;
            }
        }
        cols.push(rng);
    }
    assert!(cols.len() == max_col);
    let mut turn: &'static str = "R";
    for i in instructions {
        assert!(lines[location.0][location.1] != b'#');
        assert!(lines[location.0][location.1] != b' ');
        assert!(lines[location.0][location.1] == b'.');
        match i {
            Instruction::Forward(f) => match turn {
                "R" => {
                    for _ in 0..f {
                        if location.1 + 1 <= rows[location.0].1
                            && lines[location.0][location.1 + 1] == b'#'
                        {
                            break;
                        }
                        location.1 += 1;
                        if location.1 > rows[location.0].1 {
                            if lines[location.0][rows[location.0].0] == b'#' {
                                location.1 -= 1;
                                break;
                            } else {
                                location.1 = rows[location.0].0;
                            }
                        }
                    }
                }
                "L" => {
                    for _ in 0..f {
                        if location.1 > 0
                            && location.1 - 1 >= rows[location.0].0
                            && lines[location.0][location.1 - 1] == b'#'
                        {
                            break;
                        }
                        if location.1 > rows[location.0].0 {
                            location.1 -= 1;
                        } else {
                            if lines[location.0][rows[location.0].1] == b'#' {
                                break;
                            } else {
                                location.1 = rows[location.0].1;
                            }
                        }
                    }
                }
                "U" => {
                    for _ in 0..f {
                        if location.0 > 0
                            && location.0 - 1 >= cols[location.1].0
                            && lines[location.0 - 1][location.1] == b'#'
                        {
                            break;
                        }
                        if location.0 > cols[location.1].0 {
                            location.0 -= 1;
                        } else {
                            if lines[cols[location.1].1][location.1] == b'#' {
                                break;
                            } else {
                                location.0 = cols[location.1].1;
                            }
                        }
                    }
                }
                "D" => {
                    for _ in 0..f {
                        if location.0 + 1 <= cols[location.1].1
                            && lines[location.0 + 1][location.1] == b'#'
                        {
                            break;
                        }
                        location.0 += 1;
                        if location.0 > cols[location.1].1 {
                            if lines[cols[location.1].0][location.1] == b'#' {
                                location.0 -= 1;
                                break;
                            } else {
                                location.0 = cols[location.1].0;
                            }
                        }
                    }
                }
                _ => unreachable!(),
            },
            Instruction::Direction(d) => {
                turn = match turn {
                    "R" => {
                        if d == "R" {
                            "D"
                        } else if d == "L" {
                            "U"
                        } else {
                            unreachable!()
                        }
                    }
                    "L" => {
                        if d == "R" {
                            "U"
                        } else if d == "L" {
                            "D"
                        } else {
                            unreachable!()
                        }
                    }
                    "U" => {
                        if d == "R" {
                            "R"
                        } else if d == "L" {
                            "L"
                        } else {
                            unreachable!()
                        }
                    }
                    "D" => {
                        if d == "R" {
                            "L"
                        } else if d == "L" {
                            "R"
                        } else {
                            unreachable!()
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        assert!(lines[location.0][location.1] != b'#');
        assert!(lines[location.0][location.1] != b' ');
        assert!(lines[location.0][location.1] == b'.');
    }
    println!("{:?}", location);
    println!(
        "part1: {}",
        1000 * (location.0 + 1)
            + 4 * (location.1 + 1)
            + match turn {
                "R" => 0,
                "D" => 1,
                "L" => 2,
                "U" => 3,
                _ => unreachable!(),
            }
    );
}
