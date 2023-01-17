use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Valve {
    name: &'static str,
    rate: usize,
    neighbors: Vec<&'static str>,
}
impl Valve {
    fn new(name: &'static str, rate: usize, neighbors: Vec<&'static str>) -> Valve {
        Valve {
            name,
            rate,
            neighbors,
        }
    }
}
fn find_target_valve(
    valve_hm: &HashMap<&'static str, Valve>,
    curr: &'static str,
    target: &'static str,
    visited: HashSet<&'static str>,
    min: usize,
) -> usize {
    if curr == target {
        return min;
    }
    let mut ans = usize::MAX;
    for n in &valve_hm.get(curr).unwrap().neighbors {
        if !visited.contains(n) {
            let mut new_visited = visited.clone();
            new_visited.insert(n);
            ans = ans.min(find_target_valve(valve_hm, n, target, new_visited, min + 1));
        }
    }
    ans
}
fn brute_force_every(
    hm: &HashMap<&'static str, Valve>,
    valves_vec: &Vec<Valve>,
    curr_min: usize,
    curr_v: &'static str,
    mut openness: HashMap<&'static str, bool>,
    minutes_to_target_hm: &HashMap<String, usize>,
    pressure_total: usize,
) -> usize {
    let mut ans = pressure_total;
    for v in 0..valves_vec.len() {
        if valves_vec[v].name != curr_v
            && !openness.get(valves_vec[v].name).unwrap()
            && valves_vec[v].rate > 0
        {
            let minutes_to_target = minutes_to_target_hm
                .get(&(curr_v.to_string() + valves_vec[v].name))
                .unwrap();
            if *minutes_to_target < usize::MAX
                && curr_min + minutes_to_target + 1 < 30
                && valves_vec[v].rate > 0
            {
                let pressure = (30 - (curr_min + minutes_to_target + 1)) * valves_vec[v].rate;
                openness.insert(valves_vec[v].name, true);
                ans = ans.max(brute_force_every(
                    hm,
                    valves_vec,
                    curr_min + minutes_to_target + 1,
                    valves_vec[v].name,
                    openness.clone(),
                    minutes_to_target_hm,
                    pressure_total + pressure,
                ));
                openness.insert(valves_vec[v].name, false);
            }
        }
    }
    ans
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
        Valve::new(name, rate, neighbors)
    });
    let mut valves_vec = Vec::new();
    let mut openness = HashMap::new();
    while let Some(v) = valves.next() {
        let name = v.name;
        hm.insert(name, v.clone());
        valves_vec.push(v);
        openness.insert(name, false);
    }
    openness.insert("AA", true);
    let mut short_dist = HashMap::new();
    for v in 0..valves_vec.len() {
        for v2 in 0..valves_vec.len() {
            if v != v2 {
                short_dist.insert(
                    valves_vec[v].name.to_string() + valves_vec[v2].name,
                    find_target_valve(
                        &hm,
                        valves_vec[v].name,
                        valves_vec[v2].name,
                        HashSet::new(),
                        0,
                    ),
                );
            }
        }
    }
    let part1 = brute_force_every(&hm, &valves_vec, 0, "AA", openness, &short_dist, 0);
    println!("{part1}");
}
// FIRST THOUGHT
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
//
// SECOND THOUGHT
// Possible brute force:
// generate all possible 30 valve movement permutations
// and use that to calculate the max amount of pressure possible released in 30 min
// permutations take waaaaay too long
//
// THIRD THOUGHT
// another solution is to really just DFS until you run out of time
// too long 3 ^ 30 roughly
//
//
//
// factors:
// minimize time takes to reach valve
// open valves with most pressure as early as possible
//
//
// FINAL THOUGHTS
// rank each valve based on potential pressure given with the formula: (30 - (S + 1)) * R
// where S is how many steps/minutes it took to get there and R is the pressure rate
// then we go through each valve from highest potential pressure given...
//
// question is what would be S? S could be different depending on what path  you choose to take
// make S to be the shortest possible amount of steps...
//
//
// for every valve, check how many minutes it takes to get to and open every other valve (assuming
// we take the shortest route every time)
//
//
// greedy doesn't work. we need to try every other valve
