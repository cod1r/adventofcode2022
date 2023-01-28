use std::collections::{HashMap};

// elves that want to move to the same location cannot move
// for each elf, the first direction that they consider is moved to the back of the list of
// directions
// if there are no elves in the eight positions that the current elf considers, the current elf
// does nothing
// first half of rounds is considering if there are any other elves in the 8 positions
// and then taking the first direction they considered (valid or not) and putting at the end.
// second half is where every elf simultaneously moves to their target location if and only if
// they are the only ones moving to that specific location.
#[derive(Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}
#[derive(Clone)]
struct Elf {
    location: (i32, i32),
    d: Option<Dir>,
}
impl Elf {
    fn new(loc: (i32, i32)) -> Elf {
        Elf {
            location: loc,
            d: None,
        }
    }
    fn update(&mut self, destinations: &HashMap<String, i32>) {
        match &self.d {
            Some(dir) => match dir {
                Dir::North => {
                    let dest =
                        (self.location.0 - 1).to_string() + "," + &self.location.1.to_string();
                    if *destinations.get(&dest).unwrap() == 1 {
                        self.location.0 -= 1;
                    }
                }
                Dir::South => {
                    let dest =
                        (self.location.0 + 1).to_string() + "," + &self.location.1.to_string();
                    if *destinations.get(&dest).unwrap() == 1 {
                        self.location.0 += 1;
                    }
                }
                Dir::West => {
                    let dest =
                        self.location.0.to_string() + "," + &(self.location.1 - 1).to_string();
                    if *destinations.get(&dest).unwrap() == 1 {
                        self.location.1 -= 1;
                    }
                }
                Dir::East => {
                    let dest =
                        self.location.0.to_string() + "," + &(self.location.1 + 1).to_string();
                    if *destinations.get(&dest).unwrap() == 1 {
                        self.location.1 += 1;
                    }
                }
            },
            None => {}
        }
        self.d = None;
    }
    fn check_all(&self, loc_to_elf: &HashMap<String, Elf>) -> bool {
        if loc_to_elf
            .get(&((self.location.0 - 1).to_string() + "," + &(self.location.1 - 1).to_string())).is_some()
        {
            true
        } else if loc_to_elf
            .get(&((self.location.0 - 1).to_string() + "," + &self.location.1.to_string())).is_some()
        {
            true
        } else if loc_to_elf
            .get(&((self.location.0 - 1).to_string() + "," + &(self.location.1 + 1).to_string())).is_some()
        {
            true
        } else if loc_to_elf
            .get(&(self.location.0.to_string() + "," + &(self.location.1 + 1).to_string())).is_some()
        {
            true
        } else if loc_to_elf
            .get(&((self.location.0 + 1).to_string() + "," + &(self.location.1 + 1).to_string())).is_some()
        {
            true
        } else if loc_to_elf
            .get(&((self.location.0 + 1).to_string() + "," + &self.location.1.to_string())).is_some()
        {
            true
        } else if loc_to_elf
            .get(&((self.location.0 + 1).to_string() + "," + &(self.location.1 - 1).to_string())).is_some()
        {
            true
        } else { loc_to_elf
            .get(&(self.location.0.to_string() + "," + &(self.location.1 - 1).to_string())).is_some() }
    }
    fn check(&self, loc_to_elf: &HashMap<String, Elf>, dir: Dir) -> bool {
        match dir {
            Dir::North => {
                if loc_to_elf.get(
                    &((self.location.0 - 1).to_string() + "," + &(self.location.1 - 1).to_string()),
                ).is_some() {
                    true
                } else if loc_to_elf
                    .get(&((self.location.0 - 1).to_string() + "," + &self.location.1.to_string())).is_some()
                {
                    true
                } else { loc_to_elf.get(
                    &((self.location.0 - 1).to_string() + "," + &(self.location.1 + 1).to_string()),
                ).is_some() }
            }
            Dir::South => {
                if loc_to_elf.get(
                    &((self.location.0 + 1).to_string() + "," + &(self.location.1 + 1).to_string()),
                ).is_some() {
                    true
                } else if loc_to_elf
                    .get(&((self.location.0 + 1).to_string() + "," + &self.location.1.to_string())).is_some()
                {
                    true
                } else { loc_to_elf.get(
                    &((self.location.0 + 1).to_string() + "," + &(self.location.1 - 1).to_string()),
                ).is_some() }
            }
            Dir::West => {
                if loc_to_elf.get(
                    &((self.location.0 + 1).to_string() + "," + &(self.location.1 - 1).to_string()),
                ).is_some() {
                    true
                } else if loc_to_elf
                    .get(&(self.location.0.to_string() + "," + &(self.location.1 - 1).to_string())).is_some()
                {
                    true
                } else { loc_to_elf.get(
                    &((self.location.0 - 1).to_string() + "," + &(self.location.1 - 1).to_string()),
                ).is_some() }
            }
            Dir::East => {
                if loc_to_elf.get(
                    &((self.location.0 - 1).to_string() + "," + &(self.location.1 + 1).to_string()),
                ).is_some() {
                    true
                } else if loc_to_elf
                    .get(&(self.location.0.to_string() + "," + &(self.location.1 + 1).to_string())).is_some()
                {
                    true
                } else { loc_to_elf.get(
                    &((self.location.0 + 1).to_string() + "," + &(self.location.1 + 1).to_string()),
                ).is_some() }
            }
        }
    }
    fn to_string(&self) -> String {
        self.location.0.to_string() + "," + &self.location.1.to_string()
    }
}
pub fn day23() {
    let input_str = include_str!("input.txt");
    let lines = input_str.trim().lines();
    let mut elves = Vec::new();
    let mut loc_to_elf = HashMap::new();
    for (row, line) in lines.enumerate() {
        for (col, byte) in line.as_bytes().iter().enumerate() {
            if *byte == b'#' {
                let elf = Elf::new((row as i32, col as i32));
                loc_to_elf.insert(elf.to_string(), elf.clone());
                elves.push(elf);
            }
        }
    }
    let mut order = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    let mut destinations = HashMap::new();
    for rnd in 0..2000 {
        let mut isolated = elves.len();
        for elf in &mut elves {
            if elf.check_all(&loc_to_elf) {
                isolated -= 1;
                let order_cycle = order.iter();
                for d in order_cycle {
                    if !elf.check(&loc_to_elf, d.clone()) {
                        elf.d = Some(d.clone());
                        match d {
                            Dir::North => {
                                let dest_str = (elf.location.0 - 1).to_string()
                                    + ","
                                    + &elf.location.1.to_string();
                                if destinations.contains_key(&dest_str) {
                                    let times = destinations.get(&dest_str).unwrap();
                                    destinations.insert(dest_str, *times + 1);
                                } else {
                                    destinations.insert(dest_str, 1);
                                }
                            }
                            Dir::South => {
                                let dest_str = (elf.location.0 + 1).to_string()
                                    + ","
                                    + &elf.location.1.to_string();
                                if destinations.contains_key(&dest_str) {
                                    let times = destinations.get(&dest_str).unwrap();
                                    destinations.insert(dest_str, *times + 1);
                                } else {
                                    destinations.insert(dest_str, 1);
                                }
                            }
                            Dir::West => {
                                let dest_str = elf.location.0.to_string()
                                    + ","
                                    + &(elf.location.1 - 1).to_string();

                                if destinations.contains_key(&dest_str) {
                                    let times = destinations.get(&dest_str).unwrap();
                                    destinations.insert(dest_str, *times + 1);
                                } else {
                                    destinations.insert(dest_str, 1);
                                }
                            }
                            Dir::East => {
                                let dest_str = elf.location.0.to_string()
                                    + ","
                                    + &(elf.location.1 + 1).to_string();

                                if destinations.contains_key(&dest_str) {
                                    let times = destinations.get(&dest_str).unwrap();
                                    destinations.insert(dest_str, *times + 1);
                                } else {
                                    destinations.insert(dest_str, 1);
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        if isolated == elves.len() {
            println!("part2: {}", rnd + 1);
            break;
        }
        if rnd == 10 {
            let mut small_row = i32::MAX;
            let mut big_row = i32::MIN;
            let mut small_col = i32::MAX;
            let mut big_col = i32::MIN;
            for elf in &elves {
                small_row = small_row.min(elf.location.0);
                big_row = big_row.max(elf.location.0);
                small_col = small_col.min(elf.location.1);
                big_col = big_col.max(elf.location.1);
            }
            println!(
                "part1: {}",
                (1 + big_row - small_row) * (1 + big_col - small_col) - elves.len() as i32
            );
        }
        for elf in &mut elves {
            elf.update(&destinations);
        }
        loc_to_elf.clear();
        for elf in &elves {
            loc_to_elf.insert(elf.to_string(), elf.clone());
        }
        let first = order.remove(0);
        order.push(first);
        destinations.clear();
    }
}
