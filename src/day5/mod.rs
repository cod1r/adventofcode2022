pub fn day5() {
    let bytes = include_bytes!("input.txt");
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut end: usize = 0;
    let mut stack: usize = 0;
    for idx in (0..bytes.len()).step_by(4) {
        let one = bytes[idx];
        let two = bytes[idx + 1];
        if two == b'1' {
            end = idx;
            while bytes[end] != b'\n' {
                end += 1;
            }
            end += 2;
            break;
        }
        let three = bytes[idx + 2];
        if one == b'[' && (b'A'..=b'Z').contains(&two) && three == b']' {
            stacks[stack].push(two as char);
        }
        stack = (stack + 1) % 9;
    }
    for s in &mut stacks {
        s.reverse();
    }
    let mut stacks2 = Vec::new();
    for s in &stacks {
        stacks2.push(s.clone());
    }
    let string_lossy = String::from_utf8_lossy(&bytes[end..]);
    assert!(string_lossy.as_bytes()[0] != b'\n');
    let string = string_lossy.trim().split('\n');
    for part in string {
        let mut parts = part.split(' ');
        let one = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let two = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let three = parts.nth(1).unwrap().parse::<usize>().unwrap();
        for _ in 0..one {
            let t = stacks[two - 1].pop().unwrap();
            stacks[three - 1].push(t);
        }
        for count in 0..one {
            let t = stacks2[two - 1][stacks2[two - 1].len() - (one - count)];
            stacks2[three - 1].push(t);
        }
        for _ in 0..one {
            stacks2[two - 1].pop();
        }
    }
    println!("day5");
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!();
    for s in stacks2 {
        print!("{}", s.last().unwrap());
    }
    println!();
}
