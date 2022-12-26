use std::collections::HashSet;

fn conv_str(x: usize, y: usize, z: usize) -> String {
    String::from(x.to_string() + "," + &y.to_string() + "," + &z.to_string())
}

fn dfs(
    bounding: &((i32, i32), (i32, i32), (i32, i32)),
    hs: &HashSet<String>,
    current: (i32, i32, i32),
    visited: &mut HashSet<String>,
) -> bool {
    if current.0 < bounding.0 .0
        || current.0 > bounding.0 .1
        || current.1 < bounding.1 .0
        || current.1 > bounding.1 .1
        || current.2 < bounding.2 .0
        || current.2 > bounding.2 .1
    {
        return false;
    }
    if hs.contains(&conv_str(
        current.0 as usize,
        current.1 as usize,
        current.2 as usize,
    )) || visited.contains(&conv_str(
        current.0 as usize,
        current.1 as usize,
        current.2 as usize,
    )) {
        return true;
    }
    visited.insert(conv_str(
        current.0 as usize,
        current.1 as usize,
        current.2 as usize,
    ));
    return dfs(bounding, hs, (current.0 - 1, current.1, current.2), visited)
        && dfs(bounding, hs, (current.0 + 1, current.1, current.2), visited)
        && dfs(bounding, hs, (current.0, current.1 - 1, current.2), visited)
        && dfs(bounding, hs, (current.0, current.1 + 1, current.2), visited)
        && dfs(bounding, hs, (current.0, current.1, current.2 - 1), visited)
        && dfs(bounding, hs, (current.0, current.1, current.2 + 1), visited);
}

pub fn day18() {
    let input_str = include_str!("input.txt");
    let mut hs = HashSet::new();
    input_str.trim().lines().for_each(|line| {
        hs.insert(String::from(line));
    });
    let part1 = input_str.trim().lines().fold(0, |mut acc, line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap();
        let z = parts.next().unwrap().parse::<usize>().unwrap();
        if !hs.contains(&conv_str(x - 1, y, z)) {
            acc += 1;
        }
        if !hs.contains(&conv_str(x + 1, y, z)) {
            acc += 1;
        }
        if !hs.contains(&conv_str(x, y - 1, z)) {
            acc += 1;
        }
        if !hs.contains(&conv_str(x, y + 1, z)) {
            acc += 1;
        }
        if !hs.contains(&conv_str(x, y, z - 1)) {
            acc += 1;
        }
        if !hs.contains(&conv_str(x, y, z + 1)) {
            acc += 1;
        }
        acc
    });
    let bounding = input_str.trim().lines().fold(
        (
            (i32::MAX, i32::MIN),
            (i32::MAX, i32::MIN),
            (i32::MAX, i32::MIN),
        ),
        |mut acc, line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            let z = parts.next().unwrap().parse::<i32>().unwrap();
            acc.0 .0 = acc.0 .0.min(x);
            acc.0 .1 = acc.0 .1.max(x);

            acc.1 .0 = acc.1 .0.min(y);
            acc.1 .1 = acc.1 .1.max(y);

            acc.2 .0 = acc.2 .0.min(z);
            acc.2 .1 = acc.2 .1.max(z);
            acc
        },
    );
    let air_pocket_faces = input_str.trim().lines().fold(0, |mut acc, line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap();
        let z = parts.next().unwrap().parse::<usize>().unwrap();
        if !hs.contains(&conv_str(x - 1, y, z)) {
            let mut visited = HashSet::new();
            if dfs(
                &bounding,
                &hs,
                (x as i32 - 1, y as i32, z as i32),
                &mut visited,
            ) {
                acc += 1;
            }
        }
        if !hs.contains(&conv_str(x + 1, y, z)) {
            let mut visited = HashSet::new();
            if dfs(
                &bounding,
                &hs,
                (x as i32 + 1, y as i32, z as i32),
                &mut visited,
            ) {
                acc += 1;
            }
        }
        if !hs.contains(&conv_str(x, y - 1, z)) {
            let mut visited = HashSet::new();
            if dfs(
                &bounding,
                &hs,
                (x as i32, y as i32 - 1, z as i32),
                &mut visited,
            ) {
                acc += 1;
            }
        }
        if !hs.contains(&conv_str(x, y + 1, z)) {
            let mut visited = HashSet::new();
            if dfs(
                &bounding,
                &hs,
                (x as i32, y as i32 + 1, z as i32),
                &mut visited,
            ) {
                acc += 1;
            }
        }
        if !hs.contains(&conv_str(x, y, z - 1)) {
            let mut visited = HashSet::new();
            if dfs(
                &bounding,
                &hs,
                (x as i32, y as i32, z as i32 - 1),
                &mut visited,
            ) {
                acc += 1;
            }
        }
        if !hs.contains(&conv_str(x, y, z + 1)) {
            let mut visited = HashSet::new();
            if dfs(
                &bounding,
                &hs,
                (x as i32, y as i32, z as i32 + 1),
                &mut visited,
            ) {
                acc += 1;
            }
        }
        acc
    });
    println!("part1: {part1}");
    println!("part2: {}", part1 - air_pocket_faces);
}
