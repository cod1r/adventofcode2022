pub fn day20() {
    let input_str = include_str!("input.txt");
    let numbers = input_str
        .trim()
        .lines()
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .enumerate()
        .fold(Vec::new(), |mut acc, e| {
            acc.push(e);
            acc
        });
    let mut numbers2 = numbers.clone();
    let mut moved = numbers.clone();
    let mut moved2 = moved.clone();
    for i in 0..numbers.len() {
        let mut actual_ind = 0;
        for j in 0..numbers.len() {
            if moved[j] == numbers[i] {
                actual_ind = j;
                moved.remove(j);
                break;
            }
        }
        let location = if numbers[i].1 < 0 {
            let offset = (numbers[i].1 as i64).rem_euclid(moved.len() as i64);
            (actual_ind + offset as usize) % (moved.len())
        } else {
            (actual_ind + numbers[i].1 as usize) % (moved.len())
        };
        moved.insert(location, numbers[i]);
    }
    let mut zero_ind = 0;
    for (i, num) in moved.iter().enumerate() {
        if num.1 == 0 {
            zero_ind = i;
            break;
        }
    }
    println!(
        "part1: {}",
        moved[(1000 + zero_ind) % moved.len()].1
            + moved[(2000 + zero_ind) % moved.len()].1
            + moved[(3000 + zero_ind) % moved.len()].1
    );
    numbers2.iter_mut().for_each(|p| p.1 *= 811589153);
    moved2.iter_mut().for_each(|p| p.1 *= 811589153);
    for _ in 0..10 {
        for i in 0..numbers2.len() {
            let mut actual_ind = 0;
            for j in 0..numbers2.len() {
                if moved2[j] == numbers2[i] {
                    actual_ind = j;
                    moved2.remove(j);
                    break;
                }
            }
            let location = if numbers2[i].1 < 0 {
                let offset = (numbers2[i].1 as i64).rem_euclid(moved2.len() as i64);
                (actual_ind + offset as usize) % (moved2.len())
            } else {
                (actual_ind + numbers2[i].1 as usize) % (moved2.len())
            };
            moved2.insert(location, numbers2[i]);
            assert!(moved2.len() == numbers2.len());
        }
    }
    let mut zero_ind = 0;
    for (i, num) in moved2.iter().enumerate() {
        if num.1 == 0 {
            zero_ind = i;
            break;
        }
    }
    println!(
        "part2: {}",
        moved2[(1000 + zero_ind) % moved2.len()].1
            + moved2[(2000 + zero_ind) % moved2.len()].1
            + moved2[(3000 + zero_ind) % moved2.len()].1
    );
}
