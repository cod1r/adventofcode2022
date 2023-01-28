use std::collections::HashSet;
#[derive(Clone)]
struct Blizzard(i32, i32, u8);
#[derive(Clone)]
struct Spot(i32, i32, i32);
impl Blizzard {
    fn new(row: i32, col: i32, dir: u8) -> Blizzard {
        Blizzard(row, col, dir)
    }
    fn update(&mut self, wide: i32, long: i32) {
        match self.2 {
            b'<' => self.1 -= 1,
            b'>' => self.1 += 1,
            b'^' => self.0 -= 1,
            b'v' => self.0 += 1,
            _ => unreachable!(),
        }
        if self.0 == 0 {
            self.0 = long - 2;
        } else if self.0 == long - 1 {
            self.0 = 1;
        } else if self.1 == 0 {
            self.1 = wide - 2;
        } else if self.1 == wide - 1 {
            self.1 = 1;
        }
    }
    fn to_string(&self) -> String {
        self.0.to_string() + "," + &self.1.to_string() + "," + &self.2.to_string()
    }
}
impl Spot {
    fn new(row: i32, col: i32, steps: i32) -> Spot {
        Spot(row, col, steps)
    }
    fn to_string(&self) -> String {
        self.0.to_string() + "," + &self.1.to_string()
    }
    fn to_string_with_steps(&self) -> String {
        self.0.to_string() + "," + &self.1.to_string() + "," + &self.2.to_string()
    }
}
fn check_and_update(
    blizzes: &HashSet<String>,
    q: &mut Vec<Spot>,
    wide: i32,
    long: i32,
    done: &mut i32,
    target: (i32, i32),
) {
    let mut old_q = q.clone();
    q.clear();
    for loc in &mut old_q {
        if loc.2 + 1 > *done {
            continue;
        }
        let first = (loc.0 + 1).to_string() + "," + &loc.1.to_string();
        let second = (loc.0 - 1).to_string() + "," + &loc.1.to_string();
        let third = loc.0.to_string() + "," + &(loc.1 - 1).to_string();
        let four = loc.0.to_string() + "," + &(loc.1 + 1).to_string();
        let mut add_first = loc.0 + 1 < long - 1 && loc.1 > 0 && loc.1 < wide - 1;
        let mut add_second = loc.0 - 1 > 0 && loc.1 > 0 && loc.1 < wide - 1;
        let mut add_third = loc.0 > 0 && loc.0 < long - 1 && loc.1 - 1 > 0;
        let mut add_four = loc.0 > 0 && loc.0 < long - 1 && loc.1 + 1 < wide - 1;
        let mut add_fifth = true;
        for d in [b'<', b'>', b'^', b'v'] {
            if blizzes.contains(&(first.clone() + "," + &d.to_string())) {
                add_first = false;
            }
            if blizzes.contains(&(second.clone() + "," + &d.to_string())) {
                add_second = false;
            }
            if blizzes.contains(&(third.clone() + "," + &d.to_string())) {
                add_third = false;
            }
            if blizzes.contains(&(four.clone() + "," + &d.to_string())) {
                add_four = false;
            }
            if blizzes.contains(&(loc.to_string() + "," + &d.to_string())) {
                add_fifth = false;
            }
        }
        if (loc.0 + 1 == target.0 && loc.1 == target.1)
            || (loc.0 - 1 == target.0 && loc.1 == target.1)
        {
            *done = *done.min(&mut (loc.2 + 1));
        }
        if add_first {
            q.push(Spot::new(loc.0 + 1, loc.1, loc.2 + 1));
        }
        if add_second {
            q.push(Spot::new(loc.0 - 1, loc.1, loc.2 + 1));
        }
        if add_third {
            q.push(Spot::new(loc.0, loc.1 - 1, loc.2 + 1));
        }
        if add_four {
            q.push(Spot::new(loc.0, loc.1 + 1, loc.2 + 1));
        }
        if add_fifth {
            q.push(Spot::new(loc.0, loc.1, loc.2 + 1));
        }
    }
    for i in 0..q.len() {
        let mut j = i + 1;
        while j < q.len() {
            if i != j && q[i].0 == q[j].0 && q[i].1 == q[j].1 {
                q.remove(j);
                continue;
            }
            j += 1;
        }
    }
}
fn calc(valley: &Vec<&[u8]>, blizz: &mut Vec<Blizzard>, you: Spot, target: (i32, i32)) -> i32 {
    let mut q = Vec::new();
    q.push(you);
    let mut done = i32::MAX;
    let mut part2 = 0;
    let mut blizz_hashes = HashSet::new();
    while !q.is_empty() {
        for i in 0..blizz.len() {
            let b = &mut blizz[i];
            b.update(valley[0].len() as i32, valley.len() as i32);
            blizz_hashes.insert(b.to_string());
        }
        check_and_update(
            &blizz_hashes,
            &mut q,
            valley[0].len() as i32,
            valley.len() as i32,
            &mut done,
            target,
        );
        blizz_hashes.clear();
    }
    part2 += done;
    part2
}
pub fn day24() {
    let input_str = include_str!("input.txt");
    let valley = input_str
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    let mut blizz = Vec::new();
    for row in 0..valley.len() {
        for col in 0..valley[row].len() {
            if valley[row][col] != b'.' && valley[row][col] != b'#' {
                blizz.push(Blizzard::new(row as i32, col as i32, valley[row][col]));
            }
        }
    }
    let you = Spot::new(0, 1, 0);
    let part1 = calc(
        &valley,
        &mut blizz,
        you,
        (valley.len() as i32 - 1, valley[0].len() as i32 - 2),
    );
    let you = Spot::new(valley.len() as i32 - 1, valley[0].len() as i32 - 2, 1);
    let back = calc(&valley, &mut blizz, you, (0, 1));
    let you = Spot::new(0, 1, part1 + back + 1);
    let part2 = calc(
        &valley,
        &mut blizz,
        you,
        (valley.len() as i32 - 1, valley[0].len() as i32 - 2),
    );
    println!("part1: {part1}");
    println!("part2: {part2}");
}
