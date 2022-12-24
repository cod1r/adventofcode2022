use std::collections::HashSet;
enum Rock {
    Horiz,
    Vert,
    J,
    Plus,
    Block,
}
struct RockIter {
    index: usize,
    rocks: [Rock; 5],
}
impl Clone for Rock {
    fn clone(&self) -> Self {
        match self {
            Rock::Horiz => Rock::Horiz,
            Rock::Vert => Rock::Vert,
            Rock::Block => Rock::Block,
            Rock::J => Rock::J,
            Rock::Plus => Rock::Plus,
        }
    }
}
impl RockIter {
    fn new() -> RockIter {
        let rocks = [Rock::Horiz, Rock::Plus, Rock::J, Rock::Vert, Rock::Block];
        RockIter { index: 0, rocks }
    }
}
impl Iterator for RockIter {
    type Item = Rock;
    fn next(&mut self) -> Option<Self::Item> {
        self.index %= self.rocks.len();
        let rock = self.rocks.get(self.index).unwrap().clone();
        self.index += 1;
        Some(rock)
    }
}
pub fn day17() {
    let input_str = include_str!("example.txt");
    let mut char_iter = input_str.trim().chars().cycle();
    let mut tops = HashSet::new();
    let mut top_most: (usize, usize) = (0, 0);
    let mut rock_iter = RockIter::new();
    let mut top = top_most.1 + 3;
    let mut coords = match rock_iter.next().unwrap() {
        Rock::Horiz => [(2, top), (3, top), (4, top), (5, top)].to_vec(),
        Rock::Plus => [(2, top + 1), (3, top), (4, top + 1), (3, top + 2)].to_vec(),
        Rock::Vert => [(2, top), (2, top + 1), (2, top + 2), (2, top + 3)].to_vec(),
        Rock::J => [(2, top), (3, top), (4, top), (4, top + 1), (4, top + 2)].to_vec(),
        Rock::Block => [(2, top), (3, top), (2, top + 1), (3, top + 1)].to_vec(),
    };
    let mut num_rocks: usize = 0;
    while let Some(c) = char_iter.next() {
        if num_rocks == 2022 {
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
            coords = match rock_iter.next().unwrap() {
                Rock::Horiz => [(2, top), (3, top), (4, top), (5, top)].to_vec(),
                Rock::Plus => [(2, top + 1), (3, top), (4, top + 1), (3, top + 2)].to_vec(),
                Rock::Vert => [(2, top), (2, top + 1), (2, top + 2), (2, top + 3)].to_vec(),
                Rock::J => [(2, top), (3, top), (4, top), (4, top + 1), (4, top + 2)].to_vec(),
                Rock::Block => [(2, top), (3, top), (2, top + 1), (3, top + 1)].to_vec(),
            };
            num_rocks += 1;
        }
    }
    println!("{}", top_most.1 + 1);
    //let mut map = [['.'; 7]; 500];
    //tops.iter().for_each(|c| {
    //    let mut parts = c.split('|');
    //    let one = parts.next().unwrap().parse::<usize>().unwrap();
    //    let two = parts.next().unwrap().parse::<usize>().unwrap();
    //    map[30 - 1 - two][one] = '#';
    //});
    //map.iter().for_each(|r| {
    //    r.iter().for_each(|c| print!("{c}"));
    //    println!();
    //});
}
