struct BluePrint {
    id: u32,
    ore_cost: u32,
    clay_cost: u32,
    obs_cost: (u32, u32),
    geode_cost: (u32, u32),
}
impl BluePrint {
    fn new(
        id: u32,
        ore_cost: u32,
        clay_cost: u32,
        obs_cost: (u32, u32),
        geode_cost: (u32, u32),
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
    ore: (u32, u32),
    clay: (u32, u32),
    obsid: (u32, u32),
    geode: (u32, u32),
}
impl State {
    fn new(ore: (u32, u32), clay: (u32, u32), obsid: (u32, u32), geode: (u32, u32)) -> State {
        State {
            ore,
            clay,
            obsid,
            geode,
        }
    }
}
fn dfs<const E: u32>(bp: &BluePrint, state: State, minute: u32) -> u32 {
    if minute >= E {
        return state.geode.0;
    }
    let mut ans = 0;
    ans = ans.max(dfs::<E>(
        bp,
        State::new(
            (state.ore.0 + state.ore.1, state.ore.1),
            (state.clay.0 + state.clay.1, state.clay.1),
            (state.obsid.0 + state.obsid.1, state.obsid.1),
            (state.geode.0 + state.geode.1, state.geode.1),
        ),
        minute + 1,
    ));
    {
        let ore_cost_left = 0.max(bp.ore_cost as i32 - state.ore.0 as i32) as u32;
        let mut minutes_to_add_ore = ore_cost_left / state.ore.1;
        if minutes_to_add_ore * state.ore.1 < ore_cost_left {
            minutes_to_add_ore += 1;
        }
        if state.ore.1 < 6 {
            ans = ans.max(dfs::<E>(
                bp,
                State::new(
                    (
                        state.ore.0 + state.ore.1 * minutes_to_add_ore - bp.ore_cost,
                        state.ore.1 + 1,
                    ),
                    (
                        state.clay.0 + state.clay.1 * minutes_to_add_ore,
                        state.clay.1,
                    ),
                    (
                        state.obsid.0 + state.obsid.1 * minutes_to_add_ore,
                        state.obsid.1,
                    ),
                    (
                        state.geode.0 + state.geode.1 * minutes_to_add_ore,
                        state.geode.1,
                    ),
                ),
                minute + minutes_to_add_ore,
            ));
        }
    }
    {
        let clay_cost_left = 0.max(bp.clay_cost as i32 - state.ore.0 as i32) as u32;
        let mut minutes_to_add_clay = clay_cost_left / state.ore.1;
        if minutes_to_add_clay * state.ore.1 < clay_cost_left {
            minutes_to_add_clay += 1;
        }
        if state.clay.1 < 10 {
            ans = ans.max(dfs::<E>(
                bp,
                State::new(
                    (
                        state.ore.0 + state.ore.1 * minutes_to_add_clay - bp.clay_cost,
                        state.ore.1,
                    ),
                    (
                        state.clay.0 + state.clay.1 * minutes_to_add_clay,
                        state.clay.1 + 1,
                    ),
                    (
                        state.obsid.0 + state.obsid.1 * minutes_to_add_clay,
                        state.obsid.1,
                    ),
                    (
                        state.geode.0 + state.geode.1 * minutes_to_add_clay,
                        state.geode.1,
                    ),
                ),
                minute + minutes_to_add_clay,
            ));
        }
    }
    if state.ore.0 >= bp.obs_cost.0 && state.clay.0 >= bp.obs_cost.1 && state.obsid.1 < 10 {
        ans = ans.max(dfs::<E>(
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
        ans = ans.max(dfs::<E>(
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
    ans
}
pub fn day19() {
    let input_str = include_str!("example.txt");
    let mut blueprints = input_str.trim().lines().map(|line| {
        let mut parts = line.split(' ');
        let num_colon = parts.nth(1).unwrap();
        let id = num_colon[..num_colon.len() - 1].parse::<u32>().unwrap();
        let ore_cost = parts.nth(4).unwrap().parse::<u32>().unwrap();
        let clay_cost = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let obsidian_cost1 = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let obsidian_cost2 = parts.nth(2).unwrap().parse::<u32>().unwrap();
        let geode_cost1 = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let geode_cost2 = parts.nth(2).unwrap().parse::<u32>().unwrap();
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
        let max_geodes = dfs::<24>(&bp, State::new((0, 1), (0, 0), (0, 0), (0, 0)), 0);
        ans += bp.id * max_geodes;
        println!("{}", max_geodes);
    }
    println!("{}", ans);
}
