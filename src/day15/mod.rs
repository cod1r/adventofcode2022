use std::collections::HashSet;

pub fn day15() {
    let input_str = include_str!("input.txt");
    let mut sb = input_str.trim().lines().map(|line| {
        let mut parts = line.split(' ');
        let sx_str = parts.nth(2).unwrap();
        let sx = sx_str[2..sx_str.len() - 1].parse::<i32>().unwrap();
        let sy_str = parts.nth(0).unwrap();
        let sy = sy_str[2..sy_str.len() - 1].parse::<i32>().unwrap();

        let bx_str = parts.nth(4).unwrap();
        let bx = bx_str[2..bx_str.len() - 1].parse::<i32>().unwrap();
        let by_str = parts.nth(0).unwrap();
        let by = by_str[2..].parse::<i32>().unwrap();
        (sx, sy, bx, by)
    });
    let sb2 = sb.clone().fold(Vec::new(), |mut acc, e| {
        acc.push(e);
        acc
    });
    {
        const ROW_NUM: i32 = 2_000_000;
        let mut part1 = (i32::MAX, i32::MIN);
        let mut b_on_row = HashSet::new();
        while let Some(p) = sb.next() {
            let man_dist = (p.0 - p.2).abs() + (p.1 - p.3).abs();
            if (ROW_NUM - p.1).abs() <= man_dist {
                let diff = ((ROW_NUM - p.1).abs() - man_dist).abs();
                part1.0 = part1.0.min(p.0 - diff);
                part1.1 = part1.1.max(p.0 + diff);
                if p.3 == ROW_NUM {
                    b_on_row.insert(p.2.to_string() + &p.3.to_string());
                }
            }
        }
        println!("part1: {}", part1.1 + 1 - part1.0 - b_on_row.len() as i32);
    }
    {
        const AREA: i32 = 4_000_000;
        for rn in 0..AREA + 1 {
            let mut contig = Vec::new();
            for p in &sb2 {
                let man_dist = (p.0 - p.2).abs() + (p.1 - p.3).abs();
                if (rn - p.1).abs() <= man_dist {
                    let diff = ((rn - p.1).abs() - man_dist).abs();
                    contig.push((p.0 - diff, p.0 + diff));
                }
            }
            let mut one = contig.clone();
            one.sort_by(|a, b| a.0.cmp(&b.0));

            let final_one = one.iter().fold(one.first().unwrap().clone(), |acc, e| {
                if acc.0 - 1 <= e.1 && acc.1 + 1 >= e.0 {
                    (acc.0.min(e.0), acc.1.max(e.1))
                } else {
                    acc
                }
            });
            if final_one.0.max(0) != 0 || final_one.1.min(AREA) != AREA {
                println!(
                    "{} {rn}",
                    if final_one.0.max(0) != 0 {
                        final_one.0.max(0) - 1
                    } else {
                        final_one.1.min(AREA) + 1
                    }
                );
            }
        }
    }
}
