use std::collections::HashMap;
pub fn day7() {
    let input_str = include_str!("input.txt");
    let mut hash = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let lines = input_str.trim().lines();
    let mut times: usize = 0;
    for line in lines {
        let mut parts = line.split(" ");
        let first_part = parts.next().unwrap();
        if first_part == "$" {
            let second_part = parts.next().unwrap();
            if second_part == "cd" {
                let dir_name = parts.next().unwrap();
                if dir_name != ".." {
                    let check = String::from(dir_name.clone()) + &stack.len().to_string();
                    times += 1;
                    if hash.contains_key(&check) {
                        todo!()
                    }
                    hash.insert(
                        String::from(dir_name.clone())
                            + &stack.len().to_string()
                            + &times.to_string(),
                        0,
                    );
                    stack.push(
                        String::from(dir_name.clone())
                            + &stack.len().to_string()
                            + &times.to_string(),
                    );
                } else if dir_name == ".." {
                    stack.pop();
                }
            }
        } else if first_part.as_bytes()[0] >= '0' as u8 && first_part.as_bytes()[0] <= '9' as u8 {
            let add = first_part.parse::<usize>().unwrap();
            for key in &stack {
                let value = hash.get(key).unwrap();
                hash.insert(key.to_string(), value + add);
            }
        }
    }
    let mut sum = 0;
    let mut qualify = Vec::new();
    let total = hash.get("/01").unwrap();
    for key in hash.keys() {
        let value = hash.get(key).unwrap();
        if *value <= 100000 {
            sum += *value;
        }
        if total - value <= 40000000 {
            qualify.push(value);
        }
    }
    qualify.sort();
    let second = qualify.first().unwrap();
    println!("day7");
    println!("{sum}");
    println!("{second}");
}
