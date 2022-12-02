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
    for i in 1..4 {
        for j in 0..stuff.len() - i {
            if stuff[j] > stuff[j + 1] {
                stuff.swap(j, j + 1);
            }
        }
    }
    println!("day1");
    println!("{}", stuff[stuff.len() - 1]);
    println!("{}", stuff[stuff.len() - 3..].iter().sum::<u64>());
}
