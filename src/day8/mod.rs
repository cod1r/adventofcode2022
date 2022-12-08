pub fn day8() {
    let bytes = include_bytes!("input.txt");
    let vec = bytes[0..bytes.len() - 1]
        .split(|b| *b == b'\n')
        .map(|ba| Vec::from(ba))
        .collect::<Vec<Vec<u8>>>();
    let mut part1: usize = 0;
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            let mut add: usize = 0;
            for k in j + 1..vec[i].len() {
                if vec[i][k] >= vec[i][j] {
                    add += 1;
                    break;
                }
            }
            for k in 0..j {
                if vec[i][k] >= vec[i][j] {
                    add += 1;
                    break;
                }
            }
            for k in i + 1..vec.len() {
                if vec[k][j] >= vec[i][j] {
                    add += 1;
                    break;
                }
            }
            for k in 0..i {
                if vec[k][j] >= vec[i][j] {
                    add += 1;
                    break;
                }
            }
            if add < 4 {
                part1 += 1;
            }
        }
    }
    let mut part2: usize = 0;
    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            let mut right: usize = 0;
            for k in j + 1..vec[i].len() {
                right += 1;
                if vec[i][k] >= vec[i][j] {
                    break;
                }
            }
            let mut left: usize = 0;
            for k in (0..j).rev() {
                left += 1;
                if vec[i][k] >= vec[i][j] {
                    break;
                }
            }
            let mut bottom: usize = 0;
            for k in i + 1..vec.len() {
                bottom += 1;
                if vec[k][j] >= vec[i][j] {
                    break;
                }
            }
            let mut top: usize = 0;
            for k in (0..i).rev() {
                top += 1;
                if vec[k][j] >= vec[i][j] {
                    break;
                }
            }
            part2 = part2.max(top * bottom * left * right);
        }
    }
    println!("day8");
    println!("{}", part1);
    println!("{}", part2);
}
