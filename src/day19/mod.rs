struct BluePrint {
    id: usize,
    ore_cost: usize,
    clay_cost: usize,
    obs_cost: (usize, usize),
    geode_cost: (usize, usize),
}
impl BluePrint {
    fn new(
        id: usize,
        ore_cost: usize,
        clay_cost: usize,
        obs_cost: (usize, usize),
        geode_cost: (usize, usize),
    ) -> BluePrint {
        BluePrint {
            id,
            ore_cost,
            clay_cost,
            obs_cost,
            geode_cost,
        }
    }
}
struct State {
    ore: (usize, usize),
    clay: (usize, usize),
    obsid: (usize, usize),
    geode: (usize, usize),
}
impl State {
    fn new(
        ore: (usize, usize),
        clay: (usize, usize),
        obsid: (usize, usize),
        geode: (usize, usize),
    ) -> State {
        State {
            ore,
            clay,
            obsid,
            geode,
        }
    }
}
fn dfs(bp: &BluePrint, state: State, minute: usize) -> usize {
    if minute == 24 {
        return state.geode.0;
    }
    let mut ans = 0;
    if state.ore.0 >= bp.ore_cost {
        ans = ans.max(dfs(
            bp,
            State::new(
                (state.ore.0 + state.ore.1 - bp.ore_cost, state.ore.1 + 1),
                (state.clay.0 + state.clay.1, state.clay.1),
                (state.obsid.0 + state.obsid.1, state.obsid.1),
                (state.geode.0 + state.geode.1, state.geode.1),
            ),
            minute + 1,
        ));
    }
    if state.ore.0 >= bp.clay_cost {
        ans = ans.max(dfs(
            bp,
            State::new(
                (state.ore.0 + state.ore.1 - bp.clay_cost, state.ore.1),
                (state.clay.0 + state.clay.1, state.clay.1 + 1),
                (state.obsid.0 + state.obsid.1, state.obsid.1),
                (state.geode.0 + state.geode.1, state.geode.1),
            ),
            minute + 1,
        ));
    }
    let mut bought = false;
    if state.ore.0 >= bp.obs_cost.0 && state.clay.0 >= bp.obs_cost.1 {
        bought = true;
        ans = ans.max(dfs(
            bp,
            State::new(
                (state.ore.0 + state.ore.1 - bp.obs_cost.0, state.ore.1),
                (state.clay.0 + state.clay.1 - bp.obs_cost.1, state.clay.1),
                (state.obsid.0 + state.obsid.1, state.obsid.1 + 1),
                (state.geode.0 + state.geode.1, state.geode.1),
            ),
            minute + 1,
        ));
    }
    if state.ore.0 >= bp.geode_cost.0 && state.obsid.0 >= bp.geode_cost.1 {
        bought = true;
        ans = ans.max(dfs(
            bp,
            State::new(
                (state.ore.0 + state.ore.1 - bp.geode_cost.0, state.ore.1),
                (state.clay.0 + state.clay.1, state.clay.1),
                (
                    state.obsid.0 + state.obsid.1 - bp.geode_cost.1,
                    state.obsid.1,
                ),
                (state.geode.0 + state.geode.1, state.geode.1 + 1),
            ),
            minute + 1,
        ));
    }
    if !bought {
        ans = ans.max(dfs(
            bp,
            State::new(
                (state.ore.0 + state.ore.1, state.ore.1),
                (state.clay.0 + state.clay.1, state.clay.1),
                (state.obsid.0 + state.obsid.1, state.obsid.1),
                (state.geode.0 + state.geode.1, state.geode.1),
            ),
            minute + 1,
        ));
    }
    ans
}
pub fn day19() {
    let input_str = include_str!("input.txt");
    let mut blueprints = input_str.trim().lines().map(|line| {
        let mut parts = line.split(' ');
        let num_colon = parts.nth(1).unwrap();
        let id = num_colon[..num_colon.len() - 1].parse::<usize>().unwrap();
        let ore_cost = parts.nth(4).unwrap().parse::<usize>().unwrap();
        let clay_cost = parts.nth(5).unwrap().parse::<usize>().unwrap();
        let obsidian_cost1 = parts.nth(5).unwrap().parse::<usize>().unwrap();
        let obsidian_cost2 = parts.nth(2).unwrap().parse::<usize>().unwrap();
        let geode_cost1 = parts.nth(5).unwrap().parse::<usize>().unwrap();
        let geode_cost2 = parts.nth(2).unwrap().parse::<usize>().unwrap();
        BluePrint::new(
            id,
            ore_cost,
            clay_cost,
            (obsidian_cost1, obsidian_cost2),
            (geode_cost1, geode_cost2),
        )
    });
    let mut ans = 0;
    while let Some(bp) = blueprints.next() {
        let max_geodes = dfs(&bp, State::new((0, 1), (0, 0), (0, 0), (0, 0)), 0);
        ans += bp.id * max_geodes;
        println!("{}", max_geodes);
    }
    println!("{}", ans);
}
