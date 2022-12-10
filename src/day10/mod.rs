pub fn day10() {
    let input_str = include_str!("input.txt");
    let mut crt = [[0; 40]; 6];
    let part1 = input_str
        .trim()
        .lines()
        .fold((1 as i32, 0 as usize, 0 as usize), |acc, line| {
            let (mut x, mut cycle, mut sig_str) = acc;
            let mut parts = line.split(" ");
            match parts.nth(0).unwrap() {
                "addx" => {
                    cycle += 1;
                    if cycle == 20
                        || cycle == 60
                        || cycle == 100
                        || cycle == 140
                        || cycle == 180
                        || cycle == 220
                    {
                        sig_str += x as usize * cycle;
                    }
                    let pos = (cycle - 1) as i32 % 40;
                    if pos == x - 1 || pos == x || pos == x + 1 {
                        crt[(cycle - 1) / 40][(cycle - 1) % 40] = 1;
                    } else {
                        crt[(cycle - 1) / 40][(cycle - 1) % 40] = 0;
                    }
                    cycle += 1;
                    if cycle == 20
                        || cycle == 60
                        || cycle == 100
                        || cycle == 140
                        || cycle == 180
                        || cycle == 220
                    {
                        sig_str += x as usize * cycle;
                    }
                    let pos = (cycle - 1) as i32 % 40;
                    if pos == x - 1 || pos == x || pos == x + 1 {
                        crt[(cycle - 1) / 40][(cycle - 1) % 40] = 1;
                    } else {
                        crt[(cycle - 1) / 40][(cycle - 1) % 40] = 0;
                    }
                    x += parts.nth(0).unwrap().parse::<i32>().unwrap();
                }
                "noop" => {
                    cycle += 1;
                    if cycle == 20
                        || cycle == 60
                        || cycle == 100
                        || cycle == 140
                        || cycle == 180
                        || cycle == 220
                    {
                        sig_str += x as usize * cycle;
                    }
                    let pos = (cycle - 1) as i32 % 40;
                    if pos == x - 1 || pos == x || pos == x + 1 {
                        crt[(cycle - 1) / 40][(cycle - 1) % 40] = 1;
                    } else {
                        crt[(cycle - 1) / 40][(cycle - 1) % 40] = 0;
                    }
                }
                _ => unreachable!(),
            }
            (x, cycle, sig_str)
        });
    println!("day10: {}", part1.2);
    crt.iter().for_each(|row| {
        for i in 0..40 {
            print!("{}", if row[i] == 0 { '.' } else { '#' });
        }
        println!();
    });
}
