use std::fs;
pub fn day3() {
    let input_str_read = fs::read_to_string("src/day3/input.txt");
    match input_str_read {
        Ok(_) => {}
        Err(_) => println!("Err"),
    }
    let input_str = input_str_read.unwrap();
    let input_str_splt = input_str.trim().split('\n').collect::<Vec<&str>>();
    let first_part = input_str_splt.iter().fold(0_usize, |acc, s| {
        let len = s.len();
        let first = &s[0..len / 2].as_bytes();
        let second = &s[len / 2..].as_bytes();
        let common = first
            .iter()
            .filter(|b| Some(b) == second.iter().find(|b2| b2 == b).as_ref())
            .collect::<Vec<&u8>>();
        if !common.is_empty() {
            if *common[0] >= 97 && *common[0] <= 122 {
                acc + (*common[0] - 96) as usize
            } else if *common[0] >= 65 && *common[0] <= 90 {
                acc + (*common[0] - 38) as usize
            } else {
                acc
            }
        } else {
            acc
        }
    });
    assert!(input_str_splt.len() == 300);
    let mut threes = Vec::new();
    let mut three = Vec::new();
    for si in 0..input_str_splt.len() {
        let s = input_str_splt[si];
        if three.len() < 3 {
            three.push(s);
        }
        if three.len() == 3 {
            threes.push(three.clone());
            three.clear();
        }
    }
    debug_assert!(threes.len() == 100, "len: {}", threes.len());
    let mut second_part: u64 = 0;
    for triple in threes {
        let t1_bytes = triple[0].as_bytes();
        let t2_bytes = triple[1].as_bytes();
        let t3_bytes = triple[2].as_bytes();
        let common = t1_bytes.iter().find(|b| {
            match t2_bytes.iter().find(|b2| {
                match t3_bytes
                    .iter()
                    .find(|b3| **b2 == **b3 && **b3 == **b && **b == **b2)
                {
                    Some(_) => true,
                    None => false,
                }
            }) {
                Some(_) => true,
                None => false,
            }
        });
        match common {
            Some(b) => {
                if *b >= 97 && *b <= 122 {
                    second_part += (b - 96) as u64;
                } else if *b >= 65 && *b <= 90 {
                    second_part += (b - 38) as u64;
                }
            }
            None => {
                println!("NO COMMON");
                println!("{}\n{}\n{}", triple[0], triple[1], triple[2]);
            }
        }
    }
    println!("day3");
    println!("{first_part}");
    println!("{second_part}");
}
