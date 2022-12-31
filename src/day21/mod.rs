use std::collections::HashMap;

fn dfs(
    name: &'static str,
    hm: &HashMap<&'static str, &'static str>,
    ans: &mut HashMap<&'static str, usize>,
) -> usize {
    if hm.get(name).unwrap().split(' ').count() == 1 {
        let val = hm.get(name).unwrap().parse::<usize>().unwrap();
        ans.insert(name, val);
        return val;
    }
    if ans.contains_key(name) {
        return *ans.get(name).unwrap();
    }
    let mut two = hm.get(name).unwrap().split(' ');
    let first = dfs(two.next().unwrap(), hm, ans);
    let op = two.next().unwrap();
    let second = dfs(two.next().unwrap(), hm, ans);
    match op {
        "+" => {
            ans.insert(name, first + second);
            first + second
        }
        "-" => {
            ans.insert(name, first - second);
            first - second
        }
        "*" => {
            ans.insert(name, first * second);
            first * second
        }
        "/" => {
            ans.insert(name, first / second);
            first / second
        }
        _ => unreachable!(),
    }
}

fn find_humn(
    name: &'static str,
    hm: &HashMap<&'static str, &'static str>,
    track: Vec<(&'static str, &'static str)>,
) -> Vec<(&'static str, &'static str)> {
    if hm.get(name).unwrap().split(' ').count() == 1 {
        return Vec::new();
    }
    let mut two = hm.get(name).unwrap().split(' ');
    let left = two.next().unwrap();
    let op = two.next().unwrap();
    let right = two.next().unwrap();
    let mut op_first = track.clone();
    op_first.push((left, op));
    let mut op_second = track.clone();
    op_second.push((op, right));
    if left == "humn" {
        return op_second;
    } else if right == "humn" {
        return op_first;
    }
    let first = find_humn(left, hm, op_second);
    let second = find_humn(right, hm, op_first);
    if first.len() > 0 {
        first
    } else {
        second
    }
}

pub fn day21() {
    let input_str = include_str!("input.txt");
    let mut hm = HashMap::new();
    let monkeys = input_str.trim().lines().map(|line| {
        let mut parts = line.split(':');
        (parts.next().unwrap(), parts.next().unwrap().trim())
    });
    for mk in monkeys.clone() {
        hm.insert(mk.0, mk.1);
    }
    let mut ans = HashMap::new();
    println!("part1: {}", dfs("root", &hm, &mut ans));
    let mut root_parts = hm.get("root").unwrap().split(' ');
    let left = root_parts.next().unwrap();
    let right = root_parts.nth(1).unwrap();
    let mut first_ans = HashMap::new();
    let mut second_ans = HashMap::new();
    let mut first = dfs(left, &hm, &mut first_ans);
    let mut second = dfs(right, &hm, &mut second_ans);
    if first_ans.contains_key("humn") {
        let ops = find_humn(left, &hm, Vec::new());
        assert!(ops.len() > 0);
        for op in ops {
            match op.0 {
                "+" => {
                    second -= ans.get(op.1).unwrap();
                }
                "-" => {
                    second += ans.get(op.1).unwrap();
                }
                "*" => {
                    second /= ans.get(op.1).unwrap();
                }
                "/" => {
                    second *= ans.get(op.1).unwrap();
                }
                _ => match op.1 {
                    "+" => {
                        second -= ans.get(op.0).unwrap();
                    }
                    "-" => {
                        second = ans.get(op.0).unwrap() - second;
                    }
                    "*" => {
                        second /= ans.get(op.0).unwrap();
                    }
                    "/" => {
                        second = ans.get(op.0).unwrap() / second;
                    }
                    _ => unreachable!(),
                },
            }
        }
        println!("part2: {second}");
    } else if second_ans.contains_key("humn") {
        let ops = find_humn(right, &hm, Vec::new());
        assert!(ops.len() > 0);
        for op in ops {
            match op.0 {
                "+" => {
                    first -= ans.get(op.1).unwrap();
                }
                "-" => {
                    first += ans.get(op.1).unwrap();
                }
                "*" => {
                    first /= ans.get(op.1).unwrap();
                }
                "/" => {
                    first *= ans.get(op.1).unwrap();
                }
                _ => match op.1 {
                    "+" => {
                        first -= ans.get(op.0).unwrap();
                    }
                    "-" => {
                        first = ans.get(op.0).unwrap() - first;
                    }
                    "*" => {
                        first /= ans.get(op.0).unwrap();
                    }
                    "/" => {
                        first = ans.get(op.0).unwrap() / first;
                    }
                    _ => unreachable!(),
                },
            }
        }
        println!("part2: {first}");
    }
}
