pub fn day6() {
    let bytes = include_bytes!("input.txt");
    let mut ans: usize = 0;
    for i in 0..bytes.len() {
        let mut stop = false;
        for j in i..i + 4 {
            for k in j + 1..i + 4 {
                if bytes[j] == bytes[k] {
                    stop = true;
                    break;
                }
            }
            if stop {
                break;
            }
        }
        if !stop {
            ans = i + 4;
            break;
        }
    }
    let mut ans2: usize = 0;
    for i in 0..bytes.len() {
        let mut stop = false;
        for j in i..i + 14 {
            for k in j + 1..i + 14 {
                if bytes[j] == bytes[k] {
                    stop = true;
                    break;
                }
            }
            if stop {
                break;
            }
        }
        if !stop {
            ans2 = i + 14;
            break;
        }
    }
    println!("day6");
    println!("{ans}");
    println!("{ans2}");
}
