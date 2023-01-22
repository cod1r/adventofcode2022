use std::collections::HashSet;
#[derive(Clone, Debug)]
enum Rock {
    Horiz,
    Vert,
    J,
    Plus,
    Block,
}
pub fn day17() {
    let input_str = include_str!("input.txt");
    let rocks = [Rock::Horiz, Rock::Plus, Rock::J, Rock::Vert, Rock::Block];
    let mut char_iter = input_str.trim().chars().enumerate().cycle();
    let mut tops = HashSet::new();
    let mut top_most: (usize, usize) = (0, 0);
    let mut rock_iter = rocks.iter().cycle();
    let mut top = top_most.1 + 3;
    let mut coords = match rock_iter.next().unwrap() {
        Rock::Horiz => [(2, top), (3, top), (4, top), (5, top)].to_vec(),
        Rock::Plus => [(2, top + 1), (3, top), (4, top + 1), (3, top + 2)].to_vec(),
        Rock::Vert => [(2, top), (2, top + 1), (2, top + 2), (2, top + 3)].to_vec(),
        Rock::J => [(2, top), (3, top), (4, top), (4, top + 1), (4, top + 2)].to_vec(),
        Rock::Block => [(2, top), (3, top), (2, top + 1), (3, top + 1)].to_vec(),
    };
    let mut num_rocks: usize = 0;
    const SIZE: usize = 7000;
    let mut heights = vec![0; SIZE];
    let mut index_of_instruction = vec![(1, '<'); SIZE];
    let mut prev = 0;
    while let Some((i, c)) = char_iter.next() {
        if num_rocks == SIZE {
            break;
        }
        match c {
            '<' => {
                if coords
                    .iter()
                    .find(|c| {
                        c.0 == 0 || tops.contains(&((c.0 - 1).to_string() + "|" + &c.1.to_string()))
                    })
                    .is_none()
                {
                    coords.iter_mut().for_each(|c| {
                        c.0 -= 1;
                    });
                }
            }
            '>' => {
                if coords
                    .iter()
                    .find(|c| {
                        c.0 == 6 || tops.contains(&((c.0 + 1).to_string() + "|" + &c.1.to_string()))
                    })
                    .is_none()
                {
                    coords.iter_mut().for_each(|c| {
                        c.0 += 1;
                    });
                }
            }
            _ => unreachable!(),
        }
        if coords
            .iter()
            .find(|c| c.1 == 0 || tops.contains(&(c.0.to_string() + "|" + &(c.1 - 1).to_string())))
            .is_none()
        {
            coords.iter_mut().for_each(|c| {
                c.1 -= 1;
            });
        } else {
            coords.iter().for_each(|c| {
                if c.1 > top_most.1 {
                    top_most = *c;
                }
                let insert = String::from(c.0.to_string() + "|" + &c.1.to_string());
                tops.insert(insert);
            });
            top = top_most.1 + 4;
            let next_rock = rock_iter.next().unwrap();
            coords = match next_rock {
                Rock::Horiz => [(2, top), (3, top), (4, top), (5, top)].to_vec(),
                Rock::Plus => [(2, top + 1), (3, top), (4, top + 1), (3, top + 2)].to_vec(),
                Rock::Vert => [(2, top), (2, top + 1), (2, top + 2), (2, top + 3)].to_vec(),
                Rock::J => [(2, top), (3, top), (4, top), (4, top + 1), (4, top + 2)].to_vec(),
                Rock::Block => [(2, top), (3, top), (2, top + 1), (3, top + 1)].to_vec(),
            };
            num_rocks += 1;
            if num_rocks == 2022 {
                println!("part1: {}", top_most.1 + 1);
            }
            //println!(
            //    "{num_rocks} {} {prev} {} {i} {:?}",
            //    top_most.1 + 1,
            //    top_most.1 + 1 - prev,
            //    next_rock,
            //);
            prev = top_most.1 + 1;
            index_of_instruction[num_rocks - 1] = (i, c);
            heights[num_rocks - 1] = (top_most.1 + 1) as i32;
        }
    }
    // we need to look for same index of instruction and same block
    //let mut equal_diffs = Vec::new();
    for start in 0..SIZE {
        for next in start + 1..SIZE {
            let mut start_c = start;
            let mut next_c = next;
            while next_c < SIZE
                && index_of_instruction[next_c].0 == index_of_instruction[start_c].0
                && index_of_instruction[next_c].1 == index_of_instruction[start_c].1
            {
                //println!(
                //    "{:?} {next_c} {:?} {start_c}",
                //    index_of_instruction[next_c], index_of_instruction[start_c]
                //);
                start_c += 1;
                next_c += 1;
            }
            if next_c == SIZE {
                let diff = next - start;
                let repeat = heights[next] - heights[start];
                let already_height = heights[start];
                let sub = (1_000_000_000_000 - (start + 1)) % diff;
                let extra = heights[next_c - 1] - heights[next_c - 1 - sub];
                //println!(
                //    "{repeat} {extra} {sub} {diff} {start} {next} {already_height} {}",
                //    heights[49] - heights[14]
                //);
                println!(
                    "part2: {}",
                    ((1_000_000_000_000 - (start + 1) - sub) / diff) * (repeat as usize)
                        + (already_height as usize) + extra as usize
                );
                return;
            }
        }
    }
}
// 1561176468626
// 1561176468626
// 1561176468626
