pub fn day14() {
    let input_str = include_str!("input.txt");
    let mut cave = [['.'; 1000]; 1000];
    let mut max_row = 0;
    input_str.trim().lines().for_each(|line| {
        let mut coords = line.split(" -> ").map(|endpoint| {
            let mut endp_splt = endpoint.split(',');
            (
                endp_splt.next().unwrap().parse::<usize>().unwrap(),
                endp_splt.next().unwrap().parse::<usize>().unwrap(),
            )
        });
        let mut curr = coords.next().unwrap();
        for next in coords {
            max_row = max_row.max(curr.1);
            while curr.0 < next.0 {
                cave[curr.1][curr.0] = '#';
                curr.0 += 1;
            }
            while curr.0 > next.0 {
                cave[curr.1][curr.0] = '#';
                curr.0 -= 1;
            }
            while curr.1 < next.1 {
                cave[curr.1][curr.0] = '#';
                curr.1 += 1;
            }
            while curr.1 > next.1 {
                cave[curr.1][curr.0] = '#';
                curr.1 -= 1;
            }
            cave[curr.1][curr.0] = '#';
            curr = next;
        }
    });
    let mut cave2 = cave;
    let mut part1: usize = 0;
    loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 + 1 < cave.len() && cave[sand.1 + 1][sand.0] != '.' {
                if sand.0 > 0 && cave[sand.1 + 1][sand.0 - 1] == '.' {
                    sand.0 -= 1;
                } else if sand.0 + 1 < cave[0].len() && cave[sand.1 + 1][sand.0 + 1] == '.' {
                    sand.0 += 1;
                } else {
                    cave[sand.1][sand.0] = 'o';
                    part1 += 1;
                }
            }
            if sand.1 >= cave.len() {
                break;
            }
            if cave[sand.1][sand.0] != 'o' {
                sand.1 += 1;
            } else {
                break;
            }
        }
        if sand.1 >= cave.len() {
            break;
        }
    }
    println!("part1: {part1}");
    for col in 0..cave2[max_row + 2].len() {
        cave2[max_row + 2][col] = '#';
    }
    let mut part2: usize = 0;
    loop {
        let mut sand = (500, 0);
        if cave2[sand.1][sand.0] == 'o' {
            break;
        }
        loop {
            if sand.1 + 1 < cave2.len() && cave2[sand.1 + 1][sand.0] != '.' {
                if sand.0 > 0 && cave2[sand.1 + 1][sand.0 - 1] == '.' {
                    sand.0 -= 1;
                } else if sand.0 + 1 < cave2[0].len() && cave2[sand.1 + 1][sand.0 + 1] == '.' {
                    sand.0 += 1;
                } else {
                    cave2[sand.1][sand.0] = 'o';
                    part2 += 1;
                }
            }
            if sand.1 >= cave2.len() {
                break;
            }
            if cave2[sand.1][sand.0] != 'o' {
                sand.1 += 1;
            } else {
                break;
            }
        }
        if sand.1 >= cave2.len() {
            break;
        }
    }
    println!("part2: {part2}");
}
