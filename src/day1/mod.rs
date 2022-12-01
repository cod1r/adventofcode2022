use std::fs;
pub fn day1() {
    let input_str = fs::read_to_string("src/day1/input.txt");
    let spl = input_str.unwrap();
    let mut stuff: Vec<u64> = spl
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>())
        .map(|vs| {
            vs.into_iter()
                .map(|sn| sn.parse::<u64>())
                .collect::<Vec<_>>()
        })
        .map(|vn| {
            vn.into_iter().fold(0, |a, b| match b {
                Ok(val) => a + val,
                Err(_) => a + 0,
            })
        })
        .collect();
    stuff.sort();
    println!("1st: {}", stuff[stuff.len() - 1]);
    println!(
        "2nd: {}",
        stuff[stuff.len() - 1] + stuff[stuff.len() - 2] + stuff[stuff.len() - 3]
    );
}
