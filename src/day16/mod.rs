use std::collections::HashMap;

struct Valve {
    name: &'static str,
    rate: usize,
    neighbors: Vec<&'static str>,
    status: bool,
}
impl Valve {
    fn new(name: &'static str, rate: usize, neighbors: Vec<&'static str>, status: bool) -> Valve {
        Valve {
            name,
            rate,
            neighbors,
            status,
        }
    }
}
pub fn day16() {
    let input_str = include_str!("input.txt");
    let mut hm = HashMap::new();
    let mut valves = input_str.trim().lines().map(|line| {
        let mut parts = line.split(' ');
        let name = parts.nth(1).unwrap();
        let rate_str = parts.nth(2).unwrap();
        let rate = rate_str[5..rate_str.len() - 1].parse::<usize>().unwrap();
        let mut neighbors = Vec::new();
        parts.nth(3).unwrap();
        while let Some(n) = parts.next() {
            if n.contains(',') {
                neighbors.push(&n[..n.len() - 1]);
            } else {
                neighbors.push(n);
            }
        }
        Valve::new(name, rate, neighbors, false)
    });
    while let Some(v) = valves.next() {
        hm.insert(v.name, v);
    }
}
// opportunity cost
// cost of opening a valve takes 1 min which means you push every valve in the future back 1 min
// rate: 12,open 12,13,open 13,80
// if you open 12 and 13, you gain 24 and 26 but you lose 80 * 2
// this is for the same path
// pretty much (sum of valve rates) * (number of valves before)
//  compared with any valve rate in future * (number of valves before)
//
//  basic observation is if something is 2 times the previous valve rate, then you can
//  just skip the previous valve rate.
//
// what influences what path we take?
//  the opportunity cost for each path and the pressure gains.
//
// why wouldn't a breadth first search work?
//  instead of the smallest rate, we take the biggest rate each time
//  problem is that there is a time factor (30 mins)
//  breadth first search by itself won't factor in the opportunity cost
//  which means if we factor it in, it'll pretty much just be like DFS
//  BFS and DFS work but DFS is better since...
// we also have to consider cycles in the graph
