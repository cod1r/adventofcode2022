use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Instruction {
    Direction(String),
    Forward(usize),
}
fn wrap_cube_logic(
    mut first_facing: &'static str,
    mut second_facing: &'static str,
    mut first: (usize, usize),
    mut second: (usize, usize),
    max_col: usize,
    lines: &Vec<Vec<u8>>,
    cube_mappings: &mut HashMap<String, (usize, usize)>,
    cube_facings: &mut HashMap<String, &'static str>,
) {
    let mut been_to: HashSet<String> = HashSet::new();
    loop {
        let mut first_out = false;
        let mut second_out = false;
        if been_to
            .contains(&(first.0.to_string() + "," + &first.1.to_string() + "," + first_facing))
            || been_to.contains(
                &(second.0.to_string() + "," + &second.1.to_string() + "," + second_facing),
            )
        {
            break;
        }
        been_to.insert(first.0.to_string() + "," + &first.1.to_string() + "," + first_facing);
        been_to.insert(second.0.to_string() + "," + &second.1.to_string() + "," + second_facing);
        match first_facing {
            "R" => {
                first.1 += 1;
                if first.1 >= max_col || lines[first.0][first.1] == b' ' {
                    first_out = true;
                }
            }
            "L" => {
                if first.1 == 0 || lines[first.0][first.1 - 1] == b' ' {
                    first_out = true;
                } else if first.1 > 0 {
                    first.1 -= 1;
                }
            }
            "U" => {
                if first.0 == 0 || lines[first.0 - 1][first.1] == b' ' {
                    first_out = true;
                } else if first.0 > 0 {
                    first.0 -= 1;
                }
            }
            "D" => {
                first.0 += 1;
                if first.0 >= lines.len() || lines[first.0][first.1] == b' ' {
                    first_out = true;
                }
            }
            _ => unreachable!(),
        }
        match second_facing {
            "R" => {
                second.1 += 1;
                if second.1 >= max_col || lines[second.0][second.1] == b' ' {
                    second_out = true;
                }
            }
            "L" => {
                if second.1 == 0 || lines[second.0][second.1 - 1] == b' ' {
                    second_out = true;
                } else if second.1 > 0 {
                    second.1 -= 1;
                }
            }
            "U" => {
                if second.0 == 0 || lines[second.0 - 1][second.1] == b' ' {
                    second_out = true;
                } else if second.0 > 0 {
                    second.0 -= 1;
                }
            }
            "D" => {
                second.0 += 1;
                if second.0 >= lines.len() || lines[second.0][second.1] == b' ' {
                    second_out = true;
                }
            }
            _ => unreachable!(),
        }
        if first_out && second_out {
            break;
        }
        if first_out {
            match first_facing {
                "R" | "L" => {
                    if first_facing == "R" {
                        assert!(first.1 > 0);
                        first.1 -= 1;
                    }
                    if first.0 > 0
                        && (lines[first.0 - 1][first.1] == b'#'
                            || lines[first.0 - 1][first.1] == b'.')
                    {
                        first_facing = "U";
                    } else if first.0 + 1 < lines.len()
                        && (lines[first.0 + 1][first.1] == b'#'
                            || lines[first.0 + 1][first.1] == b'.')
                    {
                        first_facing = "D";
                    } else {
                        unreachable!()
                    }
                }
                "U" | "D" => {
                    if first_facing == "D" {
                        assert!(first.0 > 0);
                        first.0 -= 1;
                    }
                    if first.1 > 0
                        && (lines[first.0][first.1 - 1] == b'#'
                            || lines[first.0][first.1 - 1] == b'.')
                    {
                        first_facing = "L";
                    } else if first.1 + 1 < lines.len()
                        && (lines[first.0][first.1 + 1] == b'#'
                            || lines[first.0][first.1 + 1] == b'.')
                    {
                        first_facing = "R";
                    } else {
                        unreachable!()
                    }
                }
                _ => unreachable!(),
            }
        }
        if second_out {
            match second_facing {
                "R" | "L" => {
                    if second_facing == "R" {
                        assert!(second.1 > 0);
                        second.1 -= 1;
                    }
                    if second.0 > 0
                        && (lines[second.0 - 1][second.1] == b'#'
                            || lines[second.0 - 1][second.1] == b'.')
                    {
                        second_facing = "U";
                    } else if second.0 + 1 < lines.len()
                        && (lines[second.0 + 1][second.1] == b'#'
                            || lines[second.0 + 1][second.1] == b'.')
                    {
                        second_facing = "D";
                    } else {
                        unreachable!()
                    }
                }
                "U" | "D" => {
                    if second_facing == "D" {
                        assert!(second.0 > 0);
                        second.0 -= 1;
                    }
                    if second.1 > 0
                        && (lines[second.0][second.1 - 1] == b'#'
                            || lines[second.0][second.1 - 1] == b'.')
                    {
                        second_facing = "L";
                    } else if second.1 + 1 < lines.len()
                        && (lines[second.0][second.1 + 1] == b'#'
                            || lines[second.0][second.1 + 1] == b'.')
                    {
                        second_facing = "R";
                    } else {
                        unreachable!()
                    }
                }
                _ => unreachable!(),
            }
        }
        let curr_second = second.0.to_string() + "," + &second.1.to_string();
        let curr_first = first.0.to_string() + "," + &first.1.to_string();
        match second_facing {
            "R" | "L" => {
                if second.0 + 1 == lines.len() || lines[second.0 + 1][second.1] == b' ' {
                    let key = (second.0 + 1).to_string() + "," + &second.1.to_string();
                    cube_mappings.insert(key.clone(), first);
                    cube_facings.insert(curr_first.clone() + "|" + &curr_second.clone(), "U");
                }
                if second.0 == 0 || lines[second.0 - 1][second.1] == b' ' {
                    let key = (second.0 as i32 - 1).to_string() + "," + &second.1.to_string();
                    cube_mappings.insert(key.clone(), first);
                    cube_facings.insert(curr_first.clone() + "|" + &curr_second, "D");
                }
            }
            "U" | "D" => {
                if second.1 + 1 == max_col || lines[second.0][second.1 + 1] == b' ' {
                    let key = second.0.to_string() + "," + &(second.1 + 1).to_string();
                    cube_mappings.insert(key.clone(), first);
                    cube_facings.insert(curr_first.clone() + "|" + &curr_second.clone(), "L");
                }
                if second.1 == 0 || lines[second.0][second.1 - 1] == b' ' {
                    let key = second.0.to_string() + "," + &(second.1 as i32 - 1).to_string();
                    cube_mappings.insert(key.clone(), first);
                    cube_facings.insert(curr_first.clone() + "|" + &curr_second, "R");
                }
            }
            _ => unreachable!(),
        }
        match first_facing {
            "R" | "L" => {
                if first.0 + 1 == lines.len() || lines[first.0 + 1][first.1] == b' ' {
                    let key = (first.0 + 1).to_string() + "," + &first.1.to_string();
                    cube_mappings.insert(key.clone(), second);
                    cube_facings.insert(curr_second.clone() + "|" + &curr_first.clone(), "U");
                }
                if first.0 == 0 || lines[first.0 - 1][first.1] == b' ' {
                    let key = (first.0 as i32 - 1).to_string() + "," + &first.1.to_string();
                    cube_mappings.insert(key.clone(), second);
                    cube_facings.insert(curr_second.clone() + "|" + &curr_first.clone(), "D");
                }
            }
            "U" | "D" => {
                if first.1 + 1 == max_col || lines[first.0][first.1 + 1] == b' ' {
                    let key = first.0.to_string() + "," + &(first.1 + 1).to_string();
                    cube_mappings.insert(key.clone(), second);
                    cube_facings.insert(curr_second.clone() + "|" + &curr_first.clone(), "L");
                }
                if first.1 == 0 || lines[first.0][first.1 - 1] == b' ' {
                    let key = first.0.to_string() + "," + &(first.1 as i32 - 1).to_string();
                    cube_mappings.insert(key.clone(), second);
                    cube_facings.insert(curr_second.clone() + "|" + &curr_first.clone(), "R");
                }
            }
            _ => unreachable!(),
        }
    }
}
fn rotate<'a>(turn: &'static str, d: &'a str) -> &'static str {
    match turn {
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
pub fn day22() {
    let input_str = include_str!("test_wrap.txt");
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
    for i in instructions.clone() {
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
                turn = rotate(turn, &d);
            }
        }
        assert!(lines[location.0][location.1] != b'#');
        assert!(lines[location.0][location.1] != b' ');
        assert!(lines[location.0][location.1] == b'.');
    }
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
    let mut lines2 = Vec::new();
    for line in &lines {
        let mut full_line = vec![b' '; max_col];
        for (b_index, byte) in line.iter().enumerate() {
            full_line[b_index] = *byte;
        }
        lines2.push(full_line);
    }
    turn = "R";
    let mut cube_mappings: HashMap<String, (usize, usize)> = HashMap::new();
    let mut cube_facings: HashMap<String, &'static str> = HashMap::new();
    const FACE_SIZE: usize = 4;
    for corner_row_idx in 0..lines.len() / FACE_SIZE {
        for corner_col_idx in 0..max_col / FACE_SIZE {
            let actual_row_idx = corner_row_idx * FACE_SIZE;
            let actual_col_idx = corner_col_idx * FACE_SIZE;
            if lines2[actual_row_idx][actual_col_idx] == b' ' {
                if actual_row_idx > 0 && actual_col_idx > 0 {
                    // top left corner
                    let byte_diag = lines2[actual_row_idx - 1][actual_col_idx - 1];
                    let byte_left = lines2[actual_row_idx][actual_col_idx - 1];
                    let byte_right = lines2[actual_row_idx - 1][actual_col_idx];
                    let first = (actual_row_idx - 1, actual_col_idx - 1);
                    let second = (actual_row_idx - 1, actual_col_idx - 1);
                    if (byte_diag == b'#' || byte_diag == b'.')
                        && (byte_left == b'#' || byte_left == b'.')
                        && (byte_right == b'#' || byte_right == b'.')
                    {
                        let first_facing = "R";
                        let second_facing = "D";
                        wrap_cube_logic(
                            first_facing,
                            second_facing,
                            first,
                            second,
                            max_col,
                            &lines2,
                            &mut cube_mappings,
                            &mut cube_facings,
                        );
                    }
                }
                if actual_row_idx > 0 && actual_col_idx + FACE_SIZE < max_col {
                    // top right corner
                    let byte_diag = lines2[actual_row_idx - 1][actual_col_idx + FACE_SIZE];
                    let byte_left = lines2[actual_row_idx - 1][actual_col_idx + (FACE_SIZE - 1)];
                    let byte_under = lines2[actual_row_idx][actual_col_idx + FACE_SIZE];
                    let byte_under_left = lines2[actual_row_idx][actual_col_idx + (FACE_SIZE - 1)];
                    if (byte_diag == b'#' || byte_diag == b'.')
                        && (byte_left == b'#' || byte_left == b'.')
                        && (byte_under == b'#' || byte_under == b'.')
                        && (byte_under_left == b' ')
                    {
                        wrap_cube_logic(
                            "L",
                            "D",
                            (actual_row_idx - 1, actual_col_idx + FACE_SIZE),
                            (actual_row_idx - 1, actual_col_idx + FACE_SIZE),
                            max_col,
                            &lines2,
                            &mut cube_mappings,
                            &mut cube_facings,
                        );
                    }
                }
                if actual_row_idx + FACE_SIZE < lines2.len() && actual_col_idx > 0 {
                    // bottom left corner
                    let byte_diag = lines2[actual_row_idx + FACE_SIZE][actual_col_idx - 1];
                    let byte_right = lines2[actual_row_idx + FACE_SIZE][actual_col_idx];
                    let byte_above_diag =
                        lines2[actual_row_idx + (FACE_SIZE - 1)][actual_col_idx - 1];
                    let byte_above_right = lines2[actual_row_idx + (FACE_SIZE - 1)][actual_col_idx];
                    if (byte_diag == b'#' || byte_diag == b'.')
                        && (byte_right == b'#' || byte_right == b'.')
                        && (byte_above_diag == b'#' || byte_above_diag == b'.')
                        && byte_above_right == b' '
                    {
                        wrap_cube_logic(
                            "U",
                            "R",
                            (actual_row_idx + FACE_SIZE, actual_col_idx - 1),
                            (actual_row_idx + FACE_SIZE, actual_col_idx - 1),
                            max_col,
                            &lines2,
                            &mut cube_mappings,
                            &mut cube_facings,
                        )
                    }
                }
                if actual_row_idx + FACE_SIZE < lines2.len() && actual_col_idx + FACE_SIZE < max_col
                {
                    // bottom right corner
                    let byte_diag = lines2[actual_row_idx + FACE_SIZE][actual_col_idx + FACE_SIZE];
                    let byte_left =
                        lines2[actual_row_idx + FACE_SIZE][actual_col_idx + (FACE_SIZE - 1)];
                    let byte_upper_diag =
                        lines2[actual_row_idx + (FACE_SIZE - 1)][actual_col_idx + FACE_SIZE];
                    let byte_above_left =
                        lines2[actual_row_idx + (FACE_SIZE - 1)][actual_col_idx + (FACE_SIZE - 1)];
                    if (byte_diag == b'#' || byte_diag == b'.')
                        && (byte_left == b'#' || byte_left == b'.')
                        && (byte_upper_diag == b'#' || byte_upper_diag == b'.')
                        && byte_above_left == b' '
                    {
                        wrap_cube_logic(
                            "U",
                            "L",
                            (actual_row_idx + FACE_SIZE, actual_col_idx + FACE_SIZE),
                            (actual_row_idx + FACE_SIZE, actual_col_idx + FACE_SIZE),
                            max_col,
                            &lines2,
                            &mut cube_mappings,
                            &mut cube_facings,
                        )
                    }
                }
            }
        }
    }
    //for key in cube_mappings.keys() {
    //    println!("{} {:?}", key, cube_mappings.get(key).unwrap());
    //}
    //println!("-------------------------");
    //for key in cube_facings.keys() {
    //    println!("{} {}", key, cube_facings.get(key).unwrap());
    //}
    let mut location2 = (0, 0);
    for b in 0..lines2[0].len() {
        if lines2[0][b] == b'.' {
            location2.1 = b;
            break;
        }
    }
    for i in instructions.clone() {
        match i {
            Instruction::Forward(f) => {
                for _ in 0..f {
                    match turn {
                        "R" => {
                            if location2.1 + 1 <= rows[location2.0].1
                                && lines[location2.0][location2.1 + 1] == b'#'
                            {
                                break;
                            }
                            location2.1 += 1;
                            if location2.1 > rows[location2.0].1 {
                                let wrap_location = cube_mappings
                                    .get(
                                        &(location2.0.to_string() + "," + &location2.1.to_string()),
                                    )
                                    .unwrap();
                                if lines[wrap_location.0][wrap_location.1] == b'#' {
                                    location2.1 -= 1;
                                    break;
                                } else {
                                    turn = cube_facings
                                        .get(
                                            &(location2.0.to_string()
                                                + ","
                                                + &(location2.1 - 1).to_string()
                                                + "|"
                                                + &wrap_location.0.to_string()
                                                + ","
                                                + &wrap_location.1.to_string()),
                                        )
                                        .unwrap();
                                    location2 = *wrap_location;
                                }
                            }
                        }
                        "L" => {
                            if location2.1 > 0
                                && location2.1 - 1 >= rows[location2.0].0
                                && lines[location2.0][location2.1 - 1] == b'#'
                            {
                                break;
                            }
                            if location2.1 > rows[location2.0].0 {
                                location2.1 -= 1;
                            } else {
                                let wrap_location = cube_mappings
                                    .get(
                                        &(location2.0.to_string() + "," + &location2.1.to_string()),
                                    )
                                    .unwrap();
                                if lines[wrap_location.0][wrap_location.1] == b'#' {
                                    break;
                                } else {
                                    turn = cube_facings
                                        .get(
                                            &(location2.0.to_string()
                                                + ","
                                                + &location2.1.to_string()
                                                + "|"
                                                + &wrap_location.0.to_string()
                                                + ","
                                                + &wrap_location.1.to_string()),
                                        )
                                        .unwrap();
                                    location2 = *wrap_location;
                                }
                            }
                        }
                        "U" => {
                            if location2.0 > 0
                                && location2.0 - 1 >= cols[location2.1].0
                                && lines[location2.0 - 1][location2.1] == b'#'
                            {
                                break;
                            }
                            if location2.0 > cols[location2.1].0 {
                                location2.0 -= 1;
                            } else {
                                let wrap_location = cube_mappings
                                    .get(
                                        &(location2.0.to_string() + "," + &location2.1.to_string()),
                                    )
                                    .unwrap();
                                if lines[wrap_location.0][wrap_location.1] == b'#' {
                                    break;
                                } else {
                                    turn = cube_facings
                                        .get(
                                            &(location2.0.to_string()
                                                + ","
                                                + &location2.1.to_string()
                                                + "|"
                                                + &wrap_location.0.to_string()
                                                + ","
                                                + &wrap_location.1.to_string()),
                                        )
                                        .unwrap();
                                    location2 = *wrap_location;
                                }
                            }
                        }
                        "D" => {
                            if location2.0 + 1 <= cols[location2.1].1
                                && lines[location2.0 + 1][location2.1] == b'#'
                            {
                                break;
                            }
                            location2.0 += 1;
                            if location2.0 > cols[location2.1].1 {
                                let wrap_location = cube_mappings
                                    .get(
                                        &(location2.0.to_string() + "," + &location2.1.to_string()),
                                    )
                                    .unwrap();
                                if lines[wrap_location.0][wrap_location.1] == b'#' {
                                    location2.0 -= 1;
                                    break;
                                } else {
                                    turn = cube_facings
                                        .get(
                                            &((location2.0 - 1).to_string()
                                                + ","
                                                + &location2.1.to_string()
                                                + "|"
                                                + &wrap_location.0.to_string()
                                                + ","
                                                + &wrap_location.1.to_string()),
                                        )
                                        .unwrap();
                                    location2 = *wrap_location;
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Instruction::Direction(d) => {
                turn = rotate(turn, &d);
            }
        }
    }
    println!("{:?} {}", location2, turn);
    println!(
        "part2: {}",
        1000 * (location2.0 + 1)
            + 4 * (location2.1 + 1)
            + match turn {
                "R" => 0,
                "D" => 1,
                "L" => 2,
                "U" => 3,
                _ => unreachable!(),
            }
    );
}
